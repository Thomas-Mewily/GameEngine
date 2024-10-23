pub mod random;
pub use random::*;

pub mod dumb_random;

pub type Random = dumb_random::DumbRandom;
