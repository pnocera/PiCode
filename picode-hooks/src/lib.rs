//! PiCode Hooks - Configurable hook system

pub mod hooks;
pub mod registry;

pub use hooks::*;
pub use registry::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}