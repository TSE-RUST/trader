// libraries dependencies
use druid::{Widget, WidgetExt, Event};
use druid::widget::{Label, ViewSwitcher};

// local dependencies
use crate::visualizers::bot_mode::bot_side;
use crate::visualizers::user_mode::user_side;
use crate::visualizers::datas::TraderUi;
use crate::visualizers::events::EventLogger;

/// the build_ui function creates the main ui of the application
///
/// **Federico Brancasi**
pub fn build_ui() -> impl Widget<TraderUi> {

    // the view switcher is used to switch between the user side and the bots side
    let view_switcher = ViewSwitcher::new(
        |data: &TraderUi, _env| data.current_view,
        |selector, _data, _env| match selector {
            // the bots side is the first view
            0 => Box::new(user_side::user_side()),
            // the user side is the second view
            1 => Box::new(bot_side::bot_side()),
            // if the selector is not 0 or 1, the application shows an error
            _ => Box::new(Label::new("Error").center()),
        },
    );

    // returns the ui of the application
    view_switcher.controller(EventLogger {
        filter: |event| matches!(event, Event::MouseDown(_))
    })
}