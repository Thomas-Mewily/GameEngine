use super::*;

/// A 2D dimension circle
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Circle2
{
    pub pos : Vec2,
    pub radius : float,
}

impl Circle2
{
    pub fn new(pos : Vec2, radius : float) -> Self { Self { pos, radius }}

    pub fn area(&self) -> float { float::PI * self.radius * self.radius }
    pub fn perimeter(&self) -> float { 2. * float::PI * self.radius }
}