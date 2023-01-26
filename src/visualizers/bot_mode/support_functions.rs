use std::cell::RefCell;
use std::rc::Rc;

use druid::{Widget, Color, WidgetExt};
use druid::im::Vector;
use druid::widget::{Label, Split, ViewSwitcher, Scroll, List};
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

pub fn string_log(data: &TraderUi) -> String {
    if data.safe_mode {

    } else {
        //da implementare log per arbitrager
    }
    ((data.percentage_bot * 100.0) as i32).to_string()
}

pub fn view_switcher()-> impl Widget<TraderUi> {
    let view_switcher = ViewSwitcher::new(
        |data: &TraderUi, _env| data.safe_mode,
        |selector, _data, _env| match _data.safe_mode {
            // the bots side is the first view
            true => Box::new(
                Split::columns(
                    Split::columns(
                        Split::rows(
                            big_text("BFB").background(Color::rgb(255.0, 255.0, 255.0)),
                            Scroll::new(
                                List::new(|| Label::dynamic(|data: &String, _| {
                                        format!("{data}")
                                })).lens(TraderUi::bfb_logs_bot)
                            ).vertical()                            ,
                        ).split_point(0.10),
                        Split::rows(
                            big_text("PARSE").background(Color::rgb(255.0, 255.0, 255.0)),
                            Scroll::new(
                                List::new(|| Label::dynamic(|data: &String, _| {
                                        format!("{data}")
                                })).lens(TraderUi::parse_logs_bot)
                            ).vertical(),
                        ).split_point(0.10),
                    ),
                    Split::rows(
                        big_text("SOL").background(Color::rgb(255.0, 255.0, 255.0)),
                        Scroll::new(
                            List::new(|| Label::dynamic(|data: &String, _| {
                                    format!("{data}")
                            })).lens(TraderUi::sol_logs_bot)
                        ).vertical(),
                    ).split_point(0.10),
                ).split_point(0.66).border(Color::WHITE, 1.0)
            ),
            // the user side is the second view
            false => Box::new(
                Split::columns(
                    Split::columns(
                        Split::rows(
                            big_text("BFB").background(Color::rgb(255.0, 255.0, 255.0)),
                            Scroll::new(
                                List::new(|| Label::dynamic(|data: &String, _| {
                                        format!("{data}")
                                })).lens(TraderUi::bfb_logs_arb)
                            ).vertical()                            ,
                        ).split_point(0.10),
                        Split::rows(
                            big_text("PARSE").background(Color::rgb(255.0, 255.0, 255.0)),
                            Scroll::new(
                                List::new(|| Label::dynamic(|data: &String, _| {
                                        format!("{data}")
                                })).lens(TraderUi::parse_logs_arb)
                            ).vertical(),
                        ).split_point(0.10),
                    ),
                    Split::rows(
                        big_text("SOL").background(Color::rgb(255.0, 255.0, 255.0)),
                        Scroll::new(
                            List::new(|| Label::dynamic(|data: &String, _| {
                                    format!("{data}")
                            })).lens(TraderUi::sol_logs_arb)
                        ).vertical(),
                    ).split_point(0.10),
                ).split_point(0.66).border(Color::WHITE, 1.0)
            ),
            // if the selector is not 0 or 1, the application shows an error
        },
    );
    view_switcher
}


fn build_safe_mode() -> impl Widget<TraderUi> {
    Split::columns(
        Split::columns(
            Split::rows(
                big_text("BFB").background(Color::rgb(255.0, 255.0, 255.0)),
                Scroll::new(
                    List::new(|| Label::dynamic(|data, _| format!("{data}")))
                        .lens(TraderUi::bfb_logs_bot)
                ).vertical(),
            ).split_point(0.10),
            Split::rows(
                big_text("PARSE").background(Color::rgb(255.0, 255.0, 255.0)),
                Scroll::new(
                    List::new(|| Label::dynamic(|data, _| format!("{data}")))
                        .lens(TraderUi::parse_logs_bot)
                ).vertical(),
            ).split_point(0.10),
        ),
        Split::rows(
            big_text("SOL").background(Color::rgb(255.0, 255.0, 255.0)),
            Scroll::new(
                List::new(|| Label::dynamic(|data, _| format!("{data}")))
                    .lens(TraderUi::sol_logs_bot)
            ).vertical(),
        ).split_point(0.10),
    ).split_point(0.66).border(Color::WHITE, 1.0)
}

fn build_unsafe_mode() -> impl Widget<TraderUi> {
    Split::columns(
        Split::columns(
            Split::rows(
                big_text("BFB").background(Color::rgb(255.0, 255.0, 255.0)),
                Scroll::new(
                    List::new(|| Label::dynamic(|data, _| format!("{data}")))
                        .lens(TraderUi::bfb_logs_bot)
                ).vertical(),
            ).split_point(0.10),
            Split::rows(
                big_text("PARSE").background(Color::rgb(255.0, 255.0, 255.0)),
                Scroll::new(
                    List::new(|| Label::dynamic(|data, _| format!("{data}")))
                        .lens(TraderUi::parse_logs_bot)
                ).vertical(),
            ).split_point(0.10),
        ),
        Split::rows(
            big_text("SOL").background(Color::rgb(255.0, 255.0, 255.0)),
            Scroll::new(
                List::new(|| Label::dynamic(|data, _| format!("{data}")))
                    .lens(TraderUi::sol_logs_bot)
            ).vertical(),
        ).split_point(0.10),
    ).split_point(0.66).border(Color::WHITE, 1.0)
}