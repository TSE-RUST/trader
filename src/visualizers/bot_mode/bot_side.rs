// libraries dependencies
use druid::{Widget, WidgetExt, Color, lens};
use druid::widget::{Label, Button, Container, Flex, Slider, List, LensWrap, Scroll, ViewSwitcher};
use druid::widget::Split;

// market dependencies
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;
use bfb::bfb_market::Bfb as bfb;
use druid::kurbo::Shape;
use market_sol::SOLMarket as sol;
use parse_market::ParseMarket as parse;

// local dependencies
use crate::TraderUi;
use crate::visualizers::bot_mode::support_functions::*;

/// This function builds the widget that will be displayed
/// on the bots side of the application.
pub(crate) fn bot_side() -> impl Widget<TraderUi> {
    //declares the last label that will be displayed, used for let the user know which bot is running 

    Split::rows(
        Split::rows(
            trader_quantities(),
        Split::columns(
            //PULSANTE PER ATTIVARE BOT SAFE MODE
            Button::new("safe mode")
                .on_click(|ctx, data: &mut TraderUi, _env| {
                    data.safe_mode = true;
                })
                .expand_height()
                .expand_width()
                .center()
                .padding(10.0)
                .background(Color::rgb(0.0, 0.0, 255.0)),
            Button::new("unsafe mode (arbitrager)")
                .on_click(|ctx, data: &mut TraderUi, _env| {
                    data.safe_mode = false;
                })
                .expand_height()
                .expand_width()
                .center()
                .padding(10.0)
                .background(Color::rgb(255.0, 0.0, 0.0)),
        )).split_point(0.6),
        view_switcher(),


    ).split_point(0.25)
}

