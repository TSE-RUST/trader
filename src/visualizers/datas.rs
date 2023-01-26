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
use crate::visualizers::events::{LoggedEvent};

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
    pub logs: Vector<(String, String, String, Vector<f32>)>,
    pub buy_or_sell_string: Vector<String>,
    pub goodkinds_string: Vector<String>,
    pub quantity_string: Vector<String>,
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
            string_best_profit_sell: "sdrumpx morto".to_string(),
            string_best_profit_buy: "stypox figlio di puttana".to_string(),
            // BOT MODE
            safe_mode: false,
            logs: vector![],
            buy_or_sell_string: vector![],
            goodkinds_string: vector![],
            quantity_string: vector![],
            percentage_bot: 1.0,
        }
    }
}