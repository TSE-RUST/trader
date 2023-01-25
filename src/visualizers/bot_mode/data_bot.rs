use std::cell::RefCell;
use std::rc::Rc;

use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;

fn get_quantity_market(gk:GoodKind,market: &Rc<RefCell<dyn Market>>) -> String{
    let mut quantity = 0.0;
    let market = market.borrow();
    let goods = market.get_goods();
    match gk{
        GoodKind::YEN => {
            for i in 0..goods.len() {
                if goods[i].good_kind == gk {
                    quantity = goods[i].quantity;
                }
            }
        },
        GoodKind::USD => {
            for i in 0..goods.len() {
                if goods[i].good_kind == gk {
                    quantity = goods[i].quantity;
                }
            }
        },
        GoodKind::YUAN => {
            for i in 0..goods.len() {
                if goods[i].good_kind == gk {
                    quantity = goods[i].quantity;
                }
            }
        },
        GoodKind::EUR=>{
            for i in 0..goods.len() {
                if goods[i].good_kind == gk {
                    quantity = goods[i].quantity;
                }
            }
        },
        }
    quantity.to_string()
}