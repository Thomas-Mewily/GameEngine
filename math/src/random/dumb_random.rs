use crate::*;


/// NOT APPROPRIATE FOR CRYPTOGRAPHIC USE
/// 
/// A floatly simple and **dumb** random for the moment. Will be changed in the futur if needed
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DumbRandom
{
    seed: Wrapping<u64>,
}
impl Default for DumbRandom { fn default() -> Self { Self::new(123456789123456789) }}

impl IRandom for DumbRandom
{
    fn new(seed : u64) -> Self { Self { seed: Wrapping(seed) }}

    fn u64(&mut self) -> u64 {
        let mut x = self.seed;
        x ^= x << 21;
        x ^= x >> 35;
        x ^= x << 4;
        self.seed = x;
        x.0
    }
}