// libraries dependencies
use druid::{AppLauncher, WindowDesc};
use gag::Gag;

// market dependencies

// local dependencies
mod bots;

// the function to run the simulation in the trader
use bots::bot_strategy::bot::bot;
// When you want to invest some money in arbitrage you can create an arbitrager passing the 3 markets
// then call his main function "arbitrage" passing the eur you want to invest, and returns:
// 1. eur returned back (could be with 0 quantity)
// 2. optional good as rest as the arbitrage
// 3. if there was an arbitrage, returns an ArbitrageResult, otherwise None. If None implies that the second returned parameter is None
use bots::arbitrager_strategy::arbitrager::arbitrage;

mod visualizers;

// function that creates the menu of the application
use visualizers::app_menu::make_menu;
// function that creates the ui of the application
use visualizers::build_ui::build_ui;
// struct that contains the datas of the application
use visualizers::datas::TraderUi;
// function that initializes the datas inside the TraderUi struct
// use visualizers::user_mode::datas::initialize_quantities;

// function to initialize the datas of the TraderUi struct
use crate::visualizers::datas::initialize_quantities;

fn main() {

    // for blocking the stdout of the application
    // no messages from the markets will be printed
    // let _block_stdout = Gag::stdout().unwrap();

    // creation of the main window
    let main_window = WindowDesc::new(build_ui())
        .window_size((1400.0, 930.0))
        .with_min_size((1400.0, 930.0))
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
