// libraries dependencies
use druid::{AppLauncher, WindowDesc};
use gag::Gag;

// local dependencies
mod bots;
mod visualizers;
// function that creates the menu of the application
use visualizers::app_menu::make_menu;
// function that creates the ui of the application
use visualizers::build_ui::build_ui;
// struct that contains the datas of the application
use visualizers::datas::TraderUi;
// function to initialize the datas of the TraderUi struct
use crate::visualizers::datas::initialize_quantities;

#[doc= include_str!("../README.md")]

fn main() {
    // for blocking the stdout of the application
    // no messages from the markets will be printed
    let _block_stdout = Gag::stdout().unwrap();

    // creation of the main window
    let main_window = WindowDesc::new(build_ui())
        .window_size((1500.0, 930.0))
        .with_min_size((1500.0, 930.0))
        .menu(make_menu)
        .title("Trader TSE");

    // creation of the application
    let mut app = TraderUi::new();

    initialize_quantities(&mut app);

    // initial data of the application
    let initial_data = app;

    // the launcher of the application
    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
