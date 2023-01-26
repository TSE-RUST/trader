// library dependencies
use druid::im::{Vector, vector};
use std::cell::RefCell;
use std::rc::Rc;

// market dependencies
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;

// local dependencies
use crate::visualizers::user_mode::datas::TraderUi;


/// SUPPORT FUNCTIONS FOR THE TraderUi STRUCT - initializer of the TraderUi
/// struct. This function is called when the TraderUi is created and it
/// initializes the TraderUi struct datas when the program starts
///
/// **Federico Brancasi**
pub(crate) fn initialize_quantities(app: &mut TraderUi) -> &mut TraderUi {

    // set values for bfb market
    let (good_kinds_bfb, quantities_bfb, exchange_rate_buy_bfb, exchange_rate_sell_bfb) = get_market_info(&app.markets[0]);

    app.bfb_kinds = good_kinds_bfb;
    app.bfb_quantities = quantities_bfb;
    app.bfb_exchange_rate_buy = exchange_rate_buy_bfb;
    app.bfb_exchange_rate_sell = exchange_rate_sell_bfb;

    // set values for sol market
    let (good_kinds_sol, quantities_sol, exchange_rate_buy_sol, exchange_rate_sell_sol) = get_market_info(&app.markets[1]);

    app.sol_kinds = good_kinds_sol;
    app.sol_quantities = quantities_sol;
    app.sol_exchange_rate_buy = exchange_rate_buy_sol;
    app.sol_exchange_rate_sell = exchange_rate_sell_sol;

    // set values for parse market
    let (good_kinds_parse, quantities_parse, exchange_rate_buy_parse, exchange_rate_sell_parse) = get_market_info(&app.markets[2]);

    app.parse_kinds = good_kinds_parse;
    app.parse_quantities = quantities_parse;
    app.parse_exchange_rate_buy = exchange_rate_buy_parse;
    app.parse_exchange_rate_sell = exchange_rate_sell_parse;

    app.trader.goods = vector![100000.0, 100000.0, 1000000.0, 0.0];

    app.quantity = app.trader.goods[0];

    app.string_best_profit_buy = get_best_buy_trade(&app.markets, app.trader.goods[0]);
    app.string_best_profit_sell = get_best_sell_trade(&app.markets, &app.trader.goods);

    app
}

/// SUPPORT FUNCTIONS FOR THE TraderUi STRUCT - The get_market_info function
/// returns the market info in order to be used in the initialize_quantities
/// function and sort the markets info in order to be displayed in the same
/// order in the UI
///
/// **Federico Brancasi**
pub fn get_market_info(market: &Rc<RefCell<dyn Market>>) -> (Vector<String>, Vector<f32>, Vector<f32>, Vector<f32>) {
    let mut buy_rates_temp = Vector::new();
    let mut sell_rates_temp = Vector::new();
    let mut good_kinds_temp = Vector::new();
    let mut quantities_temp = Vector::new();

    for good in market.borrow().get_goods() {
        let buy_rate = good.exchange_rate_buy;
        let sell_rate = good.exchange_rate_sell;
        let quantity = good.quantity;
        let kind = good.good_kind.to_string();
        buy_rates_temp.push_back(buy_rate);
        sell_rates_temp.push_back(sell_rate);
        quantities_temp.push_back(quantity);
        good_kinds_temp.push_back(kind);
    }

    let mut buy_rates = vector![0.0, 0.0, 0.0, 0.0];
    let mut sell_rates = vector![0.0, 0.0, 0.0, 0.0];
    let mut good_kinds = vector![" ".to_string(), " ".to_string(), " ".to_string(), " ".to_string()];
    let mut quantities = vector![0.0, 0.0, 0.0, 0.0];

    for i in 0..4 {
        if good_kinds_temp[i] == "EUR".to_string() {
            buy_rates[0] = buy_rates_temp[i];
            sell_rates[0] = sell_rates_temp[i];
            quantities[0] = quantities_temp[i];
            good_kinds[0] = "EUR".to_string();
        } else if good_kinds_temp[i] == "YEN".to_string() {
            buy_rates[1] = buy_rates_temp[i];
            sell_rates[1] = sell_rates_temp[i];
            quantities[1] = quantities_temp[i];
            good_kinds[1] = "YEN".to_string();
        } else if good_kinds_temp[i] == "USD".to_string() {
            buy_rates[2] = buy_rates_temp[i];
            sell_rates[2] = sell_rates_temp[i];
            quantities[2] = quantities_temp[i];
            good_kinds[2] = "USD".to_string();
        } else if good_kinds_temp[i] == "YUAN".to_string() {
            buy_rates[3] = buy_rates_temp[i];
            sell_rates[3] = sell_rates_temp[i];
            quantities[3] = quantities_temp[i];
            good_kinds[3] = "YUAN".to_string();
        }
    }

    (good_kinds, quantities, buy_rates, sell_rates)
}

// /// SUPPORT FUNCTIONS FOR THE TraderUi STRUCT - the get_market_index function returns
// /// the index of the market from the name
// ///
// /// **Federico Brancasi**
// pub fn get_market_index(market_name: &str) -> usize {
//     match market_name.to_lowercase().as_str() {
//         "bfb" => 0,
//         "sol" => 1,
//         "parse" => 2,
//         _ => panic!("Market not found"),
//     }
// }

/// SUPPORT FUNCTION FOR THE APPLICATION - returns the max
/// quantity of a given good that can be bought or sold
///
/// **Federico Brancasi**
pub(crate) fn max_qt(markets: &Vector<Rc<RefCell<dyn Market>>>, goods_trader: &Vector<f32>, method: &String, market: &String, good: &String, _bfb: &Vector<f32>, _sol: &Vector<f32>, _parse: &Vector<f32>) -> f32 {
    let goodkind = get_good(&good);
    let mut ret = 0.0 as f32;
    if method == "SELL" {
        if market == "BFB" {
            ret = crate::bots::bot_strategy::bot::get_max_sell_quantity(
                &markets[0],
                goods_trader[get_index(&good)],
                goodkind,
            );
            println!("BFB max: {}", ret);
        } else if market == "SOL" {
            ret = crate::bots::bot_strategy::bot::get_max_sell_quantity(
                &markets[1],
                goods_trader[get_index(&good)],
                goodkind,
            );
            println!("SOL max: {}", ret);
        } else if market == "PARSE" {
            ret = crate::bots::bot_strategy::bot::get_max_sell_quantity(
                &markets[2],
                goods_trader[get_index(&good)],
                goodkind,
            );
            println!("PARSE max: {}", ret);
        }
    } else if method == "BUY" {
        if market == "BFB" {
            ret = crate::bots::bot_strategy::bot::get_max_buy_quantity(
                &markets[0],
                // bfb[get_index_bfb(&good)],
                goods_trader[0],
                goodkind,
            );
            println!("BFB max: {}", ret);
        } else if market == "SOL" {
            ret = crate::bots::bot_strategy::bot::get_max_buy_quantity(
                &markets[1],
                // sol[get_index_sol(&good)],
                goods_trader[0],
                goodkind,
            );
            println!("SOL max: {}", ret);
        } else if market == "PARSE" {
            ret = crate::bots::bot_strategy::bot::get_max_buy_quantity(
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

/// SUPPORT FUNCTION FOR THE APPLICATION - returns
/// the GoodKind of a given good
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

/// SUPPORT FUNCTION FOR THE APPLICATION - returns
/// the index of a given good
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

/// SUPPORT FUNCTION FOR THE APPLICATION - get the average sell
/// price taken by the maximum between the 1/3 of the trader.budget
/// and the good availability in the market
///
/// **Andrea Ballarini**
pub fn get_average_sell(markets: &Vector<Rc<RefCell<dyn Market>>>, kind: GoodKind, qty: f32) -> f32 {
    let mut price_sol: f32 = 0.;
    let mut price_parse: f32 = 0.;
    let mut price_bfb: f32 = 0.;

    //bfb
    let max_quantity_bfb = crate::bots::bot_strategy::bot::get_max_sell_quantity(&markets[0], qty, kind);
    if max_quantity_bfb != 0.0 {
        price_bfb = markets[0].borrow().get_sell_price(kind, max_quantity_bfb).unwrap();
    }

    //sol
    let max_quantity_sol = crate::bots::bot_strategy::bot::get_max_sell_quantity(&markets[1], qty, kind);
    if max_quantity_sol > 0.0 {
        price_sol = markets[1].borrow().get_sell_price(kind, max_quantity_sol).unwrap();
    }

    //parse
    let max_quantity_parse = crate::bots::bot_strategy::bot::get_max_sell_quantity(&markets[2], qty, kind);
    if max_quantity_parse != 0.0 {
        price_parse = markets[2].borrow().get_sell_price(kind, max_quantity_parse).unwrap();
    }

    //get the average price
    let average_price = (price_sol + price_parse + price_bfb) / (max_quantity_sol + max_quantity_parse + max_quantity_bfb);

    average_price
}

/// SUPPORT FUNCTION FOR THE APPLICATION - get the average buy
/// price taken by the maximum between the budget of the trader
/// and the good availability in the market
///
/// **Andrea Ballarini**
pub fn get_average_buy(markets: &Vector<Rc<RefCell<dyn Market>>>, kind: GoodKind, qty: f32) -> f32 {
    let mut price_sol: f32 = 0.;
    let mut price_parse: f32 = 0.;
    let mut price_bfb: f32 = 0.;

    //bfb
    let max_quantity_bfb = crate::bots::bot_strategy::bot::get_max_buy_quantity(&markets[0], qty, kind);
    if max_quantity_bfb != 0.0 {
        price_bfb = markets[0].borrow().get_buy_price(kind, max_quantity_bfb).unwrap();
    }

    //sol
    let max_quantity_sol = crate::bots::bot_strategy::bot::get_max_buy_quantity(&markets[1], qty, kind);
    if max_quantity_sol > 0.0 {
        price_sol = markets[1].borrow().get_buy_price(kind, max_quantity_sol).unwrap();
    }

    //parse
    let max_quantity_parse = crate::bots::bot_strategy::bot::get_max_buy_quantity(&markets[2], qty, kind);
    if max_quantity_parse != 0.0 {
        price_parse = markets[2].borrow().get_buy_price(kind, max_quantity_parse).unwrap();
    }

    //get the average price
    let average_price = (price_sol + price_parse + price_bfb) / (max_quantity_sol + max_quantity_parse + max_quantity_bfb);

    average_price
}

/// # GET BEST BUY MARKET, QUANTITY AND PRICE
/// SUPPORT FUNCTION FOR THE APPLICATION - get the best market to buy
/// a specific good where the price is lower than the average price of
/// the three markets and preferring the BFB market
///
/// **Andrea Ballarini**
pub fn get_best_buy(markets: &Vector<Rc<RefCell<dyn Market>>>, kind: GoodKind, qty: f32) -> (String, f32, f32) {
    let average_price = get_average_buy(markets, kind, qty);
    let mut best_price = 0.0;
    let mut best_quantity = 0.0;

    //get the best price in the bfb market
    let max_quantity_bfb = crate::bots::bot_strategy::bot::get_max_buy_quantity(&markets[0], qty, kind);
    let mut price = match markets[0].borrow().get_buy_price(kind, max_quantity_bfb) {
        Ok(price) => price,
        _ => 0.0,
    };
    let mut best_market = "BFB".to_string();
    if max_quantity_bfb > 0.0 {
        if (price / max_quantity_bfb) < average_price && average_price > 0.0 {
            best_price = price;
            best_quantity = max_quantity_bfb;
            return (best_market, best_quantity, best_price);
        }
    }

    //get the best price in the sol market
    let max_quantity_sol = crate::bots::bot_strategy::bot::get_max_buy_quantity(&markets[1], qty, kind);
    price = match markets[1].borrow().get_buy_price(kind, max_quantity_sol) {
        Ok(price) => price,
        _ => 0.0,
    };
    if max_quantity_sol > 0.0 {
        if (price / max_quantity_sol) < average_price && average_price > 0.0 {
            best_price = price;
            best_quantity = max_quantity_sol;
            best_market = "SOL".to_string();
        }
    }

    //get the best price in the parse market
    let max_quantity_parse = crate::bots::bot_strategy::bot::get_max_buy_quantity(&markets[2], qty, kind);
    price = match markets[2].borrow().get_buy_price(kind, max_quantity_parse) {
        Ok(price) => price,
        _ => 0.0,
    };
    if max_quantity_parse > 0.0 {
        if (price / max_quantity_parse) < average_price && average_price > 0.0 {
            best_price = price;
            best_quantity = max_quantity_parse;
            best_market = "PARSE".to_string();
        }
    }

    (best_market, best_quantity, best_price)
}

/// # GET BEST SELL MARKET, QUANTITY AND PRICE
/// SUPPORT FUNCTION FOR THE APPLICATION - get the best market to sell
/// a specific good where the price is higher than the average price of
/// the three markets and preferring the PARSE market
///
/// **Andrea Ballarini**
pub fn get_best_sell(markets: &Vector<Rc<RefCell<dyn Market>>>, kind: GoodKind, qty: f32) -> (String, f32, f32) {
    let average_price = get_average_sell(markets, kind, qty);
    let mut best_price = 0.0;
    let mut best_quantity = 0.0;

    //get the best price in the parse market
    let max_quantity_parse = crate::bots::bot_strategy::bot::get_max_sell_quantity(&markets[2], qty, kind);
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
    let max_quantity_sol = crate::bots::bot_strategy::bot::get_max_sell_quantity(&markets[1], qty, kind);
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
    let max_quantity_bfb = crate::bots::bot_strategy::bot::get_max_sell_quantity(&markets[0], qty, kind);
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

/// SUPPORT FUNCTION FOR THE APPLICATION - get the best buy trade for a trader
///
/// **Andrea Ballarini**
pub fn get_best_buy_trade(markets: &Vector<Rc<RefCell<dyn Market>>>, qty: f32) -> String {
    let (_, yen_quantity, yen_price) = get_best_buy(markets, GoodKind::YEN, qty);
    let average_yen = yen_quantity / yen_price;
    let (_, usd_quantity, usd_price) = get_best_buy(markets, GoodKind::USD, qty);
    let average_usd = usd_quantity / usd_price;
    let (_, yuan_quantity, yuan_price) = get_best_buy(markets, GoodKind::YUAN, qty);
    let average_yuan = yuan_quantity / yuan_price;


    let res;

    if (average_yuan > average_yen) && (average_yuan > average_usd) {
        let (yuan_market, yuan_quantity, yuan_price) = get_best_buy(markets, GoodKind::YUAN, qty);
        res = format!("buy {} yuan from {} for {} ", yuan_quantity, yuan_market, yuan_price);
    } else if (average_yen > average_usd) && (average_yen > average_yuan) {
        let (usd_market, usd_quantity, usd_price) = get_best_buy(markets, GoodKind::USD, qty);
        res = format!("buy {} usd from {} for {} ", usd_quantity, usd_market, usd_price);
    } else {
        let (yen_market, yen_quantity, yen_price) = get_best_buy(markets, GoodKind::YEN, qty);
        res = format!("buy {} yen from {} for {} ", yen_quantity, yen_market, yen_price);
    }
    res
}

/// SUPPORT FUNCTION FOR THE APPLICATION - get the best sell trade for a trader
///
/// **Andrea Ballarini**
pub fn get_best_sell_trade(markets: &Vector<Rc<RefCell<dyn Market>>>, goods: &Vector<f32>) -> String {
    let (_, yen_quantity, yen_price) = get_best_sell(markets, GoodKind::YEN, goods[1]);
    let average_yen = yen_price / yen_quantity;
    let (_, usd_quantity, usd_price) = get_best_sell(markets, GoodKind::USD, goods[2]);
    let average_usd = usd_price / usd_quantity;
    let (_, yuan_quantity, yuan_price) = get_best_sell(markets, GoodKind::YUAN, goods[3]);
    let average_yuan = yuan_price / yuan_quantity;

    let res;

    if (average_yuan > average_yen) && (average_yuan > average_usd) {
        let (yuan_market, yuan_quantity, yuan_price) = get_best_sell(markets, GoodKind::YUAN, goods[3]);
        res = format!("sell {} yuan to {} for {} ", yuan_quantity, yuan_market, yuan_price);
    } else if (average_yen > average_usd) && (average_yen > average_yuan) {
        let (usd_market, usd_quantity, usd_price) = get_best_sell(markets, GoodKind::USD, goods[2]);
        res = format!("sell {} usd to {} for {} ", usd_quantity, usd_market, usd_price);
    } else {
        let (yen_market, yen_quantity, yen_price) = get_best_sell(markets, GoodKind::YEN, goods[1]);
        res = format!("sell {} yen to {} for {} ", yen_quantity, yen_market, yen_price);
    }
    res
}