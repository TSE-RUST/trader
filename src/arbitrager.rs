use bfb::bfb_market::Bfb as bfb;
use market_sol::SOLMarket as sol;
use parse_market::ParseMarket as parse;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use unitn_market_2022::good::good::Good;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::{LockSellError, Market, MarketGetterError};

pub struct Arbitrager {
    sol: Rc<RefCell<dyn Market>>,
    bfb: Rc<RefCell<dyn Market>>,
    parse: Rc<RefCell<dyn Market>>,
    trader_name: String,
}

pub struct ArbitrageResult {
    eur_sent: f32,
    alt_received: f32,
    eur_received: f32,
    buy_market_name: String,
    sell_market_name: String,
}

impl Arbitrager {
    ///the constructor for the Arbitrager
    ///
    /// **Lorenzo Tinfena**
    pub fn new(
        trader_name: String,
        sol: &Rc<RefCell<dyn Market>>,
        bfb: &Rc<RefCell<dyn Market>>,
        parse: &Rc<RefCell<dyn Market>>,
    ) -> Self {
        Arbitrager {
            trader_name,
            sol: Rc::clone(sol),
            bfb: Rc::clone(bfb),
            parse: Rc::clone(parse),
        }
    }

    fn get_best_quantity_buy(good_kind: GoodKind, max_quantity: f32) {}

    pub fn arbitrage(&self, mut eur: Good) -> (Good, Option<ArbitrageResult>) {
        if eur.get_qty() <= 0. {
            return (eur, None);
        }

        let good_kinds = vec![GoodKind::USD, GoodKind::YEN, GoodKind::YUAN];

        let markets = vec![self.sol.clone(), self.bfb.clone(), self.parse.clone()];

        const F32SMALL: f32 = 0.001;
        const F32LARGE: f32 = 1000000000000.;

        let mut best_buy_market = sol::new_random();
        let mut best_sell_market = sol::new_random();
        let mut best_alt_to_receive: f32 = 0.0;
        let mut best_eur_to_send: f32 = 0.0;
        let mut best_kind_alt_to_receive: GoodKind = GoodKind::EUR;
        let mut best_profit = 0.;
        // Try to arbitrage over all currencies
        for goodKind in good_kinds {
            for _sell_market in &markets {
                for _buy_market in &markets {
                    let buy_market = (**_buy_market).borrow();
                    let sell_market = (**_sell_market).borrow();

                    if buy_market.get_name() == sell_market.get_name() {
                        continue;
                    }

                    // Trying to get the max good (aka goodKind or altcoin or alt) quantity to buy, such as the eur to send is less or equal than the ones I have
                    let mut max_alt_to_receive =
                        match buy_market.get_buy_price(goodKind, F32LARGE).unwrap_err() {
                            MarketGetterError::NonPositiveQuantityAsked => unimplemented!(),
                            MarketGetterError::InsufficientGoodQuantityAvailable {
                                requested_good_kind: _requested_good_kind,
                                requested_good_quantity: _requested_good_quantity,
                                available_good_quantity,
                            } => available_good_quantity,
                        };

                    if max_alt_to_receive <= F32SMALL {
                        continue;
                    }

                    let mut max_eur_to_send = buy_market
                        .get_buy_price(goodKind, max_alt_to_receive)
                        .unwrap();
                    while max_eur_to_send > eur.get_qty() {
                        max_alt_to_receive /= 2.;
                        max_eur_to_send = buy_market
                            .get_buy_price(goodKind, max_alt_to_receive)
                            .unwrap();
                    }

                    if max_alt_to_receive <= F32SMALL {
                        continue;
                    }

                    // bounds for prices

                    let buy_min_price =
                        buy_market.get_buy_price(goodKind, F32SMALL).unwrap() / F32SMALL;
                    let sell_max_price =
                        sell_market.get_sell_price(goodKind, F32SMALL).unwrap() / F32SMALL;

                    let buy_max_price = (max_eur_to_send
                        - (buy_market
                            .get_buy_price(goodKind, max_alt_to_receive - F32SMALL)
                            .unwrap()))
                        / F32SMALL;
                    let sell_min_price = (sell_market
                        .get_sell_price(goodKind, max_alt_to_receive)
                        .unwrap()
                        - sell_market
                            .get_sell_price(goodKind, max_alt_to_receive - F32SMALL)
                            .unwrap())
                        / F32SMALL;

                    // Skip if prices are not growing with the quantity or can't be profit
                    if buy_max_price < buy_min_price
                        || sell_max_price < sell_min_price
                        || buy_min_price >= sell_max_price
                    {
                        continue;
                    }

                    // Compute profit
                    let (profit, x) = if buy_max_price <= sell_min_price {
                        // First case
                        (
                            sell_market
                                .get_sell_price(goodKind, max_alt_to_receive)
                                .unwrap()
                                - max_eur_to_send,
                            1.,
                        )
                    } else {
                        let x = ((sell_min_price - buy_min_price)
                            / (buy_max_price - buy_min_price - sell_max_price + sell_min_price));
                        // Second case
                        (
                            sell_market
                                .get_sell_price(goodKind, x * max_alt_to_receive)
                                .unwrap()
                                - max_eur_to_send,
                            x,
                        )
                    };

                    if profit > best_profit {
                        best_profit = profit;
                        best_alt_to_receive = x * max_alt_to_receive;
                        best_kind_alt_to_receive = goodKind;
                        best_eur_to_send = x * max_eur_to_send;
                        best_buy_market = (*_buy_market).clone();
                        best_sell_market = (*_sell_market).clone();
                    }
                }
            }
        }

        if best_profit <= 0. {
            return (eur, None);
        }

        let mut alt_coin = Good::new(GoodKind::EUR, 100.);

        {
            let mut best_buy_market_deref = (*best_buy_market).borrow_mut();

            // Do actual trade6
            let buy_token = match best_buy_market_deref.lock_buy(
                best_kind_alt_to_receive,
                best_alt_to_receive * 0.999,
                best_eur_to_send,
                self.trader_name.clone(),
            ) {
                Ok(token) => token,
                Err(_) => unimplemented!(),
            };
            // TODO handle best_max_eur_to_send
            //==
            alt_coin = best_buy_market_deref
                .buy(buy_token, &mut eur.split(best_eur_to_send).unwrap())
                .unwrap();
        }

        let alt_coin_quantity_received = alt_coin.get_qty();
        let mut eur_received = Good::new(GoodKind::EUR, 100.);
        {
            let mut best_sell_market_deref = (*best_sell_market).borrow_mut();

            // Compute actual offer
            let offer = best_sell_market_deref
                .get_sell_price(best_kind_alt_to_receive, alt_coin.get_qty())
                .unwrap();

            // Do actual trade
            let sell_token = match best_sell_market_deref.lock_sell(
                best_kind_alt_to_receive,
                alt_coin.get_qty(),
                offer,
                self.trader_name.clone(),
            ) {
                Ok(token) => token,
                Err(_) => unimplemented!(),
            };

            eur_received = best_sell_market_deref
                .sell(sell_token, &mut alt_coin)
                .unwrap();
        }

        let eur_quantity_received = eur_received.get_qty();
        eur.merge(eur_received);

        return (
            eur,
            Some(ArbitrageResult {
                eur_sent: best_eur_to_send,
                alt_received: alt_coin_quantity_received,
                eur_received: eur_quantity_received,
                buy_market_name: (*best_buy_market).borrow().get_name().to_string(),
                sell_market_name: (*best_sell_market).borrow().get_name().to_string(),
            }),
        );
    }
}
