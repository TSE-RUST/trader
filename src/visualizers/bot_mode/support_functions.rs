// library dependencies
use druid::{Widget, Color, WidgetExt};
use druid::im::Vector;
use druid::widget::{Label, Split, ViewSwitcher, Scroll, List, Button, Flex, Slider, CrossAxisAlignment, MainAxisAlignment};

// local dependencies
use crate::bots::arbitrager_strategy::arbitrager::arbitrage;
use crate::bots::bot_strategy::bot::bot;
use crate::visualizers::datas::TraderUi;

pub fn big_text(text: &str) -> impl Widget<TraderUi> {
    Label::new(text)
        .with_text_size(20.0)
        .with_text_color(Color::rgb(0.0, 0.0, 0.0))
        .background(Color::rgb(255.0, 255.0, 255.0))
        .center()
}

pub fn switcher_header() -> impl Widget<TraderUi> {
    let switch = ViewSwitcher::new(
        |data: &TraderUi, _| data.safe_mode,
        |_selector, data, _| match data.safe_mode {
            true => Box::new(trader_quantities(Color::rgb(0.0, 0.0, 255.0), true)),
            false => Box::new(trader_quantities(Color::rgb(204.0, 0.0, 0.0), false)),
        },
    );
    switch
}

/// This function returns a widget that displays the quantities of the trader.
///
/// **Federico Brancasi**
pub fn trader_quantities(color: Color, safe: bool) -> impl Widget<TraderUi> {

    // trader header
    let label_trader = Flex::column()
        .with_spacer(8.0)
        .with_child(Label::new("Tokyo Stock Exchange Trader".to_string())
            .with_text_color(Color::rgb(255.0, 255.0, 255.0))
            .with_text_size(35.0));

    let label_trader_eur = Flex::column()
        .with_child(Label::new("EUR".to_string())
            .with_text_color(color)
            .with_text_size(26.0)
            .center())
        .with_child(Label::new("1000.0")
            .with_text_size(20.0).center())
        .with_child(Label::dynamic(move |data: &TraderUi, _| {
            let money = match safe {
                true => data.logs_bot.last(),
                false => data.logs_arb.last(),
            };
            if money.is_none() {
                return "0.0".to_string();
            } else {
                let money = money.unwrap();
                let goods = money.split(" ").collect::<Vector<&str>>();
                let string_money = format!("{}", goods[0]);
                if string_money == "0.0" {
                    format!("{:.0}", string_money)
                } else {
                    format!("{}", string_money)
                }
            }
        }).with_text_size(20.0).center())
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_yen = Flex::column()
        .with_child(Label::new("YEN".to_string())
            .with_text_color(color)
            .with_text_size(26.0))
        .with_child(Label::new("0.0")
            .with_text_size(20.0).center())
        .with_child(Label::dynamic(move |data: &TraderUi, _| {
            let money = match safe {
                true => data.logs_bot.last(),
                false => data.logs_arb.last(),
            };
            if money.is_none() {
                return "0.0".to_string();
            } else {
                let money = money.unwrap();
                let goods = money.split(" ").collect::<Vector<&str>>();
                let string_money = format!("{}", goods[1]);
                if string_money == "0.0" {
                    format!("{:.0}", string_money)
                } else {
                    format!("{}", string_money)
                }
            }
        }).with_text_size(20.0).center())
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_usd = Flex::column()
        .with_child(Label::new("USD".to_string())
            .with_text_color(color)
            .with_text_size(26.0))
        .with_child(Label::new("0.0")
            .with_text_size(20.0).center())
        .with_child(Label::dynamic(move |data: &TraderUi, _| {
            let money = match &safe {
                true => data.logs_bot.last(),
                false => data.logs_arb.last(),
            };
            if money.is_none() {
                return "0.0".to_string();
            } else {
                let money = money.unwrap();
                let goods = money.split(" ").collect::<Vector<&str>>();
                let string_money = format!("{}", goods[2]);
                if string_money == "0.0" {
                    format!("{:.0}", string_money)
                } else {
                    format!("{}", string_money)
                }
            }
        }).with_text_size(20.0).center())
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_yuan = Flex::column()
        .with_child(Label::new("YUAN".to_string())
            .with_text_color(color)
            .with_text_size(26.0))
        .with_child(Label::new("0.0")
            .with_text_size(20.0).center())
        .with_child(Label::dynamic(move |data: &TraderUi, _| {
            let money = match safe {
                true => data.logs_bot.last(),
                false => data.logs_arb.last(),
            };
            if money.is_none() {
                return "0.0".to_string();
            } else {
                let money = money.unwrap();
                let goods = money.split(" ").collect::<Vector<&str>>();
                let string_money = format!("{}", goods[3]);
                if string_money == "0.0" {
                    format!("{:.0}", string_money)
                } else {
                    format!("{}", string_money)
                }
            }
        }).with_text_size(20.0).center())
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);


    let header = Flex::row()
        .with_child(label_trader)
        .with_spacer(40.0)
        .with_child(label_trader_eur)
        .with_spacer(40.0)
        .with_child(label_trader_yen)
        .with_spacer(40.0)
        .with_child(label_trader_usd)
        .with_spacer(40.0)
        .with_child(label_trader_yuan)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .padding(25.0)
        .center();

    header
}


pub fn view_switcher() -> impl Widget<TraderUi> {
    let view_switcher = ViewSwitcher::new(
        |data: &TraderUi, _env| data.safe_mode,
        |_selector, _data, _env| match _data.safe_mode {
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
                                        })).lens(TraderUi::bfb_logs_bot).center()
                                    ).vertical(),
                                ).split_point(0.10),
                                Split::rows(
                                    big_text("PARSE").background(Color::rgb(255.0, 255.0, 255.0)),
                                    Scroll::new(
                                        List::new(|| Label::dynamic(|data: &String, _| {
                                            format!("{data}")
                                        })).lens(TraderUi::parse_logs_bot).center()
                                    ).vertical(),
                                ).split_point(0.10),
                            ),
                            Split::rows(
                                big_text("SOL").background(Color::rgb(255.0, 255.0, 255.0)),
                                Scroll::new(
                                    List::new(|| Label::dynamic(|data: &String, _| {
                                        format!("{data}")
                                    })).lens(TraderUi::sol_logs_bot).center()
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
                                    format!("{:.2}", (data.percentage_bot * 100.0) as i32)
                                }).with_text_size(20.0))
                                .with_spacer(8.0)
                                .with_child(
                                    Flex::row()
                                        .with_child(Button::new("<<").on_click(|_, data: &mut TraderUi, _| {
                                            data.percentage_bot = data.percentage_bot - 0.001;
                                        }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 1000.0 == 0.0))
                                        .with_spacer(4.0)
                                        .with_child(Button::new(">>").on_click(|_, data: &mut TraderUi, _| {
                                            data.percentage_bot = data.percentage_bot + 0.001;
                                        }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 1000.0 == 1000.0)),
                                ),
                            Button::new("ENTER").on_click(|_ctx, data: &mut TraderUi, _env| {
                                data.logs_bot = bot(&mut data.trader_bot, (data.percentage_bot * 100.0) as i32);
                                println!("start logs andrea\n");
                                for elem in data.logs_bot.iter() {
                                    println!("elem: {:?}", elem);
                                }
                                println!("\nlengt logs andrea: {}", data.logs_bot.len());
                                println!("end logs andrea\n");

                                for elem in data.logs_bot.iter() {
                                    if elem.as_str().contains("BFB") {
                                        data.bfb_logs_bot.push_front(elem.to_string());
                                    } else if elem.as_str().contains("SOL") {
                                        data.sol_logs_bot.push_front(elem.to_string());
                                    } else if elem.as_str().contains("PARSE") {
                                        data.parse_logs_bot.push_front(elem.to_string());
                                    } else {
                                        data.parse_logs_bot.push_back("".to_string());
                                    }
                                }
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
                                        })).lens(TraderUi::bfb_logs_arb).center()
                                    ).vertical(),
                                ).split_point(0.10),
                                Split::rows(
                                    big_text("PARSE").background(Color::rgb(255.0, 255.0, 255.0)),
                                    Scroll::new(
                                        List::new(|| Label::dynamic(|data: &String, _| {
                                            format!("{data}")
                                        })).lens(TraderUi::parse_logs_arb).center()
                                    ).vertical(),
                                ).split_point(0.10),
                            ),
                            Split::rows(
                                big_text("SOL").background(Color::rgb(255.0, 255.0, 255.0)),
                                Scroll::new(
                                    List::new(|| Label::dynamic(|data: &String, _| {
                                        format!("{data}")
                                    })).lens(TraderUi::sol_logs_arb).center()
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
                                    format!("{:.2}", (data.percentage_bot * 100.0) as i32)
                                }).with_text_size(20.0))
                                .with_spacer(8.0)
                                .with_child(
                                    Flex::row()
                                        .with_child(Button::new("<<").on_click(|_, data: &mut TraderUi, _| {
                                            data.percentage_bot = data.percentage_bot - 0.001;
                                        }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 1000.0 == 0.0))
                                        .with_spacer(4.0)
                                        .with_child(Button::new(">>").on_click(|_, data: &mut TraderUi, _| {
                                            data.percentage_bot = data.percentage_bot + 0.001;
                                        }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 1000.0 == 1000.0)),
                                ),
                            Button::new("ENTER").on_click(|_ctx, data: &mut TraderUi, _env| {
                                data.logs_arb = arbitrage(&mut data.trader_arb, (data.percentage_bot * 100.0) as i32);
                                println!("start logs lorenzo\n");
                                for elem in data.logs_arb.iter() {
                                    println!("elem: {:?}", elem);
                                }
                                println!("\nlengt logs lorenzo: {}", data.logs_arb.len());
                                println!("end logs lorenzo\n");

                                for elem in data.logs_arb.iter() {
                                    if elem.as_str().contains("BFB") {
                                        data.bfb_logs_arb.push_front(elem.to_string());
                                    } else if elem.as_str().contains("SOL") {
                                        data.sol_logs_arb.push_front(elem.to_string());
                                    } else if elem.as_str().contains("PARSE") {
                                        data.parse_logs_arb.push_front(elem.to_string());
                                    } else {
                                        data.parse_logs_arb.push_back("".to_string());
                                    }
                                }
                            }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot == 0.0),
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


