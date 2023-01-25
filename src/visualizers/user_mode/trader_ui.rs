// library dependencies
use druid::{Color, theme, Widget, WidgetExt};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, ProgressBar, Slider, Split};

// local dependencies
use crate::TraderUi;
use crate::visualizers::datas::Trader;
use crate::visualizers::custom_widgets::{custom_button, custom_button_white};
use crate::visualizers::user_mode::support_functions::max_qt;

/// This function builds the left side of the application.
///
/// **Federico Brancasi**
pub(crate) fn create_chart_trader() -> impl Widget<TraderUi> {

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
            println!("USD button clicked");
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
            println!("BUY button clicked");
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
            println!("SELL button clicked");
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
        .with_child(sell_flex)
        .with_spacer(40.0)
        .with_child(buy_flex)
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

                if data.selected_method_of_trade == "BUY"{
                    //DO A BUY
                    todo!("buy")
                } else if data.selected_method_of_trade == "SELL"{
                    //DO A SELL
                    todo!("sell")
                }

                // now update all the labels etc
                todo!("update")

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