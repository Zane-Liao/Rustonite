//! Rust 2024 !!!
//! 

pub mod feeds;

pub mod handler;

pub mod websockets;

pub mod rest_client;

pub mod traits;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn mult(left: i64, right: i64) -> i64 {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
