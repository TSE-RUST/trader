// libraries dependencies
use druid::{Env, LocalizedString, Menu, MenuItem, WindowId};

// local dependencies
use crate::TraderUi;

/// this function renders the menu bar for the application
#[allow(unused_assignments, unused_mut)]
pub fn make_menu(_window_id: Option<WindowId>, _app_state: &TraderUi, _env: &Env) -> Menu<TraderUi> {

    // base of our menu (create a empty menu)
    let mut base = Menu::empty();

    // mac target
    #[cfg(target_os = "macos")]
    {
        // default menu (we want to personalize it, so we don't use it)
        // base = base.entry(druid::platform_menus::mac::application::default())
        create_menu(base)

    }

    // other targets
    #[cfg(any(
    target_os = "windows",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "openbsd"
    ))]
    {
    // default menu (we want to personalize it, so we don't use it)
    // base = base.entry(druid::platform_menus::win::file::default());
    create_menu(base)
    }
}

/// this function creates the menu of our app (we use an external
/// function to make it more readable and not to duplicate code)
fn create_menu(base: Menu<TraderUi>) -> Menu<TraderUi> {
    base.entry(
        Menu::new(LocalizedString::new("App"))
            .entry(
                MenuItem::new(LocalizedString::new("Trader Mode"))
                    .on_activate(|_ctx, data: &mut TraderUi, _env| data.current_view = 0),
            )
            .entry(
                MenuItem::new(LocalizedString::new("Bot Mode"))
                    .on_activate(|_ctx, data: &mut TraderUi, _env| data.current_view = 1),
            )
            .separator()
            .entry(druid::platform_menus::win::file::exit()),
    )
}
