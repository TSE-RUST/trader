// use std::borrow::BorrowMut;
// libraries dependencies
use druid::im::Vector;

use druid::{Data, Lens};
use std::cell::RefCell;
use std::rc::Rc;

// market dependencies
use unitn_market_2022::good::good::Good;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;
use unitn_market_2022::wait_one_day;

// local dependencies
use crate::bots::bot_strategy::market_functions::initgoods;

///the struct for the trader agent
#[derive(Clone, Data, Lens)]
pub struct TraderBot {
    pub(crate) name: String,
    pub money: f32,
    pub goods: Vector<Rc<RefCell<Good>>>,
    pub sol: Rc<RefCell<dyn Market>>,
    pub bfb: Rc<RefCell<dyn Market>>,
    pub parse: Rc<RefCell<dyn Market>>,
}

impl TraderBot {
    ///the constructor for the trader agent
    ///
    /// **Andrea Ballarini**
    pub(crate) fn new(
        name: String,
        money: f32,
        sol: Rc<RefCell<dyn Market>>,
        bfb: Rc<RefCell<dyn Market>>,
        parse: Rc<RefCell<dyn Market>>,
    ) -> Self {
        TraderBot {
            name,
            money: money,
            goods: initgoods(0.0, 0.0, 0.0),
            sol,
            bfb,
            parse,
        }
    }
}

///return a vector for iterating through the GoodKinds without the EUR
///
/// **Andrea Ballarini**
fn _get_good_kinds() -> Vec<GoodKind> {
    vec![GoodKind::USD, GoodKind::YEN, GoodKind::YUAN]
}

/// get the quantity of the kind in the trader
pub fn get_trader_quantity(trader: &TraderBot, kind: GoodKind) -> f32 {
    let mut quantity = 0.0;
    for good in &trader.goods {
        if good.borrow().get_kind() == kind {
            quantity = good.borrow().get_qty();
        }
    }
    quantity
}

///get the maximum quantity of a good that can be bought from a market with a specific quantity of money
///
/// **Andrea Ballarini**
pub fn get_max_buy_quantity(market: &Rc<RefCell<dyn Market>>, money: f32, kind: GoodKind) -> f32 {
    let market = market.borrow();
    let goods = market.get_goods();
    let mut max_quantity = 0.0;
    for i in 0..goods.len() {
        if goods[i].good_kind == kind {
            max_quantity = goods[i].quantity;
        }
    }
    if max_quantity > 0.0 {
        let mut buy_price = market
            .get_buy_price(kind, max_quantity)
            .expect("Error in get_buy_price in the max_buy_quantity function");
        while money < buy_price && max_quantity > 0.0 {
            max_quantity = max_quantity / 2.;
            buy_price = market
                .get_buy_price(kind, max_quantity)
                .expect("Error in get_buy_price in the max_buy_quantity function");
        }
    }

    max_quantity
}

///get the maximum quantity of a good that can be sold to a market with a specific quantity of that good and the availability of GoodKind::EUR in the market
///
/// **Andrea Ballarini**
pub fn get_max_sell_quantity(
    market: &Rc<RefCell<dyn Market>>,
    quantity: f32,
    kind: GoodKind,
) -> f32 {
    let market = market.borrow();
    let goods = market.get_goods();

    let mut eur_quantity = 0.0;

    let mut max_quantity = quantity;

    for i in 0..goods.len() {
        if goods[i].good_kind == GoodKind::EUR {
            eur_quantity = goods[i].quantity;
        }
    }
    //look if the market has the amount of GoodKind::EUR to buy the quantity of the good
    if max_quantity > 0.0 {
        // println!("sono nell'if: {}", max_quantity);
        let mut sell_price = market
            .get_sell_price(kind, max_quantity)
            .expect("Error in get_sell_price in the max_sell_quantity function");
        while eur_quantity < sell_price && max_quantity > 0.0 {
            //divido la quantità in 2 e ritento
            max_quantity = max_quantity / 2.;
            // println!("sono nel while: {}", max_quantity);
            sell_price = market
                .get_sell_price(kind, max_quantity)
                .expect("Error in get_sell_price in the max_sell_quantity function");
        }
    }

    max_quantity
}

///get the average buy price for 1 quantity of the good taken by the maximum between the 1/3 of the trader budget and the good availability in the specific market
///
/// **Andrea Ballarini**
pub fn get_average_buy_price(trader: &mut TraderBot, kind: GoodKind) -> f32 {
    //the budget is 1/3 of the trader.budget (trader.money)
    let budget = trader.money / 3.;

    let mut price_sol: f32 = 0.;
    let mut price_parse: f32 = 0.;
    let mut price_bfb: f32 = 0.;

    //get the sol maximum quantity that can be bought with the 1/3 of the budget
    let max_quantity_sol = get_max_buy_quantity(&trader.sol, budget, kind);
    if max_quantity_sol > 0.0 {
        price_sol = trader
            .sol
            .borrow()
            .get_buy_price(kind, max_quantity_sol)
            .unwrap();
    }

    //get the parse maximum quantity that can be bought with the 1/3 of the budget
    let max_quantity_parse = get_max_buy_quantity(&trader.parse, budget, kind);
    if max_quantity_parse > 0.0 {
        price_parse = trader
            .parse
            .borrow()
            .get_buy_price(kind, max_quantity_parse)
            .unwrap();
    }

    //get the bfb maximum quantity that can be bought with the 1/3 of the budget
    let max_quantity_bfb = get_max_buy_quantity(&trader.bfb, budget, kind);
    if max_quantity_bfb > 0.0 {
        price_bfb = trader
            .bfb
            .borrow()
            .get_buy_price(kind, max_quantity_bfb)
            .unwrap();
    }

    //get the average price of the three markets for the specific GoodKind and 1 quantity
    let average_price = (price_sol + price_parse + price_bfb)
        / (max_quantity_sol + max_quantity_parse + max_quantity_bfb);

    average_price
}

///get the average sell price taken by the maximum between the 1/3 of the trader.budget and the good availability in the market
///
/// **Andrea Ballarini**
pub fn get_average_sell_price(trader: &mut TraderBot, kind: GoodKind, quantity: f32) -> f32 {
    let mut price_sol: f32 = 0.;
    let mut price_parse: f32 = 0.;
    let mut price_bfb: f32 = 0.;

    //get the sol maximum quantity that can be sold with the budget
    let max_quantity_sol = get_max_sell_quantity(&trader.sol, quantity, kind);
    if max_quantity_sol > 0.0 {
        price_sol = trader
            .sol
            .borrow()
            .get_sell_price(kind, max_quantity_sol)
            .unwrap();
    }

    //get the parse maximum quantity that can be sold with the budget
    let max_quantity_parse = get_max_sell_quantity(&trader.parse, quantity, kind);
    if max_quantity_parse != 0.0 {
        price_parse = trader
            .parse
            .borrow()
            .get_sell_price(kind, max_quantity_parse)
            .unwrap();
    }

    //get the bfb maximum quantity that can be sold with the budget
    let max_quantity_bfb = get_max_sell_quantity(&trader.bfb, quantity, kind);
    if max_quantity_bfb != 0.0 {
        price_bfb = trader
            .bfb
            .borrow()
            .get_sell_price(kind, max_quantity_bfb)
            .unwrap();
    }

    //get the average price
    let average_price = (price_sol + price_parse + price_bfb)
        / (max_quantity_sol + max_quantity_parse + max_quantity_bfb);

    average_price
}

/// # GET BEST BUY MARKET, QUANTITY AND PRICE
/// get the best market to buy a specific good where the price is lower than the average price of the three markets and preferring the BFB market
///
/// **Andrea Ballarini**
pub fn get_best_buy_market(
    trader: &mut TraderBot,
    kind: GoodKind,
) -> (&mut Rc<RefCell<dyn Market>>, f32, f32) {
    let budget = trader.money / 3.;
    let average_price = get_average_buy_price(trader, kind);
    let mut best_price = 0.0;
    let mut best_quantity = 0.0;

    //get the best price in the bfb market
    let max_quantity_bfb = get_max_buy_quantity(&trader.bfb, budget, kind);
    let mut price = match trader.bfb.borrow().get_buy_price(kind, max_quantity_bfb) {
        Ok(price) => price,
        _ => 0.0,
    };
    let mut best_market = &mut trader.bfb;
    if max_quantity_bfb > 0.0 {
        if (price / max_quantity_bfb) < average_price && average_price > 0.0 {
            best_price = price;
            best_quantity = max_quantity_bfb;
            return (best_market, best_quantity, best_price);
        }
    }

    //get the best price in the sol market
    let max_quantity_sol = get_max_buy_quantity(&trader.sol, budget, kind);
    if max_quantity_sol > 0.0 {
        price = trader
            .sol
            .borrow()
            .get_buy_price(kind, max_quantity_sol)
            .unwrap();
        if (price / max_quantity_sol) < average_price {
            best_market = &mut trader.sol;
            best_price = price;
            best_quantity = max_quantity_sol;
        }
    }

    //get the best price in the parse market

    let max_quantity_parse = get_max_buy_quantity(&trader.parse, budget, kind);
    if max_quantity_parse > 0.1 {
        price = trader
            .parse
            .borrow()
            .get_buy_price(kind, max_quantity_parse)
            .unwrap();
        if (price / max_quantity_parse) < average_price {
            best_market = &mut trader.parse;
            best_price = price;
            best_quantity = max_quantity_parse;
        }
    }

    (best_market, best_quantity, best_price)
}

///# GET BEST SELL MARKET, QUANTITY AND PRICE
/// get the best market to sell a specific good where the price is higher than the average price of the three markets and preferring the PARSE market
///
/// **Andrea Ballarini**
pub fn get_best_sell_market(
    trader: &mut TraderBot,
    kind: GoodKind,
    quantity: f32,
) -> (&mut Rc<RefCell<dyn Market>>, f32, f32) {
    let average_price = get_average_sell_price(trader, kind, quantity);
    let mut best_price = 0.0;
    let mut best_quantity = 0.0;

    //get the best price in the parse market
    let max_quantity_parse = get_max_sell_quantity(&trader.parse, quantity, kind);
    let mut price = trader
        .parse
        .borrow()
        .get_sell_price(kind, max_quantity_parse)
        .unwrap();
    let mut best_market = &mut trader.parse;
    if max_quantity_parse > 0.1 {
        if (price / max_quantity_parse) > average_price && average_price != 0.0 {
            best_price = price;
            best_quantity = max_quantity_parse;
            return (best_market, best_quantity, best_price);
        }
    }

    //get the best price in the sol market
    let max_quantity_sol = get_max_sell_quantity(&trader.sol, quantity, kind);
    if max_quantity_sol > 0.0 {
        price = trader
            .sol
            .borrow()
            .get_sell_price(kind, max_quantity_sol)
            .unwrap();
        if (price / max_quantity_sol) > average_price {
            best_market = &mut trader.sol;
            best_price = price;
            best_quantity = max_quantity_sol;
        }
    }

    //get the best price in the parse market
    let max_quantity_bfb = get_max_sell_quantity(&trader.bfb, quantity, kind);
    if max_quantity_bfb > 0.1 {
        price = trader
            .bfb
            .borrow()
            .get_sell_price(kind, max_quantity_bfb)
            .unwrap();
        if (price / max_quantity_bfb) > average_price {
            best_market = &mut trader.bfb;
            best_price = price;
            best_quantity = max_quantity_bfb;
        }
    }

    (best_market, best_quantity, best_price)
}

// ///get the best buy trade for a trader
// ///
// /// **Andrea Ballarini**
// pub fn get_best_buy_trade(trader: &mut Trader) -> String {
//     let (_,yen_quantity,yen_price) = get_best_buy_market(trader, GoodKind::YEN);
//     let average_yen= (yen_quantity/yen_price);
//     let (_,usd_quantity,usd_price) = get_best_buy_market(trader, GoodKind::USD);
//     let average_usd= (usd_quantity/usd_price);
//     let (_,yuan_quantity,yuan_price) = get_best_buy_market(trader, GoodKind::YUAN);
//     let average_yuan= (yuan_quantity/yuan_price);
//
//
//     let mut res ="".to_string();
//
//     if (average_yuan > average_yen) && (average_yuan > average_usd) {
//         let (yuan_market,yuan_quantity,yuan_price) = get_best_buy_market(trader, GoodKind::YUAN);
//         res = format!("buy {} yuan from {} for {} each", yuan_quantity, yuan_market.borrow().get_name(), yuan_price/yuan_quantity);
//     } else if (average_yen > average_usd) && (average_yen > average_yuan) {
//         let (usd_market,usd_quantity,usd_price) = get_best_buy_market(trader, GoodKind::USD);
//         res = format!("buy {} usd from {} for {} each", usd_quantity, usd_market.borrow().get_name(), usd_price/usd_quantity);
//     } else {
//         let (yen_market,yen_quantity,yen_price) = get_best_buy_market(trader, GoodKind::YEN);
//         res = format!("buy {} yen from {} for {} each", yen_quantity, yen_market.borrow().get_name(), yen_price/yen_quantity);
//     }
//     res
// }
//
// ///get the best sell trade for a trader
// ///
// /// **Andrea Ballarini**
// pub fn get_best_sell_trade(trader: &mut Trader) -> String {
//     let (_,yen_quantity,yen_price) = get_best_sell_market(trader, GoodKind::YEN,);
//     let average_yen= (yen_quantity/yen_price);
//     let (_,usd_quantity,usd_price) = get_best_sell_market(trader, GoodKind::USD);
//     let average_usd= (usd_quantity/usd_price);
//     let (_,yuan_quantity,yuan_price) = get_best_sell_market(trader, GoodKind::YUAN);
//     let average_yuan= (yuan_quantity/yuan_price);
//
//
//     let mut res ="".to_string();
//
//     if (average_yuan > average_yen) && (average_yuan > average_usd) {
//         let (yuan_market,yuan_quantity,yuan_price) = get_best_sell_market(trader, GoodKind::YUAN);
//         res = format!("sell {} yuan to {} for {} each", yuan_quantity, yuan_market.borrow().get_name(), yuan_price/yuan_quantity);
//     } else if (average_yen > average_usd) && (average_yen > average_yuan) {
//         let (usd_market,usd_quantity,usd_price) = get_best_sell_market(trader, GoodKind::USD);
//         res = format!("sell {} usd to {} for {} each", usd_quantity, usd_market.borrow().get_name(), usd_price/usd_quantity);
//     } else {
//         let (yen_market,yen_quantity,yen_price) = get_best_sell_market(trader, GoodKind::YEN);
//         res = format!("sell {} yen to {} for {} each", yen_quantity, yen_market.borrow().get_name(), yen_price/yen_quantity);
//     }
//     res
// }

// /// # The BUY Trade function
// /// This function is used to buy a specific good from a specific market
// pub fn buy_trade<'b>(trader: &'b mut Trader, market: &mut Rc<RefCell<dyn Market>>, quantity: f32, good: GoodKind, price: f32,trader_name: String, market_name: String) {
//     let mut cash = Good::new(GoodKind::EUR, price);
//
//     let token = match market.borrow_mut().lock_buy(good, quantity, price,trader_name.clone()){
//         Ok(token) => token,
//         Err(e) => {panic!("Error in lock_buy in {}: {:?}", market_name.to_string(),e);},
//     };
//     let increase= match market.borrow_mut().buy(token, &mut cash){
//         Ok(increase) => increase,
//         Err(e) => {panic!("Error in buy in {}: {:?}", market_name.to_string(),e);},
//     };
//     trader.money -= price;
//     for kind in trader.goods.iter_mut() {
//         if kind.borrow().get_kind() == good {
//             match kind.borrow_mut().merge(increase.clone()){
//                 Ok(_) => (),
//                 Err(e) => println!("Error in merge {:?}", e),
//             }
//         }
//     }
// }
//
// /// # The SELL Trade function
// /// This function is used to sell a specific good to a specific market
// /// **Andrea Ballarini**
// pub fn sell_trade<'b>(trader: &'b mut Trader, market: &mut Rc<RefCell<dyn Market>>, quantity: f32, good: GoodKind, price: f32,trader_name: String,market_name: String) {
//     let token = match market.borrow_mut().lock_sell(good, quantity, price,trader_name) {
//         Ok(token) => token,
//         Err(e) => {panic!("Error in lock_sell in {}: {:?}", market_name.to_string(),e);},
//     };
//     let mut cash = Good::new(good, quantity);
//     let _decrease = match market.borrow_mut().sell(token, &mut cash){
//         Ok(decrease) => decrease,
//         Err(e) => {panic!("Error in sell in {}: {:?}", market_name.to_string(),e);},
//     };
//     trader.money += price;
//     for kind in trader.goods.iter_mut(){
//         if kind.borrow().get_kind() == good {
//             match kind.borrow_mut().split(quantity){
//                 Ok(_) => (),
//                 Err(e) => panic!("Error in split {:?}", e),
//             }
//         }
//     }
// }

/// # GET THE BEST BUY TRADE MARKET, GOODKIND AND QUANTITY/// get the best buy trade for a trader
///
/// Andrea Ballarini
pub fn get_best_buy_trade(trader: &mut TraderBot) -> (&mut Rc<RefCell<dyn Market>>, GoodKind, f32) {
    let (_, yen_quantity, yen_price) = get_best_buy_market(trader, GoodKind::YEN);
    let average_yen = if yen_quantity != 0. && yen_price != 0. {
        yen_quantity / yen_price
    } else {
        0.
    };
    let (_, usd_quantity, usd_price) = get_best_buy_market(trader, GoodKind::USD);
    let average_usd = if usd_quantity != 0. && usd_price != 0. {
        usd_quantity / usd_price
    } else {
        0.
    };
    let (_, yuan_quantity, yuan_price) = get_best_buy_market(trader, GoodKind::YUAN);
    let average_yuan = if yuan_price != 0. && yuan_quantity != 0. {
        yuan_quantity / yuan_price
    } else {
        0.0
    };

    let res;
    if (average_yuan > average_yen) && (average_yuan > average_usd) {
        let (yuan_market, yuan_quantity, _) = get_best_buy_market(trader, GoodKind::YUAN);
        res = (yuan_market, GoodKind::YUAN, yuan_quantity);
    } else if average_yen < average_usd {
        let (usd_market, usd_quantity, _) = get_best_buy_market(trader, GoodKind::USD);
        res = (usd_market, GoodKind::USD, usd_quantity);
    } else {
        let (yen_market, yen_quantity, _) = get_best_buy_market(trader, GoodKind::YEN);
        res = (yen_market, GoodKind::YEN, yen_quantity);
    }
    res
}

/// # GET THE BEST SELL TRADE MARKET, GOODKIND AND QUANTITY/// get the best sell trade for a trader
///
/// Andrea Ballarini
pub fn get_best_sell_trade(
    trader: &mut TraderBot,
) -> (&mut Rc<RefCell<dyn Market>>, GoodKind, f32) {
    let yen_trader = get_trader_quantity(trader, GoodKind::YEN) / 3.;
    let (_, yen_quantity, yen_price) = get_best_sell_market(trader, GoodKind::YEN, yen_trader);
    let average_yen = if yen_price != 0. && yen_quantity != 0. {
        yen_price / yen_quantity
    } else {
        0.
    };
    let usd_trader = get_trader_quantity(trader, GoodKind::USD) / 3.;
    let (_, usd_quantity, usd_price) = get_best_sell_market(trader, GoodKind::USD, usd_trader);
    let average_usd = if usd_price != 0. && usd_quantity != 0. {
        usd_price / usd_quantity
    } else {
        0.
    };
    let yuan_trader = get_trader_quantity(trader, GoodKind::YUAN) / 3.;
    let (_, yuan_quantity, yuan_price) = get_best_sell_market(trader, GoodKind::YUAN, yuan_trader);
    let average_yuan = if yuan_price != 0. && yuan_quantity != 0. {
        yuan_price / yuan_quantity
    } else {
        0.
    };

    let res;
    if (average_yuan > average_yen) && (average_yuan > average_usd) {
        let (yuan_market, yuan_quantity, _) =
            get_best_sell_market(trader, GoodKind::YUAN, yuan_trader);
        res = (yuan_market, GoodKind::YUAN, yuan_quantity);
    } else if average_yen < average_usd {
        let (usd_market, usd_quantity, _) = get_best_sell_market(trader, GoodKind::USD, usd_trader);
        res = (usd_market, GoodKind::USD, usd_quantity);
    } else {
        let (yen_market, yen_quantity, _) = get_best_sell_market(trader, GoodKind::YEN, yen_trader);
        res = (yen_market, GoodKind::YEN, yen_quantity);
    }
    res
}

/// # BOT
///Loop infinitely to buy and sell goods in the three markets and to print the money of the trader at the end of each day
///
/// **Andrea Ballarini**
pub fn bot(trader: &mut TraderBot, mut max: i32) -> Vector<String> {
    // let mut res: Vector<(String, String, String,String)> = Vector::new();
    let mut res_string: Vector<String> = Vector::new();

    loop {
        if max <= 0 {
            //log all the goods and the money of the trader
            // println!("The Trader has:");
            // for good in &trader.goods {
            //     println!("{}: {}", good.borrow().get_kind(), good.borrow().get_qty());
            // }
            // println!("{} money", trader.money);
            break;
        }

        //buy and sell goods

        //make the best buy trade
        let trader_name = trader.name.clone();
        // let (market, quantity, price) = get_best_buy_market(trader, good);
        let (market, good, quantity) = get_best_buy_trade(trader);
        let market_name = match market.borrow().get_name() {
            "SOL" => "SOL",
            "PARSE" => "PARSE",
            _ => "BFB",
        };

        if quantity > 0.1 {
            let price = match market.borrow().get_buy_price(good, quantity) {
                Ok(price) => price,
                Err(e) => {
                    panic!(
                        "Error in get_buy_price in {}: {:?}",
                        market_name.to_string(),
                        e
                    );
                }
            };
            let mut cash = Good::new(GoodKind::EUR, price);
            let token =
                match market
                    .borrow_mut()
                    .lock_buy(good, quantity, price, trader_name.clone())
                {
                    Ok(token) => token,
                    Err(e) => {
                        panic!("Error in lock_buy in {}: {:?}", market_name.to_string(), e);
                    }
                };
            let increase = match market.borrow_mut().buy(token, &mut cash) {
                Ok(increase) => increase,
                Err(e) => {
                    panic!("Error in buy in {}: {:?}", market_name.to_string(), e);
                }
            };
            trader.money -= price;
            for kind in trader.goods.iter_mut() {
                if kind.borrow().get_kind() == good {
                    match kind.borrow_mut().merge(increase.clone()) {
                        Ok(_) => (),
                        Err(e) => println!("Error in merge {:?}", e),
                    }
                }
            }
            // log of the precedent buy
            // println!(
            //     "{} bought {} {} from {} for {} EUR",
            //     trader.name,
            //     quantity,
            //     good.to_string(),
            //     market_name,
            //     price
            // );
            // res.push_back(("BUY".to_string(), good.to_string(), quantity.to_string(),market_name.clone()));
            max -= 1;
            res_string.push_back(format!(
                "{} {} {} {}",
                "BUY".to_string(),
                good.to_string(),
                quantity.to_string(),
                market_name
            ));
            // println!("Money: {}", trader.money);
        } else {
            // println!("No quantity to buy");
        }
        if max <= 0 {
            //log all the goods and the money of the trader
            // println!("The Trader has:");
            // for good in &trader.goods {
            //     println!("{}: {}", good.borrow().get_kind(), good.borrow().get_qty());
            // }
            // println!("{} money", trader.money);
            break;
        }

        //# make the best sell trade

        let (market, good, quantity) = get_best_sell_trade(trader);
        let market_name = match market.borrow().get_name() {
            "SOL" => "SOL",
            "PARSE" => "PARSE",
            _ => "BFB",
        };
        if quantity > 0.0 {
            let price = match market.borrow().get_sell_price(good, quantity) {
                Ok(price) => price,
                Err(e) => {
                    panic!(
                        "Error in get_sell_price in {}: {:?}",
                        market_name.to_string(),
                        e
                    );
                }
            };
            let token = match market
                .borrow_mut()
                .lock_sell(good, quantity, price, trader_name)
            {
                Ok(token) => token,
                Err(e) => {
                    panic!("Error in lock_sell in {}: {:?}", market_name.to_string(), e);
                }
            };
            let mut cash = Good::new(good, quantity);
            let _decrease = match market.borrow_mut().sell(token, &mut cash) {
                Ok(decrease) => decrease,
                Err(e) => {
                    panic!("Error in sell in {}: {:?}", market_name.to_string(), e);
                }
            };
            trader.money += price;
            for kind in trader.goods.iter_mut() {
                if kind.borrow().get_kind() == good {
                    match kind.borrow_mut().split(quantity) {
                        Ok(_) => (),
                        Err(e) => panic!("Error in split {:?}", e),
                    }
                }
            }
            //log of the precedent sell
            // println!(
            //     "{} sold {} {} to {} for {} EUR",
            //     trader.name,
            //     quantity,
            //     good.to_string(),
            //     market_name,
            //     price
            // );
            // res.push_back(("SELL".to_string(), good.to_string(), quantity.to_string(),market_name.clone()));
            res_string.push_back(format!(
                "{} {} {} {}",
                "SELL".to_string(),
                good.to_string(),
                quantity.to_string(),
                market_name.clone()
            ));
            max -= 1;
        } else {
            wait_one_day!(trader.sol);
            wait_one_day!(trader.bfb);
            wait_one_day!(trader.parse);
            //log of no sell action
            // println!("No quantity to sell");
        }

        // println!("Money: {}", trader.money);
        // }
    }
    //add the final money and goods quantities
    res_string.push_back(format!(
        "{} {} {} {}",
        trader.money,
        get_trader_quantity(trader, GoodKind::YEN),
        get_trader_quantity(trader, GoodKind::USD),
        get_trader_quantity(trader, GoodKind::YUAN)
    ));
    res_string
}
