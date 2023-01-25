use std::cell::RefCell;
use std::rc::Rc;

use druid::{Widget, Color, WidgetExt};
use druid::im::Vector;
use druid::widget::Label;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;

use crate::Trader;
use crate::visualizers::datas::TraderUi;

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

pub fn big_text(text: &str) -> impl Widget<TraderUi> {
    Label::new(text)
        .with_text_size(20.0)
        .with_text_color(Color::rgb(0.0, 0.0, 0.0))
        .background(Color::rgb(255.0, 255.0, 255.0))
        .center()
}

// pub fn make_move(quantity: i32, data: &TraderUi){
//     let mut eur_tot=data.bfb_quantities[0]+data.sol_quantities[0]+data.parse_quantities[0];
//     if data.safe_mode{
//         Trader::new("bot", eur_tot, , bfb, parse)
//     }
// }

