//! Rust 2024 !!!
//! This is a high-performance crate for getting market data.
//! It uses websocket and rest to ensure that you get data safely.
//! traits.rs defines the public interface of this crate.
//! stock and prypto are methods for stocks and cryptocurrencies 
//! such as exchanges and some company data.

pub mod stock;

pub mod crypto;

pub mod handler;

pub mod websocket;

pub mod rest_client;

pub mod traits;


pub trait MarketStream {}

pub trait MarketHandler {}

// 
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MarketOrder {
    order_id: String,
    order_price: f64,
    order_quantity: f64,
    order_time: String,
    order_types: MarketOrderTypes,
    order_status: MarketOrderStates,
}

impl MarketOrder {
    pub fn new(order_id: &str, order_price: i64, order_quantity: i64,
                order_time: &str, order_types: MarketOrderTypes,
                order_status: MarketOrderStates, 
    ) -> Self {
        Self {
            order_id: order_id.to_string(),
            order_price: 0.0,
            order_quantity: 0.0,
            order_time: "".to_owned(),
            order_types,
            order_status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MarketOrderTypes {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MarketOrderStates {}

pub enum MarketError {
    Io(std::io::Error),
    Parse(String),
    Stream(String),
    UnConnect,
    Unexpected(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
