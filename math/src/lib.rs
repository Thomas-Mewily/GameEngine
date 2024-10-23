//! A Math library programmed by Thomas Mewily
//! 
//! Allow you to manipulate 2d, 3d and 4d point of any type (float, int, uint, or even user defined)
//! Support rectangle and grid for any dimension
//! 
//! Thank to Simple Easing function to https://docs.rs/simple-easing/latest/simple_easing/ for the easing function under the MIT License
//! 
//! Thank to https://easings.net/# for some common easing function, and the nice visualisation

#![allow(unused_imports)]

use util::*;

pub mod math_core;
pub use math_core::*;

pub mod various;
pub use various::*;

pub mod tween;
pub use tween::*;

pub mod random;
pub use random::*;