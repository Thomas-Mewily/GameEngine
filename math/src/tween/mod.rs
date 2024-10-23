use super::*;

pub mod tween;
pub use tween::*;

pub mod easing;
pub use easing::*;

/// Thank to [the simple_easing crate](https://docs.rs/simple-easing/latest/simple_easing/) crate for the easing function
/// I needed to edit it because I work with `float` and not `f32` 
pub mod easing_fn;
