//! PiCode CLI - Command-line interface components

pub mod args;
pub mod commands;

pub use args::*;
pub use commands::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}