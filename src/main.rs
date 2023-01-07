use std::cell::RefCell;
use std::rc::Rc;
use parse_market::ParseMarket as parse;
use market_sol::SOLMarket as sol;
use bfb::bfb_market::Bfb as bfb;
use unitn_market_2022::market::Market;

/// the initialization function randomly generated the initial quantity in the markets
///
/// **Andrea Ballarini**
fn random_init() -> (Rc<RefCell<dyn Market>>, Rc<RefCell<dyn Market>>, Rc<RefCell<dyn Market>>) {
    (sol::new_random(),
    parse::new_random(),
    bfb::new_random())
}

///the initialization function with the initial quantity in the markets
///
/// **Andrea Ballarini**
fn init_with_quantity(eur: f32, yen: f32, usd: f32, yuan: f32) -> (Rc<RefCell<dyn Market>>, Rc<RefCell<dyn Market>>, Rc<RefCell<dyn Market>>) {
    (sol::new_with_quantities(eur, yen, usd, yuan),
    parse::new_with_quantities(eur, yen, usd, yuan),
    bfb::new_with_quantities(eur, yen, usd, yuan))
}

///print the values in good labels for each market
///
/// **Andrea Ballarini**
fn print_values(market: &Rc<RefCell<dyn Market>>) {

    let market = market.borrow_mut();
    let goods = market.get_goods();

    //print the values in good labels
    for i in 0..goods.len() {
        println!("{:?}",goods[i]);
    }
}


fn main() {

    //initialize the trader name
    let _trader_name = "TSE".to_string();

    // the random initialization of the markets
    let (mut sol, mut parse, mut bfb) = random_init();

    // print the value in good labels for each market
    println!("\t\tRandomly Generated");
    println!("SOL:");
    print_values(&sol);
    println!("PARSE:");
    print_values(&parse);
    println!("BFB:");
    print_values(&bfb);
    println!(" ");

    // the initialization of the markets with the initial quantity
    (sol, parse, bfb) = init_with_quantity(100.0, 100.0, 100.0, 100.0);

    // print the value in good labels for each market
    println!("\t\tWith Initial Quantity");
    println!("SOL:");
    print_values(&sol);
    println!("PARSE:");
    print_values(&parse);
    println!("BFB:");
    print_values(&bfb);
    println!(" ");

}
