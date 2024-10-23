use crate::*;

/// Similar to Take
pub trait Consumable : Default
{
    fn consume(&mut self) -> Self { std::mem::take(self) }
}
impl<T : Default> Consumable for T {}

pub trait RevConsumable : Reversable
{
    /// Return the old value and return it, while reversing the original value
    fn consume_and_reverse(&mut self) -> Self { let old = *self; *self = self.reverse(); old }
}
impl<T : Reversable> RevConsumable for T {}