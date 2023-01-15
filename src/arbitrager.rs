use bfb::bfb_market::Bfb as bfb;
use market_sol::SOLMarket as sol;
use parse_market::ParseMarket as parse;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::rc::Rc;
use unitn_market_2022::good::good::Good;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::{Market, MarketGetterError};

pub struct Arbitrager {
    sol: Rc<RefCell<dyn Market>>,
    bfb: Rc<RefCell<dyn Market>>,
    parse: Rc<RefCell<dyn Market>>,
    traderName: String,
}

pub struct ArbitrageResult {
    eurSent: f32,
    altReceived: f32,
    eurReceived: f32,
    buyMarketName: String,
    sellMarketName: String,
}

impl Arbitrager {
    ///the constructor for the Arbitrager
    ///
    /// **Lorenzo Tinfena**
    pub fn new(
        traderName: String,
        sol: Rc<RefCell<dyn Market>>,
        bfb: Rc<RefCell<dyn Market>>,
        parse: Rc<RefCell<dyn Market>>,
    ) -> Self {
        Arbitrager {
            traderName,
            sol,
            bfb,
            parse,
        }
    }

    fn getBestQuantityBuy(goodKind: GoodKind, maxQuantity: f32) {}

    pub fn arbitrage(self: &Self, eur: Good) -> (Good, Option<ArbitrageResult>) {
        if eur.get_qty() <= 0. {
            return (eur, None);
        }

        let goodKinds = vec![GoodKind::USD, GoodKind::YEN, GoodKind::YUAN];

        let markets = vec![
            self.sol,
            self.bfb,
            self.parse,
        ];

        const F32SMALL: f32 = 0.0000001;
        const F32LARGE: f32 = 1000000000000.;

        let mut bestBuyMarket = self.sol.deref().borrow_mut();
        let mut bestSellMarket = self.sol.deref().borrow_mut();
        let mut bestMaxAltToReceive: f32;
        let mut bestMaxEurToSend: f32;
        let mut bestKindAltToReceive: GoodKind;
        let mut bestProfit = 0.;
        // Try to arbitrage over all currencies
        for goodKind in goodKinds {
            for _sellMarket in markets {
                for _buyMarket in markets {
                    // Get some bounds for prices (for price I mean goodKind/EUR)
                    
                    let buyMarket = _buyMarket.deref().borrow_mut();
                    let sellMarket = _sellMarket.deref().borrow_mut();

                    let buyMinPrice =
                        buyMarket.get_buy_price(goodKind, F32SMALL).unwrap() / F32SMALL;
                    let sellMaxPrice =
                        sellMarket.get_sell_price(goodKind, F32SMALL).unwrap() / F32SMALL;

                    // Trying to get the max altcoin (aka goodKind) quantity to buy, such as the eur to send is less or equal than the ones I have
                    let mut maxAltToReceive =
                        match buyMarket.get_buy_price(goodKind, F32LARGE).unwrap_err() {
                            MarketGetterError::NonPositiveQuantityAsked => unimplemented!(),
                            MarketGetterError::InsufficientGoodQuantityAvailable {
                                requested_good_kind,
                                requested_good_quantity,
                                available_good_quantity,
                            } => available_good_quantity,
                        };
                    let mut maxEurToSend =
                        buyMarket.get_buy_price(goodKind, maxAltToReceive).unwrap();
                    while maxEurToSend > eur.get_qty() {
                        maxAltToReceive /= 2.;
                        maxEurToSend = buyMarket.get_buy_price(goodKind, maxAltToReceive).unwrap();
                    }

                    // Get remaining bounds for prices
                    let buyMaxPrice = F32SMALL
                        / (maxEurToSend
                            - (buyMarket
                                .get_buy_price(goodKind, maxAltToReceive - F32SMALL)
                                .unwrap()));
                    let sellMinPrice = F32SMALL
                        / (sellMarket
                            .get_sell_price(goodKind, maxAltToReceive)
                            .unwrap()
                            - sellMarket
                                .get_sell_price(goodKind, maxAltToReceive - F32SMALL)
                                .unwrap());

                    // Skip markets where prices are not growing with the quantity
                    if buyMaxPrice < buyMinPrice || sellMaxPrice < sellMinPrice {
                        continue;
                    }

                    // Compute profit
                    let mut profit = if buyMaxPrice <= sellMinPrice {
                        // First case
                        sellMarket
                            .get_sell_price(goodKind, maxAltToReceive)
                            .unwrap()
                            - maxEurToSend
                    } else {
                        // Second case
                        maxEurToSend
                            * ((sellMinPrice - buyMinPrice)
                                / (buyMaxPrice - buyMinPrice + sellMaxPrice - sellMinPrice))
                    };

                    if profit > bestProfit {
                        bestProfit = profit;
                        bestMaxAltToReceive = maxAltToReceive;
                        bestKindAltToReceive = goodKind;
                        bestMaxEurToSend = maxEurToSend;
                        bestBuyMarket = _buyMarket.deref().borrow_mut();
                        bestSellMarket = _sellMarket.deref().borrow_mut();
                    }
                }
            }
        }

        if bestProfit <= 0. {
            return (eur, None);
        }

        // Do actual trade
        let buyToken = match bestBuyMarket.borrow_mut().lock_buy(
            bestKindAltToReceive,
            bestMaxAltToReceive * 0.999,
            bestMaxEurToSend,
            self.traderName,
        ) {
            Ok(token) => token,
            Err(err) => unimplemented!(),
        };
        // TODO handle bestMaxEurToSend==
        let altCoin = bestBuyMarket
            .borrow_mut()
            .buy(buyToken, &mut eur.split(bestMaxEurToSend).unwrap())
            .unwrap();
        let sellToken = match bestSellMarket.borrow_mut().lock_sell(
            bestKindAltToReceive,
            altCoin.get_qty(),
            bestMaxEurToSend + bestProfit,
            self.traderName,
        ) {
            Ok(token) => token,
            Err(err) => unimplemented!(),
        };
        let altCoinQuantityReceived = altCoin.get_qty();
        let eurReceived = bestSellMarket
            .borrow_mut()
            .sell(sellToken, &mut altCoin)
            .unwrap();

        let eurQuantityReceived = eurReceived.get_qty();
        eur.merge(eurReceived);

        return (
            eur,
            Some(ArbitrageResult {
                eurSent: bestMaxEurToSend,
                altReceived: altCoinQuantityReceived,
                eurReceived: eurQuantityReceived,
                buyMarketName: bestBuyMarket.get_name().to_string(),
                sellMarketName: bestSellMarket.get_name().to_string(),
            }),
        );
    }
}
