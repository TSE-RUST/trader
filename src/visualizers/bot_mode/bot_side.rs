use druid::piet::dwrite::TextLayout;
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

    let str="safe mode";
    let label = Label::dynamic(move |data: &TraderUi, _| {
        if data.safe_mode{
            format!("safe mode attivo")
        }else{
            format!("safe mode disattivo(!)")
        }
    })
    .with_text_color(Color::rgb(0.0,0.0,0.0))
    .background(Color::rgb(255.0,227.0,0.0))
    .center();
    Split::rows(
        Split::columns(
            //PULSANTE PER ATTIVARE BOT SAFE MODE
            Button::new("safe mode")
                .on_click(|ctx, data: &mut TraderUi, _env| {
                    data.safe_mode=true;
                })
                .expand_height()
                .expand_width()
                .center()
                .padding(10.0)
                .background(Color::rgb(0.0, 0.0, 255.0)),

                Button::new("unsafe mode")
                .on_click(|ctx, data: &mut TraderUi, _env| {
                     data.safe_mode=false;
                })
                .expand_height()
                .expand_width()
                .center()
                .padding(10.0)
                .background(Color::rgb(255.0, 0.0, 0.0))
        ),
        Container::new(
            Split::rows(
                Split::columns(
                    Split::columns(
                        Label::dynamic(|data: &TraderUi, _| {
                            format!("Yen")
                        }).center(),
                        Label::dynamic(|data: &TraderUi, _| {
                            format!("Eur")
                        }).center()
                    ),
                    Split::columns(
                        Label::dynamic(|data: &TraderUi, _| {
                            format!("Usd")
                        }).center(),
                        Label::dynamic(|data: &TraderUi, _| {
                            format!("Yuan")
                        }).center()
                    )
                ),
                Split::rows(
                    Label::dynamic(|data: &TraderUi, _| {
                        format!("da implementare")
                    }).center().center(),
                    label
                ).split_point(0.95)
                )
            )

    ).split_point(0.07)

}