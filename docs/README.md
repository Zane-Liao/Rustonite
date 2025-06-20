# RUSTONITE-QUANT

- RUSTONITE-QUANT is a high-performance trading core framework built in Rust, designed specifically for the cryptocurrency domain. It focuses on **data ingestion**, **order execution**, **risk control**, and **data storage**. Within a full-fledged quantitative trading architecture, it serves as the **foundational infrastructure**, distinct from strategy engines or backtesting modules.

- It uses **[PyO3](https://github.com/PyO3/pyo3)** to provide seamless Rust–Python bindings, ensuring ease of use from Python while retaining Rust's performance. If you're not familiar with calling Rust APIs directly — don’t worry. The framework will provide clear, well-documented Python interfaces, and the API design will remain minimal and intuitive.

- A complete quantitative trading framework typically consists of three core modules: **Data**, **Strategy & Backtesting**, and **Realtime Trading**. RUSTONITE provides the **infrastructure layer**, focusing specifically on the **Data** and **Realtime Trading** modules.

- The goal of RUSTONITE is not to reinvent strategy or backtesting engines. Instead, it aims to provide a robust, high-performance foundation for **data processing**, **order execution**, and **risk control** — all of which are crucial in today’s highly dynamic and latency-sensitive cryptocurrency markets.

- Most existing quantitative trading frameworks suffer from three key limitations: **limited performance**, **over-simplified design**, and **poor scalability**. These shortcomings are particularly evident in real-world cryptocurrency trading scenarios, where low latency, high concurrency, and even high-frequency execution are essential for achieving consistent performance.

- Below is a rough outline of the project documentation. Each section provides an overview of different aspects of the framework. For detailed explanations, please refer to the corresponding documents.


📚 Documentation Overview

- `README.md` — Project introduction, goals, and quick start guide
- `architecture.md` — System architecture and module overview
- `design.md` — Design principles, technical decisions, and trade-offs
- `api/` — Detailed API reference and usage examples (Rust & Python)
- `deployment/` — Setup, deployment, and configuration guides
- `strategies/` — How to integrate or connect with external strategy engines