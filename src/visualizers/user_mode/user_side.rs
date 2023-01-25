// library dependencies
use druid::Widget;
use druid::widget::Split;

// local dependencies
use crate::TraderUi;
use crate::visualizers::user_mode::charts::{create_chart_bfb, create_chart_parse, create_chart_sol};
use crate::visualizers::user_mode::trader_ui::create_chart_trader;

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

