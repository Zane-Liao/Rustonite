//! Rust 2024 !!!

pub mod market_data {
    pub mod collector;
    pub mod websockets;
    pub mod normalizer;
    pub mod boardcaster;
    pub mod feeds;
}

pub mod execution {
    pub mod traits;
}

pub mod risk {
    pub mod traits;
}

pub mod storage {
    pub mod traits;
}

pub mod utils {
    pub mod traits;
}