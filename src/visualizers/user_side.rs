use std::cell::RefCell;
use std::rc::Rc;
// library dependencies
use druid::{Color, theme, Widget, WidgetExt};
use druid::im::Vector;
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, ProgressBar, Slider, Split};
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;

// local dependencies
use crate::TraderUi;
use crate::visualizers::datas::Trader;
use crate::visualizers::custom_widget::{custom_button, custom_button_white};
use crate::visualizers::datas::trader_ui_derived_lenses::{bfb_quantities, parse_quantities, sol_quantities};

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

fn get_index_bfb(string: &String) -> usize {
    if string == "EUR" {
        3
    } else if string == "YEN" {
        1
    } else if string == "USD" {
        0
    } else {
        2
    }
}

fn get_index_sol(string: &String) -> usize {
    if string == "EUR" {
        1
    } else if string == "YEN" {
        2
    } else if string == "USD" {
        3
    } else {
        0
    }
}

fn get_index_parse(string: &String) -> usize {
    if string == "EUR" {
        3
    } else if string == "YEN" {
        1
    } else if string == "USD" {
        0
    } else {
        2
    }
}


fn max_qt(markets: &Vector<Rc<RefCell<dyn Market>>>, goods_trader: &Vector<f32>, method: &String, market: &String, good: &String, bfb: &Vector<f32>, sol: &Vector<f32>, parse: &Vector<f32>) -> f32 {
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
                bfb[get_index_bfb(&good)],
                goodkind,
            );
        } else if market == "SOL" {
            ret = crate::bots::bot::get_max_buy_quantity(
                &markets[1],
                sol[get_index_sol(&good)],
                goodkind,
            );
        } else if market == "PARSE" {
            ret = crate::bots::bot::get_max_buy_quantity(
                &markets[2],
                parse[get_index_parse(&good)],
                goodkind,
            );
        }
    }
    ret
}

/// This function builds the widget that will be displayed
/// on the user side of the application.
///
/// **Federico Brancasi**
pub(crate) fn user_side() -> impl Widget<TraderUi> {

    // creates the chart for each market
    let chart_bfb = create_chart_bfb();
    let chart_sol = create_chart_sol();
    let chart_parse = create_chart_parse();

    // union of the charts (left side of the application)
    let chart = Split::rows(
        Split::rows(chart_bfb, chart_sol).split_point(0.5),
        chart_parse).split_point(0.66);

    // creates the right side of the application
    let trader_ui = create_chart_trader();

    // union of the left and right side of the application
    Split::columns(
        chart,
        trader_ui,
    ).split_point(0.3)
}

/// This function builds the left side of the application.
///
/// **Federico Brancasi**
fn create_chart_trader() -> impl Widget<TraderUi> {

    // trader header
    let label_trader = Label::new("Tokyo Stock Exchange Trader".to_string())
        .with_text_color(theme::PRIMARY_LIGHT)
        .with_text_size(35.0);
    // .padding(5.0);

    let label_trader_eur = Flex::column()
        .with_child(Label::new("EUR".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(20.0)
            .center())
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.goods[0];
            format!("{money}")
        }).lens(TraderUi::trader).center())
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_yen = Flex::column()
        .with_child(Label::new("YEN".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(20.0))
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.goods[1];
            format!("{money}")
        }).lens(TraderUi::trader))
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_usd = Flex::column()
        .with_child(Label::new("USD".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(20.0))
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.goods[2];
            format!("{money}")
        }).lens(TraderUi::trader))
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_yuan = Flex::column()
        .with_child(Label::new("YUAN".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(20.0))
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.goods[3];
            format!("{money}")
        }).lens(TraderUi::trader))
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
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(25.0);
    // .center();

    // market buttons
    let button_bfb = custom_button("BFB")
        .on_click(|_ctx, data: &mut TraderUi, _| {
            data.selected_market = "BFB".to_string();
            println!("BFB button clicked");
            data.quantity = max_qt(&data.markets,
                                   &data.trader.goods,
                                   &data.selected_method_of_trade,
                                   &data.selected_market,
                                   &data.selected_good,
                                   &data.bfb_quantities.clone(),
                                   &data.sol_quantities.clone(),
                                   &data.parse_quantities.clone());
            println!("max quantity: {}", data.quantity);
        });

    let bfb_flex = Flex::column()
        .with_child(button_bfb)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            if data.selected_market == "BFB" {
                format!("selected")
            } else {
                format!("")
            }
        }).with_text_color(Color::from_hex_str("#ffffff").unwrap()));

    let button_sol = custom_button("SOL")
        .on_click(|_ctx, data: &mut TraderUi, _| {
            data.selected_market = "SOL".to_string();
            println!("SOL button clicked");
        });

    let sol_flex = Flex::column()
        .with_child(button_sol)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            if data.selected_market == "SOL" {
                format!("selected")
            } else {
                format!("")
            }
        }).with_text_color(Color::from_hex_str("#ffffff").unwrap()));

    let button_parse = custom_button("PARSE")
        .on_click(|_ctx, data: &mut TraderUi, _| {
            data.selected_market = "PARSE".to_string();
            println!("PARSE button clicked");
        });

    let parse_flex = Flex::column()
        .with_child(button_parse)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            if data.selected_market == "PARSE" {
                format!("selected")
            } else {
                format!("")
            }
        }).with_text_color(Color::from_hex_str("#ffffff").unwrap()));

    let flex_buttons_markets = Flex::row()
        .with_child(bfb_flex)
        .with_spacer(40.0)
        .with_child(sol_flex)
        .with_spacer(40.0)
        .with_child(parse_flex)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(5.0);
    // .center();

    // good buttons
    let button_eur = custom_button("EUR")
        .on_click(|_ctx, data: &mut TraderUi, _| {
            data.selected_good = "EUR".to_string();
            println!("EUR button clicked");
        });

    let eur_flex = Flex::column()
        .with_child(button_eur)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            if data.selected_good == "EUR" {
                format!("selected")
            } else {
                format!("")
            }
        }).with_text_color(Color::from_hex_str("#ffffff").unwrap()));

    let button_yen = custom_button("YEN")
        .on_click(|_ctx, data: &mut TraderUi, _| {
            data.selected_good = "YEN".to_string();
            println!("YEN button clicked");
        });

    let yen_flex = Flex::column()
        .with_child(button_yen)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            if data.selected_good == "YEN" {
                format!("selected")
            } else {
                format!("")
            }
        }).with_text_color(Color::from_hex_str("#ffffff").unwrap()));

    let button_usd = custom_button("USD")
        .on_click(|_ctx, data: &mut TraderUi, _| {
            data.selected_good = "USD".to_string();
            println!("USD button clicked")
        });

    let usd_flex = Flex::column()
        .with_child(button_usd)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            if data.selected_good == "USD" {
                format!("selected")
            } else {
                format!("")
            }
        }).with_text_color(Color::from_hex_str("#ffffff").unwrap()));

    let button_yuan = custom_button("YUAN")
        .on_click(|_ctx, data: &mut TraderUi, _| {
            data.selected_good = "YUAN".to_string();
            println!("YUAN button clicked");
        });

    let yuan_flex = Flex::column()
        .with_child(button_yuan)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            if data.selected_good == "YUAN" {
                format!("selected")
            } else {
                format!("")
            }
        }).with_text_color(Color::from_hex_str("#ffffff").unwrap()));

    let flex_buttons_goods = Flex::row()
        .with_child(eur_flex)
        .with_spacer(40.0)
        .with_child(yen_flex)
        .with_spacer(40.0)
        .with_child(usd_flex)
        .with_spacer(40.0)
        .with_child(yuan_flex)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(5.0);
    // .center();

    // quantity textbox
    // let textbox = TextBox::new().lens(TraderUi::quantity_str);

    // quantity slider
    let slider = Flex::row()
        .with_child(
            Slider::new()
                .with_range(0.0, 1.0)
                // .with_step(0.10)
                .lens(TraderUi::percentage)
                .fix_width(180.0),
        )
        .with_spacer(4.0)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            format!("{:.2}", data.percentage * data.quantity as f64)
        }))
        .with_spacer(4.0)
        .with_child(
            Flex::row()
                .with_child(Button::new("<<").on_click(|_, data: &mut TraderUi, _| {
                    data.percentage = (data.percentage - 0.005).max(0.0);
                }))
                .with_spacer(4.0)
                .with_child(Button::new(">>").on_click(|_, data: &mut TraderUi, _| {
                    data.percentage = (data.percentage + 0.005).min(1.0);
                })),
        );

    // quantity progressbar
    let progressbar = Flex::column()
        .with_child(ProgressBar::new().lens(TraderUi::percentage).fix_width(380.0))
        .with_spacer(4.0);

    // trade buttons
    let button_buy = custom_button("BUY")
        .on_click(|_ctx, data: &mut TraderUi, _| {
            data.selected_method_of_trade = "BUY".to_string();
            println!("BUY button clicked")
        });

    let buy_flex = Flex::column()
        .with_child(button_buy)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            if data.selected_method_of_trade == "BUY" {
                format!("selected")
            } else {
                format!("")
            }
        }).with_text_color(Color::from_hex_str("#ffffff").unwrap()));

    let button_sell = custom_button("SELL")
        .on_click(|_ctx, data: &mut TraderUi, _| {
            data.selected_method_of_trade = "SELL".to_string();
            println!("SELL button clicked")
        });

    let sell_flex = Flex::column()
        .with_child(button_sell)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            if data.selected_method_of_trade == "SELL" {
                format!("selected")
            } else {
                format!("")
            }
        }).with_text_color(Color::from_hex_str("#ffffff").unwrap()));

    let flex_buttons_trades = Flex::row()
        .with_child(buy_flex)
        .with_spacer(40.0)
        .with_child(sell_flex)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(5.0);
    // .center();

    // trade button
    let trade_button = Flex::row()
        .with_child(custom_button_white("TRADE")
            .on_click(|_ctx, data: &mut TraderUi, _| {
                println!("TRADE button clicked");
                println!("selected percentage: {}", data.percentage);
                println!("quantity: {}", data.percentage * data.quantity as f64);
            })).align_right();

    //recap label
    let recap_label = Label::new(|data: &TraderUi, _: &_| {
        if data.selected_method_of_trade == "SELL" {
            format!("Sell {:.2} {} to {}", data.percentage * data.quantity as f64, data.selected_good, data.selected_market)
        } else {
            format!("Buy {:.2} {} from {}", data.percentage * data.quantity as f64, data.selected_good, data.selected_market)
        }
    }).with_text_size(28.0)
        .with_text_color(Color::from_hex_str("#a1dcff").unwrap());

    // trader central panel
    let centralpanel = Flex::column()
        .with_child(Label::new(" Choose a market".to_string())
            .with_text_color(Color::rgb8(176, 196, 222))
            .with_text_size(26.0))
        .with_child(flex_buttons_markets)
        .with_spacer(5.0)
        .with_child(Label::new(" Choose a good".to_string())
            .with_text_color(Color::rgb8(176, 196, 222))
            .with_text_size(26.0))
        .with_child(flex_buttons_goods)
        .with_spacer(5.0)
        .with_child(Label::new(" Choose how to trade".to_string())
            .with_text_color(Color::rgb8(176, 196, 222))
            .with_text_size(26.0))
        .with_child(flex_buttons_trades)
        .with_spacer(15.0)
        .with_child(slider)
        .with_spacer(20.0)
        // .with_child(textbox.center())
        // .with_spacer(20.0)
        .with_child(progressbar)
        .with_spacer(30.0)
        .with_child(recap_label)
        .with_child(trade_button)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(5.0)
        .padding(30.0);
    // .center();

    // trader bottom panel
    let bottompanel = Label::new("Consigli interessanti su cosa acquistare".to_string())
        .with_text_color(theme::PRIMARY_LIGHT)
        .with_text_size(35.0)
        .padding(5.0)
        .center();

    // union between central panel and bottom panel
    let centralpanel_bottompanel = Split::rows(
        centralpanel,
        bottompanel,
    ).split_point(0.75);

    // union header and central panel
    let trader_ui = Split::rows(
        header,
        centralpanel_bottompanel,
    ).split_point(0.1);

    trader_ui
}

/// This function builds the bfb chart on the
/// right side of the application
///
/// **Federico Brancasi**
fn create_chart_bfb() -> impl Widget<TraderUi> {
    let label_name = Label::new("BFB".to_string())
        .with_text_color(theme::PRIMARY_LIGHT)
        .with_text_size(25.0);
    // .with_text_color(Color::from_hex_str("#2d4ee0").unwrap());

    let name1 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[0];
        format!("quantity: {name}")
    }).lens(TraderUi::bfb_kinds);

    let quantity1 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[0];
        format!("quantity: {quantity}")
    }).lens(TraderUi::bfb_quantities);

    let buy1 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[0];
        format!("buy rate: {rate}")
    }).lens(TraderUi::bfb_exchange_rate_buy);

    let sell1 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[0];
        format!("sell rate: {rate}")
    }).lens(TraderUi::bfb_exchange_rate_sell);

    let chart1 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name1, 1.0)
        .with_flex_child(quantity1, 1.0)
        .with_flex_child(buy1, 1.0)
        .with_flex_child(sell1, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let name2 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[1];
        format!("quantity: {name}")
    }).lens(TraderUi::bfb_kinds);

    let quantity2 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[1];
        format!("quantity: {quantity}")
    }).lens(TraderUi::bfb_quantities);

    let buy2 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[1];
        format!("buy rate: {rate}")
    }).lens(TraderUi::bfb_exchange_rate_buy);

    let sell2 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[1];
        format!("sell rate: {rate}")
    }).lens(TraderUi::bfb_exchange_rate_sell);

    let chart2 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name2, 1.0)
        .with_flex_child(quantity2, 1.0)
        .with_flex_child(buy2, 1.0)
        .with_flex_child(sell2, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let name3 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[2];
        format!("quantity: {name}")
    }).lens(TraderUi::bfb_kinds);

    let quantity3 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[2];
        format!("quantity: {quantity}")
    }).lens(TraderUi::bfb_quantities);

    let buy3 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[2];
        format!("buy rate: {rate}")
    }).lens(TraderUi::bfb_exchange_rate_buy);

    let sell3 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[2];
        format!("sell rate: {rate}")
    }).lens(TraderUi::bfb_exchange_rate_sell);

    let chart3 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name3, 1.0)
        .with_flex_child(quantity3, 1.0)
        .with_flex_child(buy3, 1.0)
        .with_flex_child(sell3, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let name4 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[3];
        format!("quantity: {name}")
    }).lens(TraderUi::bfb_kinds);

    let quantity4 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[3];
        format!("quantity: {quantity}")
    }).lens(TraderUi::bfb_quantities);

    let buy4 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[3];
        format!("buy rate: {rate}")
    }).lens(TraderUi::bfb_exchange_rate_buy);

    let sell4 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[3];
        format!("sell rate: {rate}")
    }).lens(TraderUi::bfb_exchange_rate_sell);

    let chart4 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name4, 1.0)
        .with_flex_child(quantity4, 1.0)
        .with_flex_child(buy4, 1.0)
        .with_flex_child(sell4, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let flex_row1 = Flex::row()
        .with_flex_child(chart1, 1.0)
        .with_default_spacer()
        .with_flex_child(chart2, 1.0)
        .padding(2.0);

    let flex_row2 = Flex::row()
        .with_flex_child(chart3, 1.0)
        .with_default_spacer()
        .with_flex_child(chart4, 1.0)
        .padding(2.0);

    let flex_column = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(label_name, 1.0)
        .with_default_spacer()
        .with_flex_child(flex_row1, 1.0)
        .with_default_spacer()
        .with_flex_child(flex_row2, 1.0)
        .with_default_spacer()
        // .fix_width(300.0)
        .padding(10.0);

    flex_column
}

/// This function builds the sol chart on the
/// right side of the application
///
/// **Federico Brancasi**
fn create_chart_sol() -> impl Widget<TraderUi> {
    let label_name = Label::new("SOL".to_string())
        .with_text_color(theme::PRIMARY_LIGHT)
        .with_text_size(25.0);
    // .with_text_color(Color::from_hex_str("#2d4ee0").unwrap());

    let name1 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[0];
        format!("quantity: {name}")
    }).lens(TraderUi::sol_kinds);

    let quantity1 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[0];
        format!("quantity: {quantity}")
    }).lens(TraderUi::sol_quantities);

    let buy1 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[0];
        format!("buy rate: {rate}")
    }).lens(TraderUi::sol_exchange_rate_buy);

    let sell1 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[0];
        format!("sell rate: {rate}")
    }).lens(TraderUi::sol_exchange_rate_sell);

    let chart1 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name1, 1.0)
        .with_flex_child(quantity1, 1.0)
        .with_flex_child(buy1, 1.0)
        .with_flex_child(sell1, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let name2 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[1];
        format!("quantity: {name}")
    }).lens(TraderUi::sol_kinds);

    let quantity2 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[1];
        format!("quantity: {quantity}")
    }).lens(TraderUi::sol_quantities);

    let buy2 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[1];
        format!("buy rate: {rate}")
    }).lens(TraderUi::sol_exchange_rate_buy);

    let sell2 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[1];
        format!("sell rate: {rate}")
    }).lens(TraderUi::sol_exchange_rate_sell);

    let chart2 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name2, 1.0)
        .with_flex_child(quantity2, 1.0)
        .with_flex_child(buy2, 1.0)
        .with_flex_child(sell2, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let name3 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[2];
        format!("quantity: {name}")
    }).lens(TraderUi::sol_kinds);

    let quantity3 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[2];
        format!("quantity: {quantity}")
    }).lens(TraderUi::sol_quantities);

    let buy3 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[2];
        format!("buy rate: {rate}")
    }).lens(TraderUi::sol_exchange_rate_buy);

    let sell3 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[2];
        format!("sell rate: {rate}")
    }).lens(TraderUi::sol_exchange_rate_sell);

    let chart3 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name3, 1.0)
        .with_flex_child(quantity3, 1.0)
        .with_flex_child(buy3, 1.0)
        .with_flex_child(sell3, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let name4 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[3];
        format!("quantity: {name}")
    }).lens(TraderUi::sol_kinds);

    let quantity4 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[3];
        format!("quantity: {quantity}")
    }).lens(TraderUi::sol_quantities);

    let buy4 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[3];
        format!("buy rate: {rate}")
    }).lens(TraderUi::sol_exchange_rate_buy);

    let sell4 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[3];
        format!("sell rate: {rate}")
    }).lens(TraderUi::sol_exchange_rate_sell);

    let chart4 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name4, 1.0)
        .with_flex_child(quantity4, 1.0)
        .with_flex_child(buy4, 1.0)
        .with_flex_child(sell4, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let flex_row1 = Flex::row()
        .with_flex_child(chart1, 1.0)
        .with_default_spacer()
        .with_flex_child(chart2, 1.0)
        .padding(2.0);

    let flex_row2 = Flex::row()
        .with_flex_child(chart3, 1.0)
        .with_default_spacer()
        .with_flex_child(chart4, 1.0)
        .padding(2.0);

    let flex_column = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(label_name, 1.0)
        .with_default_spacer()
        .with_flex_child(flex_row1, 1.0)
        .with_default_spacer()
        .with_flex_child(flex_row2, 1.0)
        .with_default_spacer()
        // .fix_width(300.0)
        .padding(10.0);

    flex_column
}

/// This function builds the parse chart on the
/// right side of the application
///
/// **Federico Brancasi**
fn create_chart_parse() -> impl Widget<TraderUi> {
    let label_name = Label::new("PARSE".to_string())
        .with_text_color(theme::PRIMARY_LIGHT)
        .with_text_size(25.0);
    // .with_text_color(Color::from_hex_str("#2d4ee0").unwrap());

    let name1 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[0];
        format!("quantity: {name}")
    }).lens(TraderUi::parse_kinds);

    let quantity1 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[0];
        format!("quantity: {quantity}")
    }).lens(TraderUi::parse_quantities);

    let buy1 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[0];
        format!("buy rate: {rate}")
    }).lens(TraderUi::parse_exchange_rate_buy);

    let sell1 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[0];
        format!("sell rate: {rate}")
    }).lens(TraderUi::parse_exchange_rate_sell);

    let chart1 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name1, 1.0)
        .with_flex_child(quantity1, 1.0)
        .with_flex_child(buy1, 1.0)
        .with_flex_child(sell1, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let name2 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[1];
        format!("quantity: {name}")
    }).lens(TraderUi::parse_kinds);

    let quantity2 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[1];
        format!("quantity: {quantity}")
    }).lens(TraderUi::parse_quantities);

    let buy2 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[1];
        format!("buy rate: {rate}")
    }).lens(TraderUi::parse_exchange_rate_buy);

    let sell2 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[1];
        format!("sell rate: {rate}")
    }).lens(TraderUi::parse_exchange_rate_sell);

    let chart2 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name2, 1.0)
        .with_flex_child(quantity2, 1.0)
        .with_flex_child(buy2, 1.0)
        .with_flex_child(sell2, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let name3 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[2];
        format!("quantity: {name}")
    }).lens(TraderUi::parse_kinds);

    let quantity3 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[2];
        format!("quantity: {quantity}")
    }).lens(TraderUi::parse_quantities);

    let buy3 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[2];
        format!("buy rate: {rate}")
    }).lens(TraderUi::parse_exchange_rate_buy);

    let sell3 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[2];
        format!("sell rate: {rate}")
    }).lens(TraderUi::parse_exchange_rate_sell);

    let chart3 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name3, 1.0)
        .with_flex_child(quantity3, 1.0)
        .with_flex_child(buy3, 1.0)
        .with_flex_child(sell3, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let name4 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[3];
        format!("quantity: {name}")
    }).lens(TraderUi::parse_kinds);

    let quantity4 = Label::dynamic(|data: &Vector<f32>, _| {
        let quantity = data[3];
        format!("quantity: {quantity}")
    }).lens(TraderUi::parse_quantities);

    let buy4 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[3];
        format!("buy rate: {rate}")
    }).lens(TraderUi::parse_exchange_rate_buy);

    let sell4 = Label::dynamic(|data: &Vector<f32>, _| {
        let rate = data[3];
        format!("sell rate: {rate}")
    }).lens(TraderUi::parse_exchange_rate_sell);

    let chart4 = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(name4, 1.0)
        .with_flex_child(quantity4, 1.0)
        .with_flex_child(buy4, 1.0)
        .with_flex_child(sell4, 1.0)
        .fix_width(300.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let flex_row1 = Flex::row()
        .with_flex_child(chart1, 1.0)
        .with_default_spacer()
        .with_flex_child(chart2, 1.0)
        .padding(2.0);

    let flex_row2 = Flex::row()
        .with_flex_child(chart3, 1.0)
        .with_default_spacer()
        .with_flex_child(chart4, 1.0)
        .padding(2.0);

    let flex_column = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(label_name, 1.0)
        .with_default_spacer()
        .with_flex_child(flex_row1, 1.0)
        .with_default_spacer()
        .with_flex_child(flex_row2, 1.0)
        .with_default_spacer()
        // .fix_width(300.0)
        .padding(10.0);

    flex_column
}
