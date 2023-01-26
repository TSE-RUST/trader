// libraries dependencies
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use druid::im::{Vector, vector};
use druid::{Data, Lens};

// market dependencies
use unitn_market_2022::market::Market;
use bfb::bfb_market::Bfb as bfb;
use market_sol::SOLMarket as sol;
use parse_market::ParseMarket as parse;
use crate::bots::arbitrager_strategy::arbitrager::arbitrage;
use crate::bots::bot_strategy::bot::bot;
use crate::visualizers::events::{LoggedEvent};

use crate::bots::bot_strategy::bot::TraderBot;
use crate::bots::bot_strategy::market_functions::random_init;
use crate::visualizers::user_mode::support_functions::{get_best_buy_trade, get_best_sell_trade, get_market_info};

/// the TraderUi struct is the main struct which
/// contains the data of the application
///
/// **Federico Brancasi**
#[derive(Clone, Data, Lens)]
pub struct TraderUi {
    pub current_view: u32,
    // USER MODE
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
    pub percentage_user: f64,
    pub boolean: bool,
    pub selected_market: String,
    pub selected_good: String,
    pub selected_method_of_trade: String,
    pub events: Arc<Vec<LoggedEvent>>,
    pub events_number: usize,
    pub string_best_profit_sell: String,
    pub string_best_profit_buy: String,
    // BOT MODE
    pub safe_mode: bool,
    pub logs: Vector<Vector<(String, String, String)>>,
    pub logs_bot: Vector<String>,
    pub bfb_logs_bot: Vector<String>,
    pub sol_logs_bot: Vector<String>,
    pub parse_logs_bot: Vector<String>,
    pub trader_bot: TraderBot,
    pub logs_arb: Vector<String>,
    pub bfb_logs_arb: Vector<String>,
    pub sol_logs_arb: Vector<String>,
    pub parse_logs_arb: Vector<String>,
    pub trader_arb: TraderBot,
    pub percentage_bot: f64,
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
            // USER MODE
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
            percentage_user: 1.0,
            boolean: false,
            selected_market: "BFB".to_string(),
            selected_good: "YEN".to_string(),
            selected_method_of_trade: "SELL".to_string(),
            events: Arc::new(Vec::new()),
            events_number: 0,
            string_best_profit_sell: "".to_string(),
            string_best_profit_buy: "".to_string(),
            // BOT MODE
            safe_mode: true,
            logs: vector![],
            logs_bot: vector![],
            bfb_logs_bot: vector![],
            sol_logs_bot: vector![],
            parse_logs_bot: vector![],
            trader_bot: TraderBot::new(
                "TSE TRADER".to_string(),
                1000.00,
                sol::new_random(),
                bfb::new_random(),
                parse::new_random(),
            ),
            logs_arb: vector![],
            bfb_logs_arb: vector![],
            sol_logs_arb: vector![],
            parse_logs_arb: vector![],
            trader_arb: TraderBot::new(
                "TSE TRADER".to_string(),
                1000.00,
                sol::new_random(),
                bfb::new_random(),
                parse::new_random(),
            ),
            percentage_bot: 1.0,
        }
    }
}

/// SUPPORT FUNCTIONS FOR THE TraderUi STRUCT - initializer of the TraderUi
/// struct. This function is called when the TraderUi is created and it
/// initializes the TraderUi struct datas when the program starts
///
/// **Federico Brancasi**
pub(crate) fn initialize_quantities(app: &mut TraderUi) -> &mut TraderUi {

    // USER MODE
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

    app.trader.goods = vector![100000.0, 100000.0, 1000000.0, 0.0];

    app.quantity = app.trader.goods[0];

    app.string_best_profit_buy = get_best_buy_trade(&app.markets, app.trader.goods[0]);
    app.string_best_profit_sell = get_best_sell_trade(&app.markets, &app.trader.goods);


    // BOT MODE
    // let (mut sol, mut parse, mut bfb) = random_init();
    //
    // //initialize the trader
    // let mut traderbot = TraderBot::new(
    //     "TSE TRADER".to_string(),
    //     1000.00,
    //     sol.clone(),
    //     bfb.clone(),
    //     parse.clone(),
    // );
    //
    //
    // app.logs_bot = bot(&mut traderbot, 1000);
    // println!("start logs andrea\n");
    // for elem in app.logs_bot.iter() {
    //     println!("elem: {:?}", elem);
    // }
    // println!("\nlengt logs andrea: {}", app.logs_bot.len());
    // println!("end logs andrea\n");
    //
    //
    // app.logs_arb = arbitrage(&mut traderbot, 1000);
    // println!("start logs lorenzo\n");
    // for elem in app.logs_arb.iter() {
    //     println!("elem: {:?}", elem);
    // }
    // println!("\nlengt logs lorenzo: {}", app.logs_arb.len());
    // println!("end logs andrea\n");
    //
    //
    // // CREAZIONE LOGS PER SAFE MODE SE NON VA BOT ANDREA
    // // let mut counter = 0;
    // // for i in 0..1000 {
    // //     if counter == 0 {
    // //         app.logs_bot.push_back("BFB HA FATTO LA COMPERA".to_string() + "    safe mode bro");
    // //         counter = 1;
    // //     } else if counter == 1 {
    // //         app.logs_bot.push_back("SOL HA FATTO LA VENDITA".to_string() + "    safe mode bro");
    // //         counter = 2;
    // //     } else {
    // //         app.logs_bot.push_back("PARSE HA FATTO LA COMPERA".to_string() + "  safe mode bro");
    // //         counter = 0;
    // //     }
    // // }
    //
    // for elem in app.logs_bot.iter() {
    //     if elem.as_str().contains("BFB") {
    //         app.bfb_logs_bot.push_back(elem.to_string());
    //     } else if elem.as_str().contains("SOL") {
    //         app.sol_logs_bot.push_back(elem.to_string());
    //     } else {
    //         app.parse_logs_bot.push_back(elem.to_string());
    //     }
    // }
    //
    // // CREAZIONE LOGS PER UNSAFE MODE SE NON VA BOT LORENZO
    // // let mut counter = 0;
    // // for i in 0..1000 {
    // //     if counter == 0 {
    // //         app.logs_arb.push_back("BFB HA FATTO LA COMPERA".to_string() + "    unsafe mode bro");
    // //         counter = 1;
    // //     } else if counter == 1 {
    // //         app.logs_arb.push_back("SOL HA FATTO LA VENDITA".to_string() + "    unsafe mode bro");
    // //         counter = 2;
    // //     } else {
    // //         app.logs_arb.push_back("PARSE HA FATTO LA COMPERA".to_string() + "  unsafe mode bro");
    // //         counter = 0;
    // //     }
    // // }
    //
    // for elem in app.logs_arb.iter() {
    //     if elem.as_str().contains("BFB") {
    //         app.bfb_logs_arb.push_back(elem.to_string());
    //     } else if elem.as_str().contains("SOL") {
    //         app.sol_logs_arb.push_back(elem.to_string());
    //     } else {
    //         app.parse_logs_arb.push_back(elem.to_string());
    //     }
    // }

    app
}