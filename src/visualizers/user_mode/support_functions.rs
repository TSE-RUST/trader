use std::cell::RefCell;
use std::rc::Rc;
// library dependencies
use druid::im::Vector;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;

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
        } else if market == "SOL" {
            ret = crate::bots::bot::get_max_sell_quantity(
                &markets[1],
                goods_trader[get_index(&good)],
                goodkind,
            );
        } else if market == "PARSE" {
            ret = crate::bots::bot::get_max_sell_quantity(
                &markets[2],
                goods_trader[get_index(&good)],
                goodkind,
            );
        }
    } else if method == "BUY" {
        if market == "BFB" {
            ret = crate::bots::bot::get_max_buy_quantity(
                &markets[0],
                // bfb[get_index_bfb(&good)],
                goods_trader[0],
                goodkind,
            );
        } else if market == "SOL" {
            ret = crate::bots::bot::get_max_buy_quantity(
                &markets[1],
                // sol[get_index_sol(&good)],
                goods_trader[0],
                goodkind,
            );
        } else if market == "PARSE" {
            ret = crate::bots::bot::get_max_buy_quantity(
                &markets[2],
                // parse[get_index_parse(&good)],
                goods_trader[0],
                goodkind,
            );
        }
    }
    ret
}