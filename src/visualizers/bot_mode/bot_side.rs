// libraries dependencies
use druid::{Widget, WidgetExt, Color};
use druid::widget::{Button};
use druid::widget::Split;

// local dependencies
use crate::TraderUi;
use crate::visualizers::bot_mode::support_functions::*;

/// This function builds the widget that will be displayed
/// on the bots side of the application.
pub(crate) fn bot_side() -> impl Widget<TraderUi> {
    //declares the last label that will be displayed, used for let the user know which bot is running 

    Split::rows(
        Split::rows(
            switcher_header(),
        Split::columns(
            //PULSANTE PER ATTIVARE BOT SAFE MODE
            Button::new("safe mode")
                .on_click(|_ctx, data: &mut TraderUi, _env| {
                    data.safe_mode = true;
                })
                .expand_height()
                .expand_width()
                .center()
                .padding(10.0)
                .background(Color::rgb(0.0, 0.0, 255.0)),
            Button::new("unsafe mode (arbitrager)")
                .on_click(|_ctx, data: &mut TraderUi, _env| {
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

