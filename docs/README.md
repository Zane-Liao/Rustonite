# RUSTONITE-QUANT

- This is a Rust high-performance trading core framework, focusing on the cryptocurrency field. Its main functions include data collection, order execution and risk control, and data storage. It is a core in the cryptocurrency field. It belongs to a large category of modules in the trading framework, and other modules include strategies and backtesting.
- It uses pyo3 and Rust for binding and encapsulation, so you don’t have to worry too much about the ease of use of the framework (if you want to call the Rust API directly and don’t know how to do it, don’t worry, I will gradually improve the API documentation later, and the project’s API will be very concise and easy to use)
- I roughly divide a complete quantitative trading framework into three modules: data module, strategy backtesting module, and real-time module. RUSTONITE belongs to the infrastructure, which focuses on the data module and the real-time module.
- Its goal is not to build a backtest or strategy engine, its goal is to help you with data processing, orders and risk control, which has become very important in today's cryptocurrency field.
- Most of the quantitative trading frameworks available today have the four acute problems I mentioned: insufficient performance, simple design, and poor scalability. When encountering these problems in real trading, especially in the field of cryptocurrency, without low latency and high concurrency, or even high frequency, we sometimes cannot achieve the desired trading results.
- I will mark the general contents of the document directory below, as well as what they are specifically introduced and what they can do for you (just a rough outline, you need to read the document in detail for details)

README.md --
architecture.md -- 
design.md --
api/ --
deployment/ --
strategies/ -- 