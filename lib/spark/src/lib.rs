//! Spark Standard Library - Where Magic Begins ✨

#![feature(const_type_name)]

pub mod math;
pub mod types;
pub mod align;
pub mod any;
pub mod shard;
pub mod array;

pub use types::*;
pub use math::operations;
pub use align::space;
pub use shard::arch;
pub use array::{CrystalArray, ArrayOps};
