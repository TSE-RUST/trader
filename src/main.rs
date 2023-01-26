// libraries dependencies
use colored::*;
use druid::im::Vector;
use druid::{AppLauncher, WindowDesc};
use druid::{Data, Lens};
use gag::Gag;
use std::cell::RefCell;
use std::rc::Rc;

// market dependencies
use unitn_market_2022::good::good::Good;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;
use unitn_market_2022::subscribe_each_other;

// local dependencies
mod bots;

// the function to run the simulation in the trader
use bots::bot::bot;
// When you want to invest some money in arbitrage you can create an arbitrager passing the 3 markets
// then call his main function "arbitrage" passing the eur you want to invest, and returns:
// 1. eur returned back (could be with 0 quantity)
// 2. optional good as rest as the arbitrage
// 3. if there was an arbitrage, returns an ArbitrageResult, otherwise None. If None implies that the second returned parameter is None
use bots::arbitrager::Arbitrager;

mod visualizers;

// function that creates the menu of the application
use visualizers::app_menu::make_menu;
// function that creates the ui of the application
use visualizers::build_ui::build_ui;
// struct that contains the datas of the application
use visualizers::datas::TraderUi;
// function that initializes the datas inside the TraderUi struct
use visualizers::datas::initialize_quantities;

mod market_functions;

// function that initializes the goods for the trader
use market_functions::initgoods;
// function that initializes the markets randomly
use market_functions::random_init;
// function that initializes the markets with the given quantities
use market_functions::init_with_quantity;
// function that prints the goods of the markets
use market_functions::print_values;

///the struct for the trader agent
#[derive(Clone, Data, Lens)]
pub struct Trader {
    name: String,
    _money: f32,
    _goods: Vector<Rc<RefCell<Good>>>,
    sol: Rc<RefCell<dyn Market>>,
    bfb: Rc<RefCell<dyn Market>>,
    parse: Rc<RefCell<dyn Market>>,
}
impl Trader {
    ///the constructor for the trader agent
    ///
    /// **Andrea Ballarini**
    fn new(
        name: String,
        money: f32,
        sol: Rc<RefCell<dyn Market>>,
        bfb: Rc<RefCell<dyn Market>>,
        parse: Rc<RefCell<dyn Market>>,
    ) -> Self {
        Trader {
            name,
            _money: money,
            _goods: initgoods(0.0, 0.0, 0.0),
            sol,
            bfb,
            parse,
        }
    }
}

fn main() {

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

    // for blocking the stdout of the application
    // no messages from the markets will be printed
    let _block_stdout = Gag::stdout().unwrap();

    // the launcher of the application
    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");

}
