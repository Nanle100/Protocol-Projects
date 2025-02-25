
pub mod arithmetics;
pub mod bitwise;
pub mod control;
pub mod environment;
pub mod stack;

// pub use arithmetics::{add, mul, sub, div};
// pub use control::stop;
// pub use stack::{push1, dup1, swap1};

pub use arithmetics::{add, mul, sub, div};
pub use control::stop;
pub use stack::{push1, dup1, swap1};