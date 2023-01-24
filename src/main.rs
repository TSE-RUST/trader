// libraries dependencies
use colored::*;
use druid::{AppLauncher, WindowDesc};
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

// todo()
use bots::bot::bot;
// todo()
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
pub struct Trader {
    name: String,
    _money: f32,
    _goods: Vec<Good>,
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
            _goods: initgoods(money, 0.0, 0.0, 0.0),
            sol,
            bfb,
            parse,
        }
    }
}

fn main() {
    //initialize the trader name
    let trader_name = "TSE".to_string();

    // the random initialization of the markets
    let (mut sol, mut parse, mut bfb) = random_init();

    // print the value in good labels for each market
    println!(
        "{}",
        "\nRandomly Generated"
            .truecolor(30, 144, 255)
            .bold()
            .underline()
    );
    println!("{}", "SOL:".truecolor(0, 191, 255).bold());
    print_values(&sol);
    println!("{}", "PARSE:".truecolor(0, 191, 255).bold());
    print_values(&parse);
    println!("{}", "BFB:".truecolor(0, 191, 255).bold());
    print_values(&bfb);
    println!(" ");

    // the initialization of the markets with the initial quantity
    {
        let _ = Gag::stdout().unwrap();
        (sol, parse, bfb) = init_with_quantity(100.0, 100.0, 100.0, 100.0);
    }

    // print the value in good labels for each market
    println!(
        "{}",
        "With Initial Quantity"
            .truecolor(30, 144, 255)
            .bold()
            .underline()
    );
    println!("{}", "SOL:".truecolor(0, 191, 255).bold());
    print_values(&sol);
    println!("{}", "PARSE:".truecolor(0, 191, 255).bold());
    print_values(&parse);
    println!("{}", "BFB:".truecolor(0, 191, 255).bold());
    print_values(&bfb);
    println!(" ");

    subscribe_each_other!(sol, parse, bfb);

    // per vedere l'output commentatela
    // let print_gag = Gag::stdout().unwrap();

    //initialize the trader
    let mut trader = Trader::new(
       trader_name,
       1000.00,
       sol.clone(),
       bfb.clone(),
       parse.clone(),
    );

    // test arbitrager
    /*{
        let mut tmp = (*parse).borrow_mut();
        let t = tmp
            .lock_buy(GoodKind::USD, 25., 200., "dsds".to_string())
            .unwrap();
        tmp.buy(t, &mut Good::new(GoodKind::EUR, 200.));
    }
    {
        let mut tmp = (*parse).borrow_mut();
        let t = tmp
            .lock_buy(GoodKind::USD, 25., 200., "dsds".to_string())
            .unwrap();
        tmp.buy(t, &mut Good::new(GoodKind::EUR, 200.));
    }*/

    // TEST ARBITRAGE
    //{
    //    let mut profit = 0.;
    //    let mut usd = Good::new(GoodKind::USD, 0.);
    //    let mut yen = Good::new(GoodKind::YEN, 0.);
    //    let mut yuan = Good::new(GoodKind::YUAN, 0.);
    //    for i in 0..100 {
    //        let arbitrager = Arbitrager::new("trado".to_string(), &sol, &bfb, &parse);
    //        //
    //        let tmp = arbitrager.arbitrage(Good::new(GoodKind::EUR, 1000.));
    //        profit += tmp.0.get_qty();
    //        match tmp.1 {
    //            Some(g) => match g.get_kind() {
    //                GoodKind::EUR => todo!(),
    //                GoodKind::YEN => {
    //                    yen.merge(g);
    //                }
    //                GoodKind::USD => {
    //                    usd.merge(g);
    //                }
    //                GoodKind::YUAN => {
    //                    yuan.merge(g);
    //                }
    //            },
    //            None => {}
    //        }
    //        match tmp.2 {
    //            Some(res) => {
    //                println!();
//
    //                println!(
    //                    "😉 {}......{}......{}",
    //                    res.buy_market_name,
    //                    res.sell_market_name,
    //                    res.eur_received - res.eur_sent
    //                );
    //            }
    //            None => {}
    //        }
    //    }
    //    println!(
    //        "EUR gained: {}, USD: {}, YEN: {}, YUAN: {}",
    //        profit,
    //        usd.get_qty(),
    //        yen.get_qty(),
    //        yuan.get_qty()
    //    );
    //}

    //test_buy_kind(GoodKind::USD, &mut trader);
    //_test_sell_kind(GoodKind::YEN, &mut trader);
    //bots(&mut trader);

    /// UI
    // creation of the main window
    let main_window = WindowDesc::new(build_ui())
        .window_size((1400.0, 900.0))
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
