// library dependencies
use druid::{Color, theme, Widget, WidgetExt};
use druid::im::Vector;
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, ProgressBar, Slider, Split, TextBox};

// local dependencies
use crate::TraderUi;
use crate::visualizers::datas::Trader;

/// This function builds the widget that will be displayed
/// on the user side of the application.
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

fn create_chart_trader() -> impl Widget<TraderUi> {

    // trader header
    let label_trader = Label::new("Trader".to_string())
        .with_text_color(theme::PRIMARY_LIGHT)
        .with_text_size(35.0)
        .padding(5.0);

    let label_trader_eur = Flex::column()
        .with_child(Label::new("EUR".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(20.0)
            .center())
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.money;
            format!("{money}")
        }).lens(TraderUi::trader).center())
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_yen = Flex::column()
        .with_child(Label::new("YEN".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(20.0))
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.goods[0];
            format!("{money}")
        }).lens(TraderUi::trader))
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_usd = Flex::column()
        .with_child(Label::new("USD".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(20.0))
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.goods[1];
            format!("{money}")
        }).lens(TraderUi::trader))
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    let label_trader_yuan = Flex::column()
        .with_child(Label::new("YUAN".to_string())
            .with_text_color(theme::PRIMARY_LIGHT)
            .with_text_size(20.0))
        .with_child(Label::dynamic(|data: &Trader, _| {
            let money = data.goods[2];
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
        .padding(5.0)
        .center();

    // market buttons
    let button_bfb = Button::new("BFB")
        .on_click(|ctx, data: &mut TraderUi, _| {
            data.current_trade = 0;
            println!("current trade: BFB");
        })
        .border(({
            let mut a = Color::from_hex_str("#ffffff").unwrap();
            if true {
                a = Color::from_hex_str("#5cc4ff").unwrap();
            }
            a
        }), 2.0)
        .padding(5.0)
        .center();

    let button_sol = Button::new("SOL")
        .on_click(|ctx, data: &mut TraderUi, _| {
            data.current_trade = 1;
            println!("current trade: SOL");
        })
        .border(({
            let mut a = Color::from_hex_str("#ffffff").unwrap();
            if true {
                a = Color::from_hex_str("#5cc4ff").unwrap();
            }
            a
        }), 2.0)
        .padding(5.0)
        .center();

    let button_parse = Button::new("PARSE")
        .on_click(|ctx, data: &mut TraderUi, _| {
            data.current_trade = 2;
            println!("current trade: PARSE");
        })
        .border(({
            let mut a = Color::from_hex_str("#ffffff").unwrap();
            if true {
                a = Color::from_hex_str("#5cc4ff").unwrap();
            }
            a
        }), 2.0)
        .padding(5.0)
        .center();

    let flex_buttons_markets = Flex::row()
        .with_child(button_bfb)
        .with_spacer(40.0)
        .with_child(button_sol)
        .with_spacer(40.0)
        .with_child(button_parse)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(5.0)
        .center();

    // quantity textbox
    let textbox = TextBox::new().lens(TraderUi::quantity_str);

    // quantity slider
    let slider = Flex::column()
        .with_child(
            Slider::new()
                .with_range(0.0, 1.0)
                .with_step(0.01)
                .lens(TraderUi::quantity),
        )
        .with_spacer(4.0);

    // quantity progressbar
    let progressbar = Flex::column()
        .with_child(ProgressBar::new().lens(TraderUi::quantity))
        .with_spacer(4.0)
        .with_child(Label::new(|data: &TraderUi, _: &_| {
            format!("{:.1}", data.quantity * 100.0)
            // format!("{:.1}%", data.quantity * 100.0)
        }))
        .with_spacer(4.0)
        .with_child(
            Flex::row()
                .with_child(Button::new("<<").on_click(|_, data: &mut TraderUi, _| {
                    data.quantity = (data.quantity - 0.05).max(0.0);
                }))
                .with_spacer(4.0)
                .with_child(Button::new(">>").on_click(|_, data: &mut TraderUi, _| {
                    data.quantity = (data.quantity + 0.05).min(1.0);
                })),
        );

    // trade buttons
    let button_buy = Button::new("BUY")
        .on_click(|ctx, data: &mut TraderUi, _| {
            // TODO
        })
        .border(Color::from_hex_str("#ffffff").unwrap(), 2.0)
        .padding(5.0);

    let button_sell = Button::new("SELL")
        .on_click(|ctx, data: &mut TraderUi, _| {
            // TODO
        })
        .border(Color::from_hex_str("#ffffff").unwrap(), 2.0)
        .padding(5.0);

    let flex_buttons_trades = Flex::row()
        .with_child(button_buy)
        .with_spacer(40.0)
        .with_child(button_sell)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(5.0)
        .center();

    // trader central panel
    let centralpanel = Flex::column()
        .with_child(flex_buttons_markets)
        .with_spacer(20.0)
        .with_child(slider.center())
        .with_spacer(20.0)
        .with_child(textbox.center())
        .with_spacer(20.0)
        .with_child(progressbar.center())
        .with_spacer(20.0)
        .with_child(flex_buttons_trades.center())
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(5.0)
        .center();

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
