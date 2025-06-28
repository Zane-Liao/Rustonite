use std::io::Error;
use crate::MarketStream;

// Live Subscribing/Unsubscribing to streams
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubScribingsStreams {
    result: bool,
    id: u32,
}

// Aggregate Trade Streams
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TradeStreams {
    event_type: String,
    event_time: i64,
    symbol: String,
    aggregate_trade_id: i64,
    price: String,
    quantity: String,
    first_trade_id: i64,
    last_trade_id: i64,
    trade_time: i64,
    market_maker: bool,
}

