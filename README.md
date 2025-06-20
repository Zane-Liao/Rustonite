# Rustonite
- This is a framework focusing on quantitative trading in the field of cryptocurrency. It supports mainstream exchanges and currencies on the market, is extremely fast, and lightweight.
- It uses Rust to write the core trading components. I regularly use benchmarks to test the speed and usability and compare it with other mainstream platforms.
- For more information, please refer to docs/...(This includes not only future project documentation and APIs, but also my design ideas and framework, and some thoughts.)
- At present, this project is just a starting point, and there are still many things to be completed.
- Welcome to put forward any valuable suggestions, your ideas are the basis for making this framework better.
- Questions and contributions are welcome, please contact me if you have any questions.




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

pub mod analysis {
    pub mod traits;
}