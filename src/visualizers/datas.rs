// libraries dependencies
use std::cell::RefCell;
use std::rc::Rc;
use druid::im::{Vector, vector};
use druid::{Data, Lens};

// market dependencies
use unitn_market_2022::market::Market;
use bfb::bfb_market::Bfb as bfb;
use market_sol::SOLMarket as sol;
use parse_market::ParseMarket as parse;

// local dependencies
// #[path = "../bots/mod.rs"]
// mod bots;

/// the TraderUi struct is the main struct which
/// contains the data of the application
///
/// **Federico Brancasi**
#[derive(Clone, Data, Lens)]
pub struct TraderUi {
    pub current_view: u32,
    pub bfb_kinds: Vector<String>,
    pub bfb_quantities: Vector<f32>,
    pub bfb_exchange_rate_buy: Vector<f32>,
    pub bfb_exchange_rate_sell: Vector<f32>,
    pub sol_kinds: Vector<String>,
    pub sol_quantities: Vector<f32>,
    pub sol_exchange_rate_buy: Vector<f32>,
    pub sol_exchange_rate_sell: Vector<f32>,
    pub parse_kinds: Vector<String>,
    pub parse_quantities: Vector<f32>,
    pub parse_exchange_rate_buy: Vector<f32>,
    pub parse_exchange_rate_sell: Vector<f32>,
    pub markets: Vector<Rc<RefCell<dyn Market>>>,
    pub trader: Trader,
    pub quantity: f32,
    pub percentage: f64,
    pub boolean: bool,
    pub safe_mode: bool,
    pub selected_market: String,
    pub selected_good: String,
    pub selected_method_of_trade: String,
}

/// the SingleMarket struct is used to store the data of
/// a single market
///
/// **Federico Brancasi**
#[derive(Clone, Data, Lens)]
pub struct Trader {
    pub(crate) name: String,
    pub(crate) goods: Vector<f32>,
}

/// impl block of the TraderUi struct
///
///
/// **Federico Brancasi**
impl TraderUi {
    /// the new function creates a new instance of the TraderUi struct
    pub fn new() -> Self {
        Self {
            current_view: 0,
            bfb_kinds: vector![" ".to_string(), " ".to_string(), " ".to_string(), " ".to_string()],
            bfb_quantities: vector![0.0, 0.0, 0.0, 0.0],
            bfb_exchange_rate_buy: vector![0.0, 0.0, 0.0, 0.0],
            bfb_exchange_rate_sell: vector![0.0, 0.0, 0.0, 0.0],
            sol_kinds: vector![" ".to_string(), " ".to_string(), " ".to_string(), " ".to_string()],
            sol_quantities: vector![0.0, 0.0, 0.0, 0.0],
            sol_exchange_rate_buy: vector![0.0, 0.0, 0.0, 0.0],
            sol_exchange_rate_sell: vector![0.0, 0.0, 0.0, 0.0],
            parse_kinds: vector![" ".to_string(), " ".to_string(), " ".to_string(), " ".to_string()],
            parse_quantities: vector![0.0, 0.0, 0.0, 0.0],
            parse_exchange_rate_buy: vector![0.0, 0.0, 0.0, 0.0],
            parse_exchange_rate_sell: vector![0.0, 0.0, 0.0, 0.0],
            markets: vector![bfb::new_random(), sol::new_random(), parse::new_random(),],
            trader: Trader { name: "TRADER TSE".to_string(), goods: vector![0.0, 0.0, 0.0, 0.0] },
            quantity: 0.0,
            percentage: 1.0,
            boolean: false,
            safe_mode: true,
            selected_market: "BFB".to_string(),
            selected_good: "EUR".to_string(),
            selected_method_of_trade: "SELL".to_string(),
        }
    }
}

/// support functions of the TraderUi struct:

/// the get_market_index function returns the index of the market from the name
///
/// **Federico Brancasi**
// pub fn get_market_index(market_name: &str) -> usize {
//     match market_name.to_lowercase().as_str() {
//         "bfb" => 0,
//         "sol" => 1,
//         "parse" => 2,
//         _ => panic!("Market not found"),
//     }
// }

/// initializer of the TraderUi struct
/// this function is called when the TraderUi is created and it
/// initializes the TraderUi struct datas when the program starts
///
/// **Federico Brancasi**
pub(crate) fn initialize_quantities(app: &mut TraderUi) -> &mut TraderUi {

    // set values for bfb market
    let (good_kinds_bfb, quantities_bfb, exchange_rate_buy_bfb, exchange_rate_sell_bfb) = get_market_info(&app.markets[0]);

    app.bfb_kinds = good_kinds_bfb;
    app.bfb_quantities = quantities_bfb;
    app.bfb_exchange_rate_buy = exchange_rate_buy_bfb;
    app.bfb_exchange_rate_sell = exchange_rate_sell_bfb;

    // set values for sol market
    let (good_kinds_sol, quantities_sol, exchange_rate_buy_sol, exchange_rate_sell_sol) = get_market_info(&app.markets[1]);

    app.sol_kinds = good_kinds_sol;
    app.sol_quantities = quantities_sol;
    app.sol_exchange_rate_buy = exchange_rate_buy_sol;
    app.sol_exchange_rate_sell = exchange_rate_sell_sol;

    // set values for parse market
    let (good_kinds_parse, quantities_parse, exchange_rate_buy_parse, exchange_rate_sell_parse) = get_market_info(&app.markets[2]);

    app.parse_kinds = good_kinds_parse;
    app.parse_quantities = quantities_parse;
    app.parse_exchange_rate_buy = exchange_rate_buy_parse;
    app.parse_exchange_rate_sell = exchange_rate_sell_parse;

    app.trader.goods = vector![40000.0, 0.0, 20000.0, 30000.0];

    app.quantity = app.trader.goods[0];

    app
}

/// the get_market_info function returns the market info in order to be used in the initialize_quantities function
pub fn get_market_info(market: &Rc<RefCell<dyn Market>>) -> (Vector<String>, Vector<f32>, Vector<f32>, Vector<f32>) {
    let mut buy_rates = Vector::new();
    let mut sell_rates = Vector::new();
    let mut good_kinds = Vector::new();
    let mut quantities = Vector::new();

    for good in market.borrow().get_goods() {
        let buy_rate = good.exchange_rate_buy;
        let sell_rate = good.exchange_rate_sell;
        let quantity = good.quantity;
        let kind = good.good_kind.to_string();
        buy_rates.push_back(buy_rate);
        sell_rates.push_back(sell_rate);
        good_kinds.push_back(kind);
        quantities.push_back(quantity);
    }

    (good_kinds, quantities, buy_rates, sell_rates)
}