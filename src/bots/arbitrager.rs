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
    pub eur_sent: f32,
    pub alt_received: f32,
    pub eur_received: f32,
    pub buy_market_name: String,
    pub sell_market_name: String,
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

    // Pass the eur you want to invest, and returns:
    // 1. eur returned back (could be with 0 quantity)
    // 2. optional good as rest as the arbitrage
    // 3. if there was an arbitrage, returns an ArbitrageResult, otherwise None. If None implies that the second returned parameter is None
    pub fn arbitrage(&self, mut eur: Good) -> (Good, Option<Good>, Option<ArbitrageResult>) {
        if eur.get_qty() <= 0. {
            return (eur, None, None);
        }

        let good_kinds = vec![GoodKind::USD, GoodKind::YEN, GoodKind::YUAN];

        let markets = vec![self.sol.clone(), self.bfb.clone(), self.parse.clone()];

        const F32SMALL: f32 = 0.01;
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
                        if max_alt_to_receive <= F32SMALL {
                            continue;
                        }
                    }

                    // bounds for prices

                    let mut buy_min_price =
                        buy_market.get_buy_price(goodKind, F32SMALL).unwrap() / F32SMALL;
                    let sell_max_price =
                        sell_market.get_sell_price(goodKind, F32SMALL).unwrap() / F32SMALL;

                    let buy_max_price = (max_eur_to_send
                        - (buy_market
                            .get_buy_price(goodKind, max_alt_to_receive - F32SMALL)
                            .unwrap()))
                        / F32SMALL;
                    let mut sell_min_price = (sell_market
                        .get_sell_price(goodKind, max_alt_to_receive)
                        .unwrap()
                        - sell_market
                            .get_sell_price(goodKind, max_alt_to_receive - F32SMALL)
                            .unwrap())
                        / F32SMALL;

                    if buy_min_price > buy_max_price {
                        buy_min_price = buy_max_price;
                    }

                    if sell_min_price > sell_max_price {
                        sell_min_price = sell_max_price;
                    }

                    if buy_min_price >= sell_max_price {
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
                        let x = ((sell_max_price - buy_min_price)
                            / (buy_max_price - buy_min_price + sell_max_price - sell_min_price));
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
            return (eur, None, None);
        }

        let mut alt_coin = Good::new(GoodKind::EUR, 100.);

        {
            let mut best_buy_market_deref = (*best_buy_market).borrow_mut();

            // Do actual trade
            let buy_token = match best_buy_market_deref.lock_buy(
                best_kind_alt_to_receive,
                best_alt_to_receive,
                best_eur_to_send,
                self.trader_name.clone(),
            ) {
                Ok(token) => token,
                Err(_) => unimplemented!(),
            };

            alt_coin = best_buy_market_deref
                .buy(buy_token, &mut eur.split(best_eur_to_send).unwrap())
                .unwrap();
        }

        let alt_coin_quantity_received = alt_coin.get_qty();

        if alt_coin_quantity_received <= 0. {
            return (eur, None, None);
        }

        let mut rest = None;
        let mut eur_received = Good::new(GoodKind::EUR, 100.);
        {
            let mut best_sell_market_deref = (*best_sell_market).borrow_mut();

            let mut eur_budget = 0.;
            for good in best_sell_market_deref.get_goods() {
                if good.good_kind == GoodKind::EUR {
                    eur_budget = good.quantity;
                }
            }

            // Compute actual qty_to_send and offer
            let mut qty_to_send = alt_coin.get_qty();
            let mut offer = best_sell_market_deref
                .get_sell_price(best_kind_alt_to_receive, qty_to_send)
                .unwrap();
            while offer > eur_budget {
                qty_to_send /= 2.;
                offer = best_sell_market_deref
                    .get_sell_price(best_kind_alt_to_receive, qty_to_send)
                    .unwrap();
                if offer <= F32SMALL {
                    break;
                }
            }
            if offer <= F32SMALL {
                return (eur, Some(alt_coin), None);
            }

            let mut alt_coin_to_send: Good;
            if qty_to_send == alt_coin.get_qty() {
                alt_coin_to_send = alt_coin;
                rest = None;
            } else {
                alt_coin_to_send = alt_coin.split(qty_to_send).unwrap();
                rest = Some(alt_coin);
            };

            // Do actual trade
            let sell_token = match best_sell_market_deref.lock_sell(
                best_kind_alt_to_receive,
                alt_coin_to_send.get_qty(),
                offer,
                self.trader_name.clone(),
            ) {
                Ok(token) => token,
                Err(_) => unimplemented!(),
            };

            eur_received = best_sell_market_deref
                .sell(sell_token, &mut alt_coin_to_send)
                .unwrap();
        }

        let eur_quantity_received = eur_received.get_qty();
        eur.merge(eur_received);

        return (
            eur,
            rest,
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
