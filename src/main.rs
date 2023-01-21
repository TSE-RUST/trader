use bfb::bfb_market::Bfb as bfb;
use colored::*;
use gag::Gag;
use market_sol::SOLMarket as sol;
use parse_market::ParseMarket as parse;
use std::cell::RefCell;
use std::rc::Rc;
use unitn_market_2022::good::good::Good;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;
use unitn_market_2022::subscribe_each_other;

mod arbitrager;

use crate::arbitrager::ArbitrageResult;
use crate::arbitrager::Arbitrager;

///the struct for the trader agent
pub struct Trader {
    name: String,
    _money: f32,
    _goods: Vec<Good>,
    sol: Rc<RefCell<dyn Market>>,
    bfb: Rc<RefCell<dyn Market>>,
    parse: Rc<RefCell<dyn Market>>,
}

///initialize the goods for the trader
///
/// **Andrea Ballarini**
fn initgoods(eur: f32, usd: f32, yen: f32, yuan: f32) -> Vec<Good> {
    let mut goods: Vec<Good> = Vec::new();
    goods.push(Good::new(GoodKind::EUR, eur));
    goods.push(Good::new(GoodKind::USD, usd));
    goods.push(Good::new(GoodKind::YEN, yen));
    goods.push(Good::new(GoodKind::YUAN, yuan));
    goods
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

/// the initialization function randomly generated the initial quantity in the markets
///
/// **Andrea Ballarini**
fn random_init() -> (
    Rc<RefCell<dyn Market>>,
    Rc<RefCell<dyn Market>>,
    Rc<RefCell<dyn Market>>,
) {
    (sol::new_random(), parse::new_random(), bfb::new_random())
}

///the initialization function with the initial quantity in the markets
///
/// **Andrea Ballarini**
fn init_with_quantity(
    eur: f32,
    yen: f32,
    usd: f32,
    yuan: f32,
) -> (
    Rc<RefCell<dyn Market>>,
    Rc<RefCell<dyn Market>>,
    Rc<RefCell<dyn Market>>,
) {
    (
        sol::new_with_quantities(eur, yen, usd, yuan),
        parse::new_with_quantities(eur, yen, usd, yuan),
        bfb::new_with_quantities(eur, yen, usd, yuan),
    )
}

///print the values in good labels for each market
///
/// **Andrea Ballarini**
fn print_values(market: &Rc<RefCell<dyn Market>>) {
    let market = market.borrow();
    let goods = market.get_goods();

    //print the values in good labels
    for i in 0..goods.len() {
        println!("{:?}", goods[i]);
    }
}

///print the values in a specific good label kind
///
/// **Andrea Ballarini**
fn print_good_kind(market: &Rc<RefCell<dyn Market>>, kind: GoodKind) {
    let market = market.borrow();
    let goods = market.get_goods();

    //print the values in good labels
    for i in 0..goods.len() {
        if goods[i].good_kind == kind {
            println!("{:?}", goods[i]);
        }
    }
}

///testing the buy function for a specific
/// GoodKind and the fluctuation prices in markets
///
/// **Andrea Ballarini**
fn test_buy_kind(kind: GoodKind, trader: &mut Trader) {
    //try the changes after buying all sol
    let mut count = 0;
    println!("\t\tAfter Buying All SOL");
    loop {
        if count == 150 {
            break;
        }
        //buy from SOL
        let sol_price = match trader.sol.borrow().get_buy_price(kind, 10.0) {
            Ok(price) => price,
            Err(_) => break,
        };
        let token =
            match (*trader.sol)
                .borrow_mut()
                .lock_buy(kind, 10.0, sol_price, trader.name.clone())
            {
                Ok(token) => token,
                Err(_) => break,
            };
        let _ = (*trader.sol)
            .borrow_mut()
            .buy(token, &mut Good::new(GoodKind::EUR, sol_price));
        print_good_kind(&trader.sol, kind);
        count += 1;
    }

    //try the changes after buying all PARSE
    println!("\t\tAfter Buying All PARSE");
    loop {
        //buy from PARSE
        let parse_price = match trader.parse.borrow().get_buy_price(kind, 10.0) {
            Ok(price) => price,
            Err(_) => break,
        };
        let token = match (*trader.parse).borrow_mut().lock_buy(
            kind,
            10.0,
            parse_price,
            trader.name.clone(),
        ) {
            Ok(token) => token,
            Err(_) => break,
        };
        let _ = (*trader.parse)
            .borrow_mut()
            .buy(token, &mut Good::new(GoodKind::EUR, parse_price));
        print_good_kind(&trader.parse, kind);
    }

    //try the changes after buying all BFB
    println!("\t\tAfter Buying All BFB");
    loop {
        //buy from BFB
        let bfb_price = match trader.bfb.borrow().get_buy_price(kind, 10.0) {
            Ok(price) => price,
            Err(_) => break,
        };
        let token =
            match (*trader.bfb)
                .borrow_mut()
                .lock_buy(kind, 10.0, bfb_price, trader.name.clone())
            {
                Ok(token) => token,
                Err(_) => break,
            };
        let _ = (*trader.bfb)
            .borrow_mut()
            .buy(token, &mut Good::new(GoodKind::EUR, bfb_price));
        print_good_kind(&trader.bfb, kind);
    }
}

///testing the sell function for a specific GoodKind and the fluctuation prices in markets
///
/// **Andrea Ballarini**
fn _test_sell_kind(kind: GoodKind, trader: &mut Trader) {
    //try the changes after selling all sol
    let mut count = 0;
    println!("\t\tAfter Selling All SOL");
    loop {
        if count == 150 {
            break;
        }
        //sell from SOL
        let sol_price = match trader.sol.borrow().get_sell_price(kind, 10.0) {
            Ok(price) => price,
            Err(_) => break,
        };
        let token =
            match (*trader.sol)
                .borrow_mut()
                .lock_sell(kind, 10.0, sol_price, trader.name.clone())
            {
                Ok(token) => token,
                Err(_) => break,
            };
        let _ = match (*trader.sol)
            .borrow_mut()
            .sell(token, &mut Good::new(kind, 10.0))
        {
            Ok(_) => (),
            Err(_) => break,
        };
        print_good_kind(&trader.sol, kind);
        count += 1;
    }

    //try the changes after selling all PARSE
    println!("\t\tAfter Selling All PARSE");
    loop {
        //sell from PARSE
        let parse_price = match trader.parse.borrow().get_sell_price(kind, 10.0) {
            Ok(price) => price,
            Err(_) => break,
        };
        let token = match (*trader.parse).borrow_mut().lock_sell(
            kind,
            10.0,
            parse_price,
            trader.name.clone(),
        ) {
            Ok(token) => token,
            Err(_) => break,
        };
        let _ = match (*trader.parse)
            .borrow_mut()
            .sell(token, &mut Good::new(kind, 10.0))
        {
            Ok(_) => (),
            Err(_) => break,
        };
        print_good_kind(&trader.parse, kind);
    }

    //try the changes after selling all BFB
    println!("\t\tAfter Selling All BFB");
    loop {
        //sell from BFB
        let bfb_price = match trader.bfb.borrow().get_sell_price(kind, 10.0) {
            Ok(price) => price,
            Err(_) => break,
        };
        let token =
            match (*trader.bfb)
                .borrow_mut()
                .lock_sell(kind, 10.0, bfb_price, trader.name.clone())
            {
                Ok(token) => token,
                Err(_) => break,
            };
        let _ = match (*trader.bfb)
            .borrow_mut()
            .sell(token, &mut Good::new(kind, 10.0))
        {
            Ok(_) => (),
            Err(_) => break,
        };
        print_good_kind(&trader.bfb, kind);
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
        let hide_stdout = Gag::stdout().unwrap();
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
    let print_gag = Gag::stdout().unwrap();

    //initialize the trader
    let mut trader = Trader::new(
        trader_name,
        1000.00,
        sol.clone(),
        bfb.clone(),
        parse.clone(),
    );

    // test
    {
        let mut tmp = (*sol).borrow_mut();
        let t = tmp
            .lock_buy(GoodKind::USD, 50., 200., "dsds".to_string())
            .unwrap();
        tmp.buy(t, &mut Good::new(GoodKind::EUR, 200.));
    }
    let arbitrager = Arbitrager::new("trado".to_string(), &sol, &bfb, &parse);

    arbitrager.arbitrage(Good::new(GoodKind::EUR, 1000.));

    test_buy_kind(GoodKind::USD, &mut trader);
    //test_sell_kind(GoodKind::USD, &mut trader);
}
