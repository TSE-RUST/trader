// library dependencies
use druid::{Color, theme, Widget, WidgetExt};
use druid::im::Vector;
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment};

// local dependencies
use crate::TraderUi;

/// This function builds the bfb chart on the
/// right side of the application
///
/// **Federico Brancasi**
pub(crate) fn create_chart_bfb() -> impl Widget<TraderUi> {
    let label_name = Label::new("BFB".to_string())
        .with_text_color(theme::PRIMARY_LIGHT)
        .with_text_size(27.0);


    let name1 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[0];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::bfb_kinds);

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
        .with_spacer(6.0)
        .with_child(name1)
        .with_child(quantity1)
        .with_child(buy1)
        .with_child(sell1)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let name2 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[1];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::bfb_kinds);

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
        .with_spacer(6.0)
        .with_child(name2)
        .with_child(quantity2)
        .with_child(buy2)
        .with_child(sell2)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let name3 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[2];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::bfb_kinds);

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
        .with_spacer(6.0)
        .with_child(name3)
        .with_child(quantity3)
        .with_child(buy3)
        .with_child(sell3)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let name4 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[3];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::bfb_kinds);

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
        .with_spacer(6.0)
        .with_child(name4)
        .with_child(quantity4)
        .with_child(buy4)
        .with_child(sell4)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let flex_row1 = Flex::row()
        .with_child(chart1)
        .with_spacer(7.0)
        .with_child(chart2);

    let flex_row2 = Flex::row()
        .with_child(chart3)
        .with_spacer(7.0)
        .with_child(chart4);

    let flex_column = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(label_name)
        .with_spacer(5.0)
        .with_child(flex_row1)
        .with_spacer(7.0)
        .with_child(flex_row2)
        .with_spacer(7.0);

    flex_column
}

/// This function builds the sol chart on the
/// right side of the application
///
/// **Federico Brancasi**
pub(crate) fn create_chart_sol() -> impl Widget<TraderUi> {
    let label_name = Label::new("SOL".to_string())
        .with_text_color(theme::PRIMARY_LIGHT)
        .with_text_size(27.0);


    let name1 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[0];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::sol_kinds);

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
        .with_spacer(6.0)
        .with_child(name1)
        .with_child(quantity1)
        .with_child(buy1)
        .with_child(sell1)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let name2 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[1];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::sol_kinds);

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
        .with_spacer(6.0)
        .with_child(name2)
        .with_child(quantity2)
        .with_child(buy2)
        .with_child(sell2)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let name3 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[2];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::sol_kinds);

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
        .with_spacer(6.0)
        .with_child(name3)
        .with_child(quantity3)
        .with_child(buy3)
        .with_child(sell3)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let name4 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[3];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::sol_kinds);

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
        .with_spacer(6.0)
        .with_child(name4)
        .with_child(quantity4)
        .with_child(buy4)
        .with_child(sell4)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let flex_row1 = Flex::row()
        .with_child(chart1)
        .with_spacer(7.0)
        .with_child(chart2);

    let flex_row2 = Flex::row()
        .with_child(chart3)
        .with_spacer(7.0)
        .with_child(chart4);

    let flex_column = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacer(30.0)
        .with_child(label_name)
        .with_spacer(5.0)
        .with_child(flex_row1)
        .with_spacer(7.0)
        .with_child(flex_row2)
        .with_spacer(7.0);

    flex_column
}

/// This function builds the parse chart on the
/// right side of the application
///
/// **Federico Brancasi**
pub(crate) fn create_chart_parse() -> impl Widget<TraderUi> {
    let label_name = Label::new("PARSE".to_string())
        .with_text_color(theme::PRIMARY_LIGHT)
        .with_text_size(27.0);


    let name1 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[0];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::parse_kinds);

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
        .with_spacer(6.0)
        .with_child(name1)
        .with_child(quantity1)
        .with_child(buy1)
        .with_child(sell1)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let name2 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[1];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::parse_kinds);

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
        .with_spacer(6.0)
        .with_child(name2)
        .with_child(quantity2)
        .with_child(buy2)
        .with_child(sell2)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let name3 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[2];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::parse_kinds);

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
        .with_spacer(6.0)
        .with_child(name3)
        .with_child(quantity3)
        .with_child(buy3)
        .with_child(sell3)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let name4 = Label::dynamic(|data: &Vector<String>, _| {
        let name = &data[3];
        format!("{name}")
    }).with_text_color(Color::rgb8(176, 196, 222))
        .with_text_size(18.0)
        .lens(TraderUi::parse_kinds);

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
        .with_spacer(6.0)
        .with_child(name4)
        .with_child(quantity4)
        .with_child(buy4)
        .with_child(sell4)
        .with_spacer(6.0)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(190.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);


    let flex_row1 = Flex::row()
        .with_child(chart1)
        .with_spacer(7.0)
        .with_child(chart2);

    let flex_row2 = Flex::row()
        .with_child(chart3)
        .with_spacer(7.0)
        .with_child(chart4);

    let flex_column = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(label_name)
        .with_spacer(5.0)
        .with_child(flex_row1)
        .with_spacer(7.0)
        .with_child(flex_row2)
        .with_spacer(7.0);

    flex_column
}
