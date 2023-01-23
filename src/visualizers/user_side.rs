// library dependencies
use druid::{Color, theme, Widget, WidgetExt};
use druid::im::Vector;
use druid::widget::{Align, Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, Split};

// local dependencies
use crate::TraderUi;

/// This function builds the widget that will be displayed
/// on the user side of the application.
pub(crate) fn user_side() -> impl Widget<TraderUi> {
    let chart_bfb = create_chart_bfb();
    let chart_sol = create_chart_sol();
    let chart_parse = create_chart_parse();

    let chart = Split::rows(
        Split::rows(chart_bfb, chart_sol).split_point(0.5),
        chart_parse).split_point(0.66);

    Split::columns(
        chart,
        Align::centered(Button::new("BFB").on_click(|_, data: &mut TraderUi, _| {
            data.bfb_exchange_rate_buy[0] = 999999999999.0;
        })),
    ).split_point(0.3)
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
