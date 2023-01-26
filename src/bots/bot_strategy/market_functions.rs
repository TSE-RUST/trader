// libraries dependencies
use druid::im::{vector, Vector};
use std::cell::RefCell;
use std::rc::Rc;

// market dependencies
use bfb::bfb_market::Bfb as bfb;
use market_sol::SOLMarket as sol;
use parse_market::ParseMarket as parse;
use unitn_market_2022::good::good::Good;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;

// local dependencies
use crate::bots::bot_strategy::bot::Trader;

///initialize the goods for the trader
///
/// **Andrea Ballarini**
pub(crate) fn initgoods(usd: f32, yen: f32, yuan: f32) -> Vector<Rc<RefCell<Good>>> {
    let mut goods = vector![];
    goods.push_back(Rc::new(RefCell::new(Good::new(GoodKind::USD, usd))));
    goods.push_back(Rc::new(RefCell::new(Good::new(GoodKind::YEN, yen))));
    goods.push_back(Rc::new(RefCell::new(Good::new(GoodKind::YUAN, yuan))));
    goods
}

/// the initialization function randomly generated the initial quantity in the markets
///
/// **Andrea Ballarini**
pub(crate) fn random_init() -> (
    Rc<RefCell<dyn Market>>,
    Rc<RefCell<dyn Market>>,
    Rc<RefCell<dyn Market>>,
) {
    (sol::new_random(), parse::new_random(), bfb::new_random())
}

///the initialization function with the initial quantity in the markets
///
/// **Andrea Ballarini**
pub(crate) fn init_with_quantity(
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
pub(crate) fn print_values(market: &Rc<RefCell<dyn Market>>) {
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
fn test_sell_kind(kind: GoodKind, trader: &mut Trader) {
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
