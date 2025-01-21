pub mod core;
pub mod interface;
pub mod bridge;

pub use crate::core::Lazuline;

pub mod prelude {
    pub use crate::core::Lazuline;
}

pub fn init() -> Result<Lazuline, Box<dyn std::error::Error>> {
    Lazuline::new()
}
