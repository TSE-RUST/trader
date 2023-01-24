// libraries dependencies
use druid::{Widget, WidgetExt, Color, lens, Env};
use druid::widget::{Label, Button, Container};
use druid::widget::{Split};

// market dependencies
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;
use bfb::bfb_market::Bfb as bfb;
use market_sol::SOLMarket as sol;
use parse_market::ParseMarket as parse;

// local dependencies
use crate::{TraderUi};

/// This function builds the widget that will be displayed
/// on the bots side of the application.
pub(crate) fn bot_side() -> impl Widget<TraderUi>{
    // creates a label

    let mut label=Label::dynamic(|data: &TraderUi, _| {
        if data.safe_mode {
            format!("safe mode")
        } else {
            format!("unsafe mode")
        }
    })
        .center()
        .padding(10.0)
        .background(Color::grey(0.1));
    Split::rows(
        Split::columns(
            //PULSANTE PER ATTIVARE BOT SAFE MODE
            Button::new("safe mode")
                .on_click(|ctx, data: &mut TraderUi, _env| {
                    let _ = data.safe_mode==true;
                    //todo
                })
                .fix_width(100.0)
                .fix_height(50.0)
                .center()
                .padding(10.0)
                .background(Color::rgb(0.0, 0.0, 255.0)),

                Button::new("unsafe mode")
                .on_click(|ctx, data: &mut TraderUi, _env| {
                    let _ = data.safe_mode==false;
                //todo
                })
                .fix_width(100.0)
                .fix_height(50.0)
                .center()
                .padding(10.0)
                .background(Color::rgb(255.0, 0.0, 0.0))
        ),
        label
    ).split_point(0.1)

}