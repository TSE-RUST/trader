use std::cell::RefCell;
use std::rc::Rc;
// library dependencies
use druid::im::Vector;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;
use crate::visualizers::datas::trader_ui_derived_lenses::quantity;

/// returns the GoodKind of a given good
///
/// **Federico Brancasi**
fn get_good(good: &String) -> GoodKind {
    if good == "EUR" {
        GoodKind::EUR
    } else if good == "YEN" {
        GoodKind::YEN
    } else if good == "USD" {
        GoodKind::USD
    } else {
        GoodKind::YUAN
    }
}

/// returns the index of a given good
///
/// **Federico Brancasi**
fn get_index(string: &String) -> usize {
    if string == "EUR" {
        0
    } else if string == "YEN" {
        1
    } else if string == "USD" {
        2
    } else {
        3
    }
}

// fn get_index_bfb(string: &String) -> usize {
//     if string == "EUR" {
//         3
//     } else if string == "YEN" {
//         1
//     } else if string == "USD" {
//         0
//     } else {
//         2
//     }
// }

// fn get_index_sol(string: &String) -> usize {
//     if string == "EUR" {
//         1
//     } else if string == "YEN" {
//         2
//     } else if string == "USD" {
//         3
//     } else {
//         0
//     }
// }

// fn get_index_parse(string: &String) -> usize {
//     if string == "EUR" {
//         3
//     } else if string == "YEN" {
//         1
//     } else if string == "USD" {
//         0
//     } else {
//         2
//     }
// }

/// returns the max quantity of a given good that can be bought or sold
///
/// **Federico Brancasi**
pub(crate) fn max_qt(markets: &Vector<Rc<RefCell<dyn Market>>>, goods_trader: &Vector<f32>, method: &String, market: &String, good: &String, _bfb: &Vector<f32>, _sol: &Vector<f32>, _parse: &Vector<f32>) -> f32 {
    let goodkind = get_good(&good);
    let mut ret = 0.0 as f32;
    if method == "SELL" {
        if market == "BFB" {
            ret = crate::bots::bot::get_max_sell_quantity(
                &markets[0],
                goods_trader[get_index(&good)],
                goodkind,
            );
            println!("BFB max: {}", ret);
        } else if market == "SOL" {
            ret = crate::bots::bot::get_max_sell_quantity(
                &markets[1],
                goods_trader[get_index(&good)],
                goodkind,
            );
            println!("SOL max: {}", ret);
        } else if market == "PARSE" {
            ret = crate::bots::bot::get_max_sell_quantity(
                &markets[2],
                goods_trader[get_index(&good)],
                goodkind,
            );
            println!("PARSE max: {}", ret);
        }
    } else if method == "BUY" {
        if market == "BFB" {
            ret = crate::bots::bot::get_max_buy_quantity(
                &markets[0],
                // bfb[get_index_bfb(&good)],
                goods_trader[0],
                goodkind,
            );
            println!("BFB max: {}", ret);
        } else if market == "SOL" {
            ret = crate::bots::bot::get_max_buy_quantity(
                &markets[1],
                // sol[get_index_sol(&good)],
                goods_trader[0],
                goodkind,
            );
            println!("SOL max: {}", ret);
        } else if market == "PARSE" {
            ret = crate::bots::bot::get_max_buy_quantity(
                &markets[2],
                // parse[get_index_parse(&good)],
                goods_trader[0],
                goodkind,
            );
            println!("PARSE max: {}", ret);
        }
    }
    ret
}

///get the average sell price taken by the maximum between the 1/3 of the trader.budget and the good availability in the market
///
/// **Andrea Ballarini**
pub fn get_average_sell(markets: &Vector<Rc<RefCell<dyn Market>>>,kind:GoodKind, qty:f32)->f32{
    let mut price_sol:f32 = 0.;
    let mut price_parse:f32 = 0.;
    let mut price_bfb:f32 = 0.;

    //bfb
    let max_quantity_bfb = crate::bots::bot::get_max_sell_quantity(&markets[0], qty, kind);
    if max_quantity_bfb != 0.0{
        price_bfb = markets[0].borrow().get_sell_price(kind, max_quantity_bfb).unwrap();
    }

    //sol
    let max_quantity_sol = crate::bots::bot::get_max_sell_quantity(&markets[1], qty, kind);
    if max_quantity_sol > 0.0 {
        price_sol = markets[1].borrow().get_sell_price(kind, max_quantity_sol).unwrap();
    }

    //parse
    let max_quantity_parse = crate::bots::bot::get_max_sell_quantity(&markets[2], qty, kind);
    if max_quantity_parse != 0.0{
        price_parse = markets[2].borrow().get_sell_price(kind, max_quantity_parse).unwrap();
    }

    //get the average price
    let average_price = (price_sol + price_parse + price_bfb) / (max_quantity_sol + max_quantity_parse + max_quantity_bfb);

    average_price
}

///get the average buy price taken by the maximum between the budget of the trader and the good availability in the market
///
/// **Andrea Ballarini**
pub fn get_average_buy(markets: &Vector<Rc<RefCell<dyn Market>>>,kind:GoodKind, qty:f32)->f32{
    let mut price_sol:f32 = 0.;
    let mut price_parse:f32 = 0.;
    let mut price_bfb:f32 = 0.;

    //bfb
    let max_quantity_bfb = crate::bots::bot::get_max_buy_quantity(&markets[0], qty, kind);
    if max_quantity_bfb != 0.0{
        price_bfb = markets[0].borrow().get_buy_price(kind, max_quantity_bfb).unwrap();
    }

    //sol
    let max_quantity_sol = crate::bots::bot::get_max_buy_quantity(&markets[1], qty, kind);
    if max_quantity_sol > 0.0 {
        price_sol = markets[1].borrow().get_buy_price(kind, max_quantity_sol).unwrap();
    }

    //parse
    let max_quantity_parse = crate::bots::bot::get_max_buy_quantity(&markets[2], qty, kind);
    if max_quantity_parse != 0.0{
        price_parse = markets[2].borrow().get_buy_price(kind, max_quantity_parse).unwrap();
    }

    //get the average price
    let average_price = (price_sol + price_parse + price_bfb) / (max_quantity_sol + max_quantity_parse + max_quantity_bfb);

    average_price
}

/// # GET BEST BUY MARKET, QUANTITY AND PRICE
/// get the best market to buy a specific good where the price is lower than the average price of the three markets and preferring the BFB market
///
/// **Andrea Ballarini**
pub fn get_best_buy(markets: &Vector<Rc<RefCell<dyn Market>>>, kind: GoodKind, qty: f32,) -> (String, f32, f32) {
    let average_price = get_average_buy(markets, kind,qty);
    let mut best_price = 0.0;
    let mut best_quantity = 0.0;

    //get the best price in the bfb market
    let max_quantity_bfb = crate::bots::bot::get_max_buy_quantity(&markets[0], qty, kind);
    let mut price = match markets[0].borrow().get_buy_price(kind, max_quantity_bfb){
        Ok(price) => price,
        _ => 0.0,
    };
    let mut best_market= "BFB".to_string();
    if max_quantity_bfb > 0.0 {
        if (price/max_quantity_bfb) < average_price && average_price > 0.0 {
            best_price = price;
            best_quantity = max_quantity_bfb;
            return (best_market,best_quantity,best_price);
        }
    }

    //get the best price in the sol market
    let max_quantity_sol = crate::bots::bot::get_max_buy_quantity(&markets[1], qty, kind);
    price = match markets[1].borrow().get_buy_price(kind, max_quantity_sol){
        Ok(price) => price,
        _ => 0.0,
    };
    if max_quantity_sol > 0.0 {
        if (price/max_quantity_sol) < average_price && average_price > 0.0 {
            best_price = price;
            best_quantity = max_quantity_sol;
            best_market = "SOL".to_string();
        }
    }

    //get the best price in the parse market
    let max_quantity_parse = crate::bots::bot::get_max_buy_quantity(&markets[2], qty, kind);
    price = match markets[2].borrow().get_buy_price(kind, max_quantity_parse){
        Ok(price) => price,
        _ => 0.0,
    };
    if max_quantity_parse > 0.0 {
        if (price/max_quantity_parse) < average_price && average_price > 0.0 {
            best_price = price;
            best_quantity = max_quantity_parse;
            best_market = "PARSE".to_string();
        }
    }

    (best_market,best_quantity,best_price)
}

/// # GET BEST SELL MARKET, QUANTITY AND PRICE
/// get the best market to sell a specific good where the price is higher than the average price of the three markets and preferring the PARSE market
pub fn get_best_sell(markets: &Vector<Rc<RefCell<dyn Market>>>, kind: GoodKind, qty: f32,) -> (String, f32, f32) {
    let average_price = get_average_sell(markets, kind, qty);
    let mut best_price = 0.0;
    let mut best_quantity = 0.0;

    //get the best price in the parse market
    let max_quantity_parse = crate::bots::bot::get_max_sell_quantity(&markets[2], qty, kind);
    let mut price = match markets[2].borrow().get_sell_price(kind, max_quantity_parse) {
        Ok(price) => price,
        _ => 0.0,
    };
    let mut best_market = "PARSE".to_string();
    if max_quantity_parse > 0.1 {
        if (price / max_quantity_parse) > average_price && average_price > 0.0 {
            best_price = price;
            best_quantity = max_quantity_parse;
            return (best_market, best_quantity, best_price);
        }
    }

    //get the best price in the sol market
    let max_quantity_sol = crate::bots::bot::get_max_sell_quantity(&markets[1], qty, kind);
    price = match markets[1].borrow().get_sell_price(kind, max_quantity_sol) {
        Ok(price) => price,
        _ => 0.0,
    };
    if max_quantity_sol > 0.1 {
        if (price / max_quantity_sol) > average_price && average_price > 0.0 {
            best_price = price;
            best_quantity = max_quantity_sol;
            best_market = "SOL".to_string();
        }
    }

    //get the best price in the bfb market
    let max_quantity_bfb = crate::bots::bot::get_max_sell_quantity(&markets[0], qty, kind);
    price = match markets[0].borrow().get_sell_price(kind, max_quantity_bfb) {
        Ok(price) => price,
        _ => 0.0,
    };
    if max_quantity_bfb > 0.1 {
        if (price / max_quantity_bfb) > average_price && average_price > 0.0 {
            best_price = price;
            best_quantity = max_quantity_bfb;
            best_market = "BFB".to_string();
        }
    }

    (best_market, best_quantity, best_price)
}

///get the best buy trade for a trader
///
/// **Andrea Ballarini**
pub fn get_best_buy_trade(markets: &Vector<Rc<RefCell<dyn Market>>>, qty: f32,) -> String {
    let (_,yen_quantity,yen_price) = get_best_buy(markets, GoodKind::YEN,qty);
    let average_yen= yen_quantity/yen_price;
    let (_,usd_quantity,usd_price) = get_best_buy(markets, GoodKind::USD,qty);
    let average_usd= usd_quantity/usd_price;
    let (_,yuan_quantity,yuan_price) = get_best_buy(markets, GoodKind::YUAN,qty);
    let average_yuan= yuan_quantity/yuan_price;


    let mut res ="".to_string();

    if (average_yuan > average_yen) && (average_yuan > average_usd) {
        let (yuan_market,yuan_quantity,yuan_price) = get_best_buy(markets, GoodKind::YUAN,qty);
        res = format!("buy {} yuan from {} for {} ", yuan_quantity, yuan_market, yuan_price);
    } else if (average_yen > average_usd) && (average_yen > average_yuan) {
        let (usd_market,usd_quantity,usd_price) = get_best_buy(markets, GoodKind::USD,qty);
        res = format!("buy {} usd from {} for {} ", usd_quantity, usd_market, usd_price);
    } else {
        let (yen_market,yen_quantity,yen_price) = get_best_buy(markets, GoodKind::YEN,qty);
        res = format!("buy {} yen from {} for {} ", yen_quantity, yen_market, yen_price);
    }
    res
}

///get the best sell trade for a trader
///
/// **Andrea Ballarini**
pub fn get_best_sell_trade(markets: &Vector<Rc<RefCell<dyn Market>>>, goods: &Vector<f32>) -> String {
    let (_,yen_quantity,yen_price) = get_best_sell(markets, GoodKind::YEN,goods[1]);
    let average_yen= yen_price/yen_quantity;
    let (_,usd_quantity,usd_price) = get_best_sell(markets, GoodKind::USD,goods[2]);
    let average_usd= usd_price/usd_quantity;
    let (_,yuan_quantity,yuan_price) = get_best_sell(markets, GoodKind::YUAN,goods[3]);
    let average_yuan= yuan_price/yuan_quantity;

    let mut res ="".to_string();

    if (average_yuan > average_yen) && (average_yuan > average_usd) {
        let (yuan_market,yuan_quantity,yuan_price) = get_best_sell(markets, GoodKind::YUAN,goods[3]);
        res = format!("sell {} yuan to {} for {} ", yuan_quantity, yuan_market, yuan_price);
    } else if (average_yen > average_usd) && (average_yen > average_yuan) {
        let (usd_market,usd_quantity,usd_price) = get_best_sell(markets, GoodKind::USD,goods[2]);
        res = format!("sell {} usd to {} for {} ", usd_quantity, usd_market, usd_price);
    } else {
        let (yen_market,yen_quantity,yen_price) = get_best_sell(markets, GoodKind::YEN,goods[1]);
        res = format!("sell {} yen to {} for {} ", yen_quantity, yen_market, yen_price);
    }
    res
}