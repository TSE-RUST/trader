// libraries dependencies
use druid::{Widget, WidgetExt};
use druid::widget::{Label};

// market dependencies
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;
use bfb::bfb_market::Bfb as bfb;
use market_sol::SOLMarket as sol;
use parse_market::ParseMarket as parse;

// local dependencies
use crate::TraderUi;

/// This function builds the widget that will be displayed
/// on the bots side of the application.
pub(crate) fn bot_side() -> impl Widget<TraderUi>{

    // creates a label
    Label::new("BOT MODE").center()

}