use std::cell::RefCell;
use std::rc::Rc;

use druid::im::Vector;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;

pub fn get_quantity_market(gk:GoodKind,market: Vector<f32>) -> String{
    let mut quantity = 0.0;
    match gk{
        GoodKind::YEN => {
            quantity = market[1];
        },
        GoodKind::USD => {
            quantity = market[2];
        },
        GoodKind::YUAN => {
            quantity = market[3];
        },
        GoodKind::EUR=>{
            quantity = market[0];
        },
        }
    quantity.to_string()
}

