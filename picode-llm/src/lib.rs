//! PiCode LLM - Large Language Model integrations

pub mod client;
pub mod providers;
pub mod openapi;

pub use client::*;
pub use providers::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}