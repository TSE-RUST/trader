use std::cell::RefCell;
use std::rc::Rc;

use druid::{Widget, Color, WidgetExt};
use druid::im::Vector;
use druid::widget::{Label, Split, ViewSwitcher, Scroll, List, Button, Flex, Slider};
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

pub fn view_switcher()-> impl Widget<TraderUi> {

    let view_switcher = ViewSwitcher::new(
        |data: &TraderUi, _env| data.safe_mode,
        |selector, _data, _env| match _data.safe_mode {
            // the bots side is the first view
            true => Box::new(
                Split::rows(
                    Split::rows(
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
                        ).split_point(0.66).border(Color::WHITE, 1.0),
                        Split::columns(
                            Flex::row()
                                .main_axis_alignment(druid::widget::MainAxisAlignment::Center)
                                .with_child(
                                    Slider::new()
                                        .with_range(0.0, 1.0)
                                        // .with_step(0.10)
                                        .lens(TraderUi::percentage_bot)
                                        .fix_width(700.0),
                                )
                                .with_spacer(8.0)
                                .with_child(Label::new(|data: &TraderUi, _: &_| {
                                    format!("{:.2}", (data.percentage_bot * 1000.0) as i32)
                                }).with_text_size(20.0))
                                .with_spacer(8.0)
                                .with_child(
                                    Flex::row()
                                        .with_child(Button::new("<<").on_click(|_, data: &mut TraderUi, _| {
                                            data.percentage_bot = data.percentage_bot - 0.001;
                                        }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 1000.0 == 0.0))
                                        .with_spacer(4.0)
                                        .with_child(Button::new(">>").on_click(|_, data: &mut TraderUi, _| {
                                            data.percentage_bot = (data.percentage_bot + 0.001);
                                        }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 1000.0 == 1000.0)),
                                ),
                            Button::new("ENTER").on_click(|ctx, data: &mut TraderUi, _env| {
                                println!("oooo");
                            }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 1000.0 == 0.0),
                        ).split_point(0.8),
                    ).split_point(0.9),
                    Label::new("Safe mode").center().background(Color::rgb(0.0, 0.0, 255.0)),
                ).split_point(0.95)
            ),
            // the user side is the second view
            false => Box::new(
                Split::rows(
                    Split::rows(
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
                        ).split_point(0.66).border(Color::WHITE, 1.0),
                        Split::columns(
                            Flex::row()
                                .main_axis_alignment(druid::widget::MainAxisAlignment::Center)
                                .with_child(
                                    Slider::new()
                                        .with_range(0.0, 1.0)
                                        // .with_step(0.10)
                                        .lens(TraderUi::percentage_bot)
                                        .fix_width(700.0),
                                )
                                .with_spacer(8.0)
                                .with_child(Label::new(|data: &TraderUi, _: &_| {
                                    format!("{:.2}", (data.percentage_bot * 1000.0) as i32)
                                }).with_text_size(20.0))
                                .with_spacer(8.0)
                                .with_child(
                                    Flex::row()
                                        .with_child(Button::new("<<").on_click(|_, data: &mut TraderUi, _| {
                                            data.percentage_bot = (data.percentage_bot - 0.001);
                                        }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 1000.0 == 0.0))
                                        .with_spacer(4.0)
                                        .with_child(Button::new(">>").on_click(|_, data: &mut TraderUi, _| {
                                            data.percentage_bot = (data.percentage_bot + 0.001);
                                        }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 1000.0 == 1000.0)),
                                ),
                            Button::new("ENTER").on_click(|ctx, data: &mut TraderUi, _env| {
                                println!("oooo");
                            }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 1000.0 == 0.0),
                        ).split_point(0.8),
                    ).split_point(0.9),
                    Label::new("Unsafe mode").center().background(Color::rgb(255.0, 0.0, 0.0)),
                ).split_point(0.95)
            ),
            // if the selector is not 0 or 1, the application shows an error
        },
    );
    view_switcher
}


