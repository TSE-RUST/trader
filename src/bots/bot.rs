// libraries dependencies
use std::cell::RefCell;
use std::rc::Rc;

// market dependencies
use unitn_market_2022::good::good::Good;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::Market;
use unitn_market_2022::wait_one_day;

// local dependencies
use crate::Trader;


///return a vector for iterating through the GoodKinds without the EUR
///
/// **Andrea Ballarini**
fn get_good_kinds() -> Vec<GoodKind> {
    vec![GoodKind::USD, GoodKind::YEN, GoodKind::YUAN]
}

/// get the 1/3 quantity of the kind in the trader
pub fn get_trader_quantity(trader: &Trader, kind: GoodKind) -> f32 {
    let mut quantity = 0.0;
    for good in &trader._goods {
        if good.get_kind() == kind {
            quantity = good.get_qty();
        }
    }
    quantity
}

///get the maximum quantity of a good that can be bought from a market with a specific quantity of money
///
/// **Andrea Ballarini**
fn get_max_buy_quantity(market: &Rc<RefCell<dyn Market>>, money: f32, kind: GoodKind) -> f32 {
    let market = market.borrow();
    let goods = market.get_goods();
    let mut max_quantity = 0.0;
    for i in 0..goods.len() {
        if goods[i].good_kind == kind {

            max_quantity = goods[i].quantity;

        }
    }
    if max_quantity > 0.0{
        let mut buy_price = market.get_buy_price(kind, max_quantity).expect("Error in get_buy_price in the max_buy_quantity function");
        while money < buy_price && max_quantity > 0.0 {
            max_quantity = max_quantity/2.;
            buy_price = market.get_buy_price(kind, max_quantity).expect("Error in get_buy_price in the max_buy_quantity function");
        }
    }

    max_quantity
}

///get the maximum quantity of a good that can be sold to a market with a specific quantity of that good and the availability of GoodKind::EUR in the market
///
/// **Andrea Ballarini**
pub fn get_max_sell_quantity(market: &Rc<RefCell<dyn Market>>, quantity: f32, kind: GoodKind) -> f32 {
    let market = market.borrow();
    let goods = market.get_goods();

    let mut eur_quantity = 0.0;

    let mut max_quantity = quantity;

     for i in 0..goods.len() {

        if goods[i].good_kind == GoodKind::EUR {
            eur_quantity=goods[i].quantity;
        }
     }
    //look if the market has the amount of GoodKind::EUR to buy the quantity of the good
    if eur_quantity > 0.0 && max_quantity > 0.0{
        let mut sell_price = market.get_sell_price(kind, max_quantity).expect("Error in get_sell_price in the max_sell_quantity function");
        while eur_quantity < sell_price && max_quantity > 0.0 {
            //divido la quantità in 2 e ritento
            max_quantity = max_quantity/2.;
            sell_price = market.get_sell_price(kind, max_quantity).expect("Error in get_sell_price in the max_sell_quantity function");
        }
    }

    max_quantity
}

///get the average buy price for 1 quantity of the good taken by the maximum between the 1/3 of the trader budget and the good availability in the specific market
///
/// **Andrea Ballarini**
pub fn get_average_buy_price(trader: &mut Trader, kind: GoodKind) -> f32 {
    //the budget is 1/3 of the trader.budget (trader._money)
    let budget = trader._money / 3.;

    let mut price_sol:f32 = 0.;
    let mut price_parse:f32 = 0.;
    let mut price_bfb:f32 = 0.;

    //get the sol maximum quantity that can be bought with the 1/3 of the budget
    let max_quantity_sol = get_max_buy_quantity(&trader.sol, budget, kind);
    if max_quantity_sol > 0.0 {
        price_sol = trader.sol.borrow().get_buy_price(kind, max_quantity_sol).unwrap();
    }

    //get the parse maximum quantity that can be bought with the 1/3 of the budget
    let max_quantity_parse = get_max_buy_quantity(&trader.parse, budget, kind);
    if max_quantity_parse > 0.0{
        price_parse = trader.parse.borrow().get_buy_price(kind, max_quantity_parse).unwrap();
    }

    //get the bfb maximum quantity that can be bought with the 1/3 of the budget
    let max_quantity_bfb = get_max_buy_quantity(&trader.bfb, budget, kind);
    if max_quantity_bfb > 0.0{
        price_bfb = trader.bfb.borrow().get_buy_price(kind, max_quantity_bfb).unwrap();
    }

    //get the average price of the three markets for the specific GoodKind and 1 quantity
    let average_price = (price_sol + price_parse + price_bfb) / (max_quantity_sol + max_quantity_parse + max_quantity_bfb);

    average_price
}

///get the average sell price taken by the maximum between the 1/3 of the trader.budget and the good availability in the market
///
/// **Andrea Ballarini**
pub fn get_average_sell_price(trader: &mut Trader, kind: GoodKind,quantity:f32) -> f32 {
    let mut price_sol:f32 = 0.;
    let mut price_parse:f32 = 0.;
    let mut price_bfb:f32 = 0.;


    //get the sol maximum quantity that can be sold with the 1/3 of the budget
    let max_quantity_sol = get_max_sell_quantity(&trader.sol, quantity, kind);
    if max_quantity_sol > 0.0 {
        price_sol = trader.sol.borrow().get_sell_price(kind, max_quantity_sol).unwrap();
    }

    //get the parse maximum quantity that can be sold with the 1/3 of the budget
    let max_quantity_parse = get_max_sell_quantity(&trader.parse, quantity, kind);
    if max_quantity_parse != 0.0{
        price_parse = trader.parse.borrow().get_sell_price(kind, max_quantity_parse).unwrap();
    }

    //get the bfb maximum quantity that can be sold with the 1/3 of the budget
    let max_quantity_bfb = get_max_sell_quantity(&trader.bfb, quantity, kind);
    if max_quantity_bfb != 0.0{
        price_bfb = trader.bfb.borrow().get_sell_price(kind, max_quantity_bfb).unwrap();
    }

    //get the average price
    let average_price = (price_sol + price_parse + price_bfb) / (max_quantity_sol + max_quantity_parse + max_quantity_bfb);

    average_price
}

/// # GET BEST BUY MARKET, QUANTITY AND PRICE
/// get the best market to buy a specific good where the price is lower than the average price of the three markets and preferring the BFB market
///
/// **Andrea Ballarini**
pub fn get_best_buy_market(trader: &mut Trader, kind: GoodKind) -> (&mut Rc<RefCell<dyn Market>>, f32, f32) {
    let budget = trader._money / 3.;
    let average_price = get_average_buy_price(trader, kind);
    let mut best_price = 0.0;
    let mut best_quantity = 0.0;

    //get the best price in the bfb market
    let max_quantity_bfb = get_max_buy_quantity(&trader.bfb, budget, kind);
    let mut price = match trader.bfb.borrow().get_buy_price(kind, max_quantity_bfb){
        Ok(price) => price,
        _ => 0.0,
    };
    let mut best_market= &mut trader.bfb;
    if max_quantity_bfb > 0.0 {

        if (price/max_quantity_bfb) < average_price && average_price > 0.0 {
            best_price = price;
            best_quantity = max_quantity_bfb;
            return (best_market,best_quantity,best_price);
        }
    }

    //get the best price in the sol market
    let max_quantity_sol = get_max_buy_quantity(&trader.sol, budget, kind);
    if max_quantity_sol > 0.0 {
        price = trader.sol.borrow().get_buy_price(kind, max_quantity_sol).unwrap();
        if (price/max_quantity_sol) < average_price {
            best_market = &mut trader.sol;
            best_price = price;
            best_quantity = max_quantity_sol;
        }
    }

    //get the best price in the parse market

    let max_quantity_parse = get_max_buy_quantity(&trader.parse, budget, kind);
    if max_quantity_parse > 0.1{
        price = trader.parse.borrow().get_buy_price(kind, max_quantity_parse).unwrap();
        if (price/max_quantity_parse) < average_price {
            best_market = &mut trader.parse;
            best_price = price;
            best_quantity = max_quantity_parse;
        }
    }


    (best_market,best_quantity,best_price)
}

///# GET BEST SELL MARKET, QUANTITY AND PRICE
/// get the best market to sell a specific good where the price is higher than the average price of the three markets and preferring the PARSE market
///
/// **Andrea Ballarini**
pub fn get_best_sell_market(trader: &mut Trader, kind: GoodKind, quantity:f32) -> (&mut Rc<RefCell<dyn Market>>,f32,f32) {

    let average_price = get_average_sell_price(trader, kind,quantity);
    let mut best_price = 0.0;
    let mut best_quantity = 0.0;


    //get the best price in the parse market
    let max_quantity_parse = get_max_sell_quantity(&trader.parse, quantity, kind);
    let mut price = trader.parse.borrow().get_sell_price(kind, max_quantity_parse).unwrap();
    let mut best_market= &mut trader.parse;
    if max_quantity_parse > 0.1 {
        if (price/max_quantity_parse) > average_price && average_price != 0.0 {
            best_price = price;
            best_quantity = max_quantity_parse;
            return (best_market, best_quantity, best_price);
        }
    }

    //get the best price in the sol market
    let max_quantity_sol = get_max_sell_quantity(&trader.sol, quantity, kind);
    if max_quantity_sol > 0.0 {
        price = trader.sol.borrow().get_sell_price(kind, max_quantity_sol).unwrap();
        if (price/max_quantity_sol) > average_price {
            best_market = &mut trader.sol;
            best_price = price;
            best_quantity = max_quantity_sol;
        }
    }

    //get the best price in the parse market
    let max_quantity_bfb = get_max_sell_quantity(&trader.bfb, quantity, kind);
    if max_quantity_bfb > 0.1 {
        price = trader.bfb.borrow().get_sell_price(kind, max_quantity_bfb).unwrap();
        if (price/max_quantity_bfb) > average_price {
            best_market = &mut trader.bfb;
            best_price = price;
            best_quantity = max_quantity_bfb;
        }
    }

    (best_market, best_quantity, best_price)
}

/// # BOT
///Loop infinitely to buy and sell goods in the three markets and to print the money of the trader at the end of each day
///
/// **Andrea Ballarini**
pub fn bot(trader: &mut Trader) {

    let mut cont = 0;

    loop {
        if cont == 150 {
            //log all the goods and the money of the trader
            println!("The Trader has:");
            for good in &trader._goods {
                println!("{}: {}", good.get_kind(), good.get_qty());
            }
            println!("{} money", trader._money);



            break;
        }

        //buy and sell goods
        for good in get_good_kinds() {

            //buy from the market that kind
            let trader_name = trader.name.clone();
            let (market, quantity, price) = get_best_buy_market(trader, good);
            let market_name = market.borrow().get_name();

            if quantity > 0.1 {
                let mut cash = Good::new(GoodKind::EUR, price);

                let token = match market.borrow_mut().lock_buy(good, quantity, price,trader_name.clone()){
                    Ok(token) => token,
                    Err(e) => {panic!("Error in lock_buy in {}: {:?}", market_name.to_string(),e);},
                };
                let increase= match market.borrow_mut().buy(token, &mut cash){
                    Ok(increase) => increase,
                    Err(e) => {panic!("Error in buy in {}: {:?}", market_name.to_string(),e);},
                };
                trader._money -= price;
                for kind in trader._goods.iter_mut(){
                    if kind.get_kind() == good {
                        match kind.merge(increase.clone()){
                            Ok(_) => (),
                            Err(e) => println!("Error in merge {:?}", e),
                        }
                    }
                }
            }else {
                println!("No quantity to buy");
            }

            //log of the precedent buy
            println!("{} bought {} {} from {} for {} EUR", trader.name, quantity, good.to_string(), market_name, price);
            println!("Money: {}", trader._money);


            //sell to the market that kind

            //get the 1/3 quantity of the kind in the trader
            let quantity = get_trader_quantity(trader, good)/3.;

            let (market, quantity, price) = get_best_sell_market(trader, good, quantity);
            if quantity > 0.0 {
                let token = match market.borrow_mut().lock_sell(good, quantity, price,trader_name) {
                    Ok(token) => token,
                    Err(e) => {panic!("Error in lock_sell in {}: {:?}", market_name.to_string(),e);},
                };
                let mut cash = Good::new(good, quantity);
                let _decrease = match market.borrow_mut().sell(token, &mut cash){
                    Ok(decrease) => decrease,
                    Err(e) => {panic!("Error in sell in {}: {:?}", market_name.to_string(),e);},
                };
                trader._money += price;
                for kind in trader._goods.iter_mut(){
                    if kind.get_kind() == good {
                        match kind.split(quantity){
                            Ok(_) => (),
                            Err(e) => panic!("Error in split {:?}", e),
                        }
                    }
                }
                //log of the precedent sell
                println!("{} sold {} {} to {} for {} EUR", trader.name, quantity, good.to_string(), market_name, price);
            }else {
                //log of no sell action
                println!("No quantity to sell");
            }

            println!("Money: {}", trader._money);

        }


        cont += 1;

    }

    //sell everything to the market with the highest price
    let trader_name = trader.name.clone();
    let mut cont  = 0;
    for good in get_good_kinds() {
        let mut max_quantity = get_trader_quantity(trader, good);
        while max_quantity > 0.1 && cont < 100 {
            let (market, quantity, price) = get_best_sell_market(trader, good, max_quantity);
            if quantity > 0.1 {
                let market_name = market.borrow().get_name();
                let token = match market.borrow_mut().lock_sell(good, quantity, price,trader_name.clone()) {
                    Ok(token) => token,
                    Err(e) => {panic!("Error in lock_sell in {}: {:?}", market_name.to_string(),e);},
                };
                let mut cash = Good::new(good, quantity);
                let _decrease = match market.borrow_mut().sell(token, &mut cash){
                    Ok(decrease) => decrease,
                    Err(e) => {panic!("Error in sell in {}: {:?}", market_name.to_string(),e);},
                };
                trader._money += price;
                for kind in trader._goods.iter_mut(){
                    if kind.get_kind() == good {
                        match kind.split(quantity){
                            Ok(_) => (),
                            Err(e) => panic!("Error in split {:?}", e),
                        }
                    }
                }
                println!("{} sold {} {} to {} for {} EUR", trader.name, quantity, good.to_string(), market_name, price);
                println!("Money: {}", trader._money);
            }else {
                wait_one_day!(trader.sol);
                wait_one_day!(trader.parse);
                wait_one_day!(trader.bfb);
                max_quantity = get_trader_quantity(trader, good);
                println!("{} has {} {} left", trader.name, max_quantity, good.to_string());
                cont+=1;
            }

            max_quantity = get_trader_quantity(trader, good);


        }
        cont = 0;
    }
    println!("Money: {}", trader._money);
}
