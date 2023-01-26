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


    let label = Label::dynamic(move |data: &TraderUi, _| {
        if data.safe_mode {
            format!("safe mode attivo")
        } else {
            format!("safe mode disattivo(!) ")
        }
    })
        .with_text_color(Color::rgb(0.0, 0.0, 0.0))
        .background(Color::rgb(255.0, 227.0, 0.0))
        .center();

    Split::rows(
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
        ),
        Split::rows(
            Split::rows(
                view_switcher(),
                Split::columns(
                    Flex::row()
                        .main_axis_alignment(druid::widget::MainAxisAlignment::Center)
                        .with_child(
                            Slider::new()
                                .with_range(0.0, 1.0)
                                // .with_step(0.10)
                                .lens(TraderUi::percentage_bot)
                                .fix_width(700.0),
                        )
                        .with_spacer(8.0)
                        .with_child(Label::new(|data: &TraderUi, _: &_| {
                            format!("{:.2}", (data.percentage_bot * 100.0) as i32)
                        }).with_text_size(20.0))
                        .with_spacer(8.0)
                        .with_child(
                            Flex::row()
                                .with_child(Button::new("<<").on_click(|_, data: &mut TraderUi, _| {
                                    data.percentage_bot = (data.percentage_bot - 0.01);
                                }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 100.0 == 0.0))
                                .with_spacer(4.0)
                                .with_child(Button::new(">>").on_click(|_, data: &mut TraderUi, _| {
                                    data.percentage_bot = (data.percentage_bot + 0.01);
                                }).disabled_if(|data: &TraderUi, _: &_| data.percentage_bot * 100.0 == 100.0)),
                        ),
                    Button::new("ENTER").on_click(|ctx, data: &mut TraderUi, _env| {
                        println!("{}", string_log(data));
                    }),
                ).split_point(0.8),
            ).split_point(0.9),
            label.background(Color::rgb(255.0, 227.0, 0.0)),
        ).split_point(0.95),
    ).split_point(0.07)
}

// label.background(Color::rgb(255.0,227.0,0.0)