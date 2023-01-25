
// libraries dependencies
use druid::{Widget, WidgetExt, Color};
use druid::widget::{Label, Button, Container};
use druid::widget::Split;

// market dependencies
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;
use bfb::bfb_market::Bfb as bfb;
use market_sol::SOLMarket as sol;
use parse_market::ParseMarket as parse;

// local dependencies
use crate::TraderUi;
use crate::visualizers::bot_mode::data_bot::*;

/// This function builds the widget that will be displayed
/// on the bots side of the application.
pub(crate) fn bot_side() -> impl Widget<TraderUi>{
    //declares the last label that will be displayed, used for let the user know which bot is running 
    let label = Label::dynamic(move |data: &TraderUi, _| {
        if data.safe_mode{
            format!("safe mode attivo")
        }else{
            format!("safe mode disattivo(!) ")
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

                Button::new("unsafe mode (arbitrager)")
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
                         Split::rows(
                                Split::rows(
                                    Split::rows(
                                        big_text("BFB").background(Color::rgb(255.0, 255.0, 255.0))
                                        ,
                                        Label::dynamic(|data: &TraderUi, _| {
                                            format!("EUR: {}",get_quantity_market(GoodKind::EUR, data.bfb_quantities.clone()))
                                        }).center()
                                    ).split_point(0.5),
                                    Split::rows(
                                        Label::dynamic(|data: &TraderUi, _| {
                                            format!("YEN: {}",get_quantity_market(GoodKind::YEN, data.bfb_quantities.clone()))
                                        }).center(),
                                        Label::dynamic(|data: &TraderUi, _| {
                                            format!("YUAN: {}",get_quantity_market(GoodKind::YUAN, data.bfb_quantities.clone()))
                                        }).center()
                                    )),
                                Label::dynamic(|data: &TraderUi, _| {
                                    format!("USD: {}",get_quantity_market(GoodKind::YUAN, data.bfb_quantities.clone()))
                                }).center()
                        ).split_point(0.8)
                        ,Split::rows(
                            Split::rows(
                                Split::rows(
                                    big_text("PARSE").background(Color::rgb(255.0, 255.0, 255.0))
                                    ,
                                    Label::dynamic(|data: &TraderUi, _| {
                                        format!("EUR: {}",get_quantity_market(GoodKind::EUR, data.parse_quantities.clone()))
                                    }).center()
                                ).split_point(0.5),
                                Split::rows(
                                    Label::dynamic(|data: &TraderUi, _| {
                                        format!("YEN: {}",get_quantity_market(GoodKind::YEN, data.parse_quantities.clone()))
                                    }).center(),
                                    Label::dynamic(|data: &TraderUi, _| {
                                        format!("YUAN: {}",get_quantity_market(GoodKind::YUAN, data.parse_quantities.clone()))
                                    }).center()
                                )),
                            Label::dynamic(|data: &TraderUi, _| {
                                format!("USD: {}",get_quantity_market(GoodKind::USD, data.parse_quantities.clone()))
                            }).center()
                    ).split_point(0.8)
                    ),
                    Split::rows(
                        Split::rows(
                            Split::rows(
                                big_text("SOL").background(Color::rgb(255.0, 255.0, 255.0))
                                ,
                                Label::dynamic(|data: &TraderUi, _| {
                                    format!("EUR: {}",get_quantity_market(GoodKind::EUR, data.sol_quantities.clone()))
                                }).center()
                            ).split_point(0.5),
                            Split::rows(
                                Label::dynamic(|data: &TraderUi, _| {
                                    format!("YEN: {}",get_quantity_market(GoodKind::YEN, data.sol_quantities.clone()))
                                }).center(),
                                Label::dynamic(|data: &TraderUi, _| {
                                    format!("YUAN: {}",get_quantity_market(GoodKind::YUAN, data.sol_quantities.clone()))
                                }).center()
                            )),
                        Label::dynamic(|data: &TraderUi, _| {
                            format!("USD: {}",get_quantity_market(GoodKind::USD, data.sol_quantities.clone()))
                        }).center()
                ).split_point(0.8)
                ).split_point(0.66).border(Color::grey(0.9), 3.0),
                Split::rows(
                    Split::rows(
                        Split::columns(
                            Split::columns(
                                Button::new("Fai fare 1 trade al bot").on_click(|ctx, data: &mut TraderUi, _env| {
                                    println!("1 move da fare")
                                }),
                                Button::new("Fai fare 10 trade al bot").on_click(|ctx, data: &mut TraderUi, _env| {
                                    println!("10 move da fare")
                                }),
                            ),
                            Button::new("Fai fare 30 trade al bot").on_click(|ctx, data: &mut TraderUi, _env| {
                                println!("30 move da fare")
                            }),
                        ).split_point(0.66),
                        Label::dynamic(|data: &TraderUi, _| {
                            format!("Log del bot")
                        }).center()
                ).split_point(0.3),
                    label.background(Color::rgb(255.0,227.0,0.0))
                ).split_point(0.95)
                )
            )

    ).split_point(0.07)

}
