//! Magical Mathematics

mod add;
mod sub;

pub mod operations {
    pub use super::add::enchant as add;
    pub use super::sub::enchant as sub;
}
