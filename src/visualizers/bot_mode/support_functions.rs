use std::cell::RefCell;
use std::rc::Rc;

use druid::{Widget, Color, WidgetExt};
use druid::im::Vector;
use druid::widget::Label;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;

use crate::visualizers::datas::TraderUi;

pub fn big_text(text: &str) -> impl Widget<TraderUi> {
    Label::new(text)
        .with_text_size(20.0)
        .with_text_color(Color::rgb(0.0, 0.0, 0.0))
        .background(Color::rgb(255.0, 255.0, 255.0))
        .center()
}

