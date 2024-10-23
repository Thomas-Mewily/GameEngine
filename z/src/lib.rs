//! A small video game framework based on MacroQuad
//! Support for rendering, playing sound, getting inputs

#![allow(unused_imports)]
#![allow(dead_code)]

pub use macroquad_macro::main;
pub use macroquad;

pub use math::*;
pub use util::*;

pub mod context;
pub use context::*;

pub mod wrapper;
pub use wrapper::*;

pub mod diagnostic;
pub use diagnostic::*;