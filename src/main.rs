mod match_engine;
use match_engine::engine::{MatchingEngine, TradingPair};
use match_engine::orderbook::{Order, OrderBook, BidOrAsk};
use rust_decimal_macros::dec;
fn main() {
    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    let mut orderbook = OrderBook::new();  
    orderbook.add_limit_order(dec!(4.4), buy_order);
    //println!("{:?}", orderbook);
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());
    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    engine.place_limit_order(pair, dec!(10000), buy_order).unwrap();  
}