// library dependencies
use druid::{Color, theme, Widget, WidgetExt};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, ProgressBar, Slider, Split};

// market dependencies
use unitn_market_2022::good::good::Good;
use unitn_market_2022::good::good_kind::GoodKind;

// local dependencies
use crate::TraderUi;
use crate::visualizers::datas::Trader;
use crate::visualizers::custom_widgets::{custom_button, custom_button_white};
use crate::visualizers::user_mode::support_functions::{get_best_buy_trade, get_best_sell_trade, get_market_info, max_qt};

/// This function builds the left side of the application.
///
/// **Federico Brancasi**
pub(crate) fn create_chart_trader() -> impl Widget<TraderUi> {

    // trader header
    let label_trader = Flex::column()
        .with_spacer(8.0)
        .with_child(Label::new("Tokyo Stock Exchange Trader".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(35.0));

    let label_trader_eur = Flex::column()
        .with_child(Label::new("EUR".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(26.0)
            .center())
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.goods[0];
            // let string_money = format!("{:.1}", money);
            if money as i32 == 0 {
                format!("0")
            } else {
                format!("{:.2}", money)
            }
        }).with_text_size(20.0).lens(TraderUi::trader).center())
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_yen = Flex::column()
        .with_child(Label::new("YEN".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(26.0))
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.goods[1];
            let string_money = format!("{:.1}", money);
            if string_money == "0.0" {
                format!("0")
            } else {
                format!("{:.2}", money)
            }
        }).with_text_size(20.0).lens(TraderUi::trader).center())
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_usd = Flex::column()
        .with_child(Label::new("USD".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(26.0))
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.goods[2];
            let string_money = format!("{:.1}", money);
            if string_money == "0.0" {
                format!("0")
            } else {
                format!("{:.2}", money)
            }
        }).with_text_size(20.0).lens(TraderUi::trader).center())
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_yuan = Flex::column()
        .with_child(Label::new("YUAN".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(26.0))
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.goods[3];
            let string_money = format!("{:.1}", money);
            if string_money == "0.0" {
                format!("0")
            } else {
                format!("{:.2}", money)
            }
        }).with_text_size(20.0).lens(TraderUi::trader).center())
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
            data.string_best_profit_buy = get_best_buy_trade(&data.markets, data.trader.goods[0]);
            data.string_best_profit_sell = get_best_sell_trade(&data.markets, &data.trader.goods);
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
            data.string_best_profit_buy = get_best_buy_trade(&data.markets, data.trader.goods[0]);
            data.string_best_profit_sell = get_best_sell_trade(&data.markets, &data.trader.goods);
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
            data.string_best_profit_buy = get_best_buy_trade(&data.markets, data.trader.goods[0]);
            data.string_best_profit_sell = get_best_sell_trade(&data.markets, &data.trader.goods);
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
            data.string_best_profit_buy = get_best_buy_trade(&data.markets, data.trader.goods[0]);
            data.string_best_profit_sell = get_best_sell_trade(&data.markets, &data.trader.goods);
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
            data.string_best_profit_buy = get_best_buy_trade(&data.markets, data.trader.goods[0]);
            data.string_best_profit_sell = get_best_sell_trade(&data.markets, &data.trader.goods);
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
            data.string_best_profit_buy = get_best_buy_trade(&data.markets, data.trader.goods[0]);
            data.string_best_profit_sell = get_best_sell_trade(&data.markets, &data.trader.goods);
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
        .with_child(yen_flex)
        .with_spacer(40.0)
        .with_child(usd_flex)
        .with_spacer(40.0)
        .with_child(yuan_flex)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(5.0);

    // quantity slider
    let slider = Flex::row()
        .with_child(
            Slider::new()
                .with_range(0.0, 1.0)
                // .with_step(0.10)
                .lens(TraderUi::percentage)
                .fix_width(180.0)
                .disabled_if(|data: &TraderUi, _: &_| data.quantity == 0.0 ||
                    ((data.trader.goods[0] * 100.0) as i32 == 0) ||
                    (data.selected_method_of_trade == "BUY" && data.trader.goods[0] as i32 == 0)),
        )
        .with_spacer(8.0)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            let money = data.percentage * data.quantity as f64;
            let string_money = format!("{:.1}", money);
            if data.selected_method_of_trade == "BUY" && data.trader.goods[0] as i32 == 0 {
                format!("0")
            } else if string_money == "0.0" {
                format!("0")
            } else {
                format!("{:.2}", money)
            }
        }).with_text_size(20.0))
        .with_spacer(8.0)
        .with_child(
            Flex::row()
                .with_child(Button::new("<<").on_click(|_, data: &mut TraderUi, _| {
                    data.percentage = (data.percentage - 0.005).max(0.0);
                }).disabled_if(|data: &TraderUi, _: &_| data.quantity == 0.0 ||
                    ((data.trader.goods[0] * 100.0) as i32 == 0) ||
                    (data.selected_method_of_trade == "BUY" && data.trader.goods[0] as i32 == 0)))
                .with_spacer(4.0)
                .with_child(Button::new(">>").on_click(|_, data: &mut TraderUi, _| {
                    data.percentage = (data.percentage + 0.005).min(1.0);
                }).disabled_if(|data: &TraderUi, _: &_| data.quantity == 0.0 ||
                    ((data.trader.goods[0] * 100.0) as i32 == 0) ||
                    (data.selected_method_of_trade == "BUY" && data.trader.goods[0] as i32 == 0))),
        );

    // quantity progressbar
    let progressbar = Flex::column()
        .with_child(ProgressBar::new().lens(TraderUi::percentage_bot).fix_width(380.0))
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
            data.string_best_profit_buy = get_best_buy_trade(&data.markets, data.trader.goods[0]);
            data.string_best_profit_sell = get_best_sell_trade(&data.markets, &data.trader.goods);
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
            data.string_best_profit_buy = get_best_buy_trade(&data.markets, data.trader.goods[0]);
            data.string_best_profit_sell = get_best_sell_trade(&data.markets, &data.trader.goods);
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

    // trade button
    let trade_button = Flex::row()
        .with_child(custom_button_white("TRADE")
            .on_click(|_ctx, data: &mut TraderUi, _| {
                println!("TRADE button clicked");
                println!("selected percentage: {}", data.percentage);
                println!("quantity: {}", data.percentage * data.quantity as f64);

                if data.selected_method_of_trade == "BUY" {
                    //DO A BUY - Andrea Ballarini
                    let trader_name = data.trader.name.clone();
                    let selected_market =
                        match &data.selected_market[0..] {
                            "BFB" => 0,
                            "SOL" => 1,
                            "PARSE" => 2,
                            _ => 0,
                        };
                    let mut market = data.markets[selected_market].borrow_mut();
                    let market_name = market.get_name().clone();
                    let good = match &data.selected_good[0..] {
                        "EUR" => GoodKind::EUR,
                        "YEN" => GoodKind::YEN,
                        "USD" => GoodKind::USD,
                        "YUAN" => GoodKind::YUAN,
                        _ => GoodKind::EUR
                    };
                    let quantity = data.quantity * data.percentage as f32;
                    let price = match market.get_buy_price(good, quantity)
                    {
                        Ok(price) => price,
                        Err(e) => panic!("Error: in get_buy_price {:?}", e),
                    };
                    let mut cash = Good::new(GoodKind::EUR, price);
                    let token = match market.lock_buy(good, quantity, price, trader_name.clone()) {
                        Ok(token) => token,
                        Err(e) => { panic!("Error in lock_buy in {}: {:?}", market_name.to_string(), e); }
                    };
                    let increase = match market.buy(token, &mut cash) {
                        Ok(increase) => increase,
                        Err(e) => { panic!("Error in buy in {}: {:?}", market_name.to_string(), e); }
                    };
                    data.trader.goods[0] -= price;
                    data.trader.goods[
                        match good {
                            GoodKind::EUR => 0,
                            GoodKind::YEN => 1,
                            GoodKind::USD => 2,
                            GoodKind::YUAN => 3,
                        }
                        ] += increase.get_qty();
                    println!("buying {} {} from {}", data.percentage * data.quantity as f64, data.selected_good, data.selected_market);
                } else if data.selected_method_of_trade == "SELL" {
                    //DO A SELL - Andrea Ballarini
                    let trader_name = data.trader.name.clone();
                    let selected_market =
                        match &data.selected_market[0..] {
                            "BFB" => 0,
                            "SOL" => 1,
                            "PARSE" => 2,
                            _ => 0,
                        };
                    let mut market = data.markets[selected_market].borrow_mut();
                    let market_name = market.get_name().clone();
                    let good = match &data.selected_good[0..] {
                        "EUR" => GoodKind::EUR,
                        "YEN" => GoodKind::YEN,
                        "USD" => GoodKind::USD,
                        "YUAN" => GoodKind::YUAN,
                        _ => GoodKind::EUR
                    };
                    println!("data quantity: {}", data.quantity);
                    let quantity = data.quantity * data.percentage as f32;
                    let price = match market.get_sell_price(good, quantity)
                    {
                        Ok(price) => price,
                        Err(e) => panic!("Error: in get_sell_price {:?}", e),
                    };
                    let token = match market.lock_sell(good, quantity, price, trader_name) {
                        Ok(token) => token,
                        Err(e) => { panic!("Error in lock_sell in {}: {:?}", market_name.to_string(), e); }
                    };
                    let mut cash = Good::new(good, quantity);
                    let _decrease = match market.sell(token, &mut cash) {
                        Ok(decrease) => decrease,
                        Err(e) => { panic!("Error in sell in {}: {:?}", market_name.to_string(), e); }
                    };
                    data.trader.goods[0] += price;
                    println!("selling {} {} to {}", data.percentage * data.quantity as f64, data.selected_good, data.selected_market);

                    if data.selected_good == "EUR" {
                        if quantity >= data.trader.goods[0] {
                            data.trader.goods[0] = 0.0;
                        } else {
                            data.trader.goods[0] -= quantity;
                        }
                    } else if data.selected_good == "YEN" {
                        if quantity >= data.trader.goods[1] {
                            data.trader.goods[1] = 0.0;
                        } else {
                            data.trader.goods[1] -= quantity;
                        }
                    } else if data.selected_good == "USD" {
                        if quantity >= data.trader.goods[2] {
                            data.trader.goods[2] = 0.0;
                        } else {
                            data.trader.goods[2] -= quantity;
                        }
                    } else if data.selected_good == "YUAN" {
                        if quantity >= data.trader.goods[3] {
                            data.trader.goods[3] = 0.0;
                        } else {
                            data.trader.goods[3] -= quantity;
                        }
                    }
                }

                // set values for bfb market
                let (good_kinds_bfb, quantities_bfb, exchange_rate_buy_bfb, exchange_rate_sell_bfb) = get_market_info(&data.markets[0]);

                data.bfb_kinds = good_kinds_bfb;
                data.bfb_quantities = quantities_bfb;
                data.bfb_exchange_rate_buy = exchange_rate_buy_bfb;
                data.bfb_exchange_rate_sell = exchange_rate_sell_bfb;

                // set values for sol market
                let (good_kinds_sol, quantities_sol, exchange_rate_buy_sol, exchange_rate_sell_sol) = get_market_info(&data.markets[1]);

                data.sol_kinds = good_kinds_sol;
                data.sol_quantities = quantities_sol;
                data.sol_exchange_rate_buy = exchange_rate_buy_sol;
                data.sol_exchange_rate_sell = exchange_rate_sell_sol;

                // set values for parse market
                let (good_kinds_parse, quantities_parse, exchange_rate_buy_parse, exchange_rate_sell_parse) = get_market_info(&data.markets[2]);

                data.parse_kinds = good_kinds_parse;
                data.parse_quantities = quantities_parse;
                data.parse_exchange_rate_buy = exchange_rate_buy_parse;
                data.parse_exchange_rate_sell = exchange_rate_sell_parse;

                data.quantity = max_qt(&data.markets,
                                       &data.trader.goods,
                                       &data.selected_method_of_trade,
                                       &data.selected_market,
                                       &data.selected_good,
                                       &data.bfb_quantities.clone(),
                                       &data.sol_quantities.clone(),
                                       &data.parse_quantities.clone());

                data.string_best_profit_buy = get_best_buy_trade(&data.markets, data.trader.goods[0]);
                data.string_best_profit_sell = get_best_sell_trade(&data.markets, &data.trader.goods);
            }).disabled_if(|data: &TraderUi, _: &_| {
            let mut quantity_eur_market_zero = false;
            if data.selected_market == "BFB" {
                if data.bfb_quantities[0] as i32 == 0 {
                    quantity_eur_market_zero = true;
                }
            } else if data.selected_market == "SOL" {
                if data.sol_quantities[0] as i32 == 0 {
                    quantity_eur_market_zero = true;
                }
            } else if data.selected_market == "PARSE" {
                if data.parse_quantities[0] as i32 == 0 {
                    quantity_eur_market_zero = true;
                }
            }

            return ((data.quantity * 100.0) as i32) == 0 ||
                data.percentage == 0.0 ||
                (((data.quantity * 100.0) as i32) == 0 && data.percentage == 0.0) ||
                quantity_eur_market_zero ||
                (data.trader.goods[0] * 100.0) as i32 == 0 ||
                (data.selected_method_of_trade == "BUY" && data.trader.goods[0] as i32 == 0);
        }))
        .align_right();

    //recap label
    let recap_label = Label::new(|data: &TraderUi, _: &_| {
        let mut quantity_eur_market_zero = false;
        if data.selected_market == "BFB" {
            if data.bfb_quantities[0] as i32 == 0 {
                quantity_eur_market_zero = true;
            }
        } else if data.selected_market == "SOL" {
            if data.sol_quantities[0] as i32 == 0 {
                quantity_eur_market_zero = true;
            }
        } else if data.selected_market == "PARSE" {
            if data.parse_quantities[0] as i32 == 0 {
                quantity_eur_market_zero = true;
            }
        }

        if quantity_eur_market_zero {
            format!("The market has no money!")
        } else if (data.trader.goods[0] * 100.0) as i32 == 0 ||
            (data.selected_method_of_trade == "BUY" && data.trader.goods[0] as i32 == 0) {
            format!("The trader has no money!")
        } else if data.quantity == 0.0 {
            format!("The quantity of the good selected is zero!")
        } else if data.percentage == 0.0 {
            format!("The quantity of the good selected is zero!")
        } else if data.quantity == 0.0 && data.percentage == 0.0 {
            format!("The quantity of the good selected is zero!")
        } else {
            if data.selected_method_of_trade == "SELL" {
                format!("Sell {:.2} {} to {}", data.percentage * data.quantity as f64, data.selected_good, data.selected_market)
            } else {
                format!("Buy {:.2} {} from {}", data.percentage * data.quantity as f64, data.selected_good, data.selected_market)
            }
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

    // trader bottom panel
    let profit_sell_header = Label::new("best profit sell".to_string())
        .with_text_color(theme::PRIMARY_LIGHT)
        .with_text_size(30.0)
        .center();

    let profit_sell_desc = Label::dynamic(|data: &TraderUi, _| {
        format!("{}", data.string_best_profit_sell)
    }).with_text_size(20.0).center();

    let sell_bottompanel = Flex::column()
        .with_child(profit_sell_header)
        .with_child(profit_sell_desc)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center);

    let profit_buy_header = Label::new("best profit buy".to_string())
        .with_text_color(theme::PRIMARY_LIGHT)
        .with_text_size(30.0)
        .center();

    let profit_buy_desc = Label::dynamic(|data: &TraderUi, _| {
        format!("{}", data.string_best_profit_buy)
    }).with_text_size(20.0).center();

    let buy_bottompanel = Flex::column()
        .with_child(profit_buy_header)
        .with_child(profit_buy_desc)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center);

    let bottompanel = Flex::row()
        .with_child(sell_bottompanel)
        .with_spacer(140.0)
        .with_child(buy_bottompanel)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(30.0)
        .center();

    // union between central panel and bottom panel
    let centralpanel_bottompanel = Split::rows(
        centralpanel,
        bottompanel,
    ).split_point(0.80);

    // union header and central panel
    let trader_ui = Split::rows(
        header,
        centralpanel_bottompanel,
    ).split_point(0.13);

    trader_ui
}