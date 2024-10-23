// File related to float
use crate::*;

#[allow(non_camel_case_types)]
pub type float = f32;
//pub type float = f64;

//pub const FLOAT_PI        : float = float::PI;
//pub const FLOAT_FRAC_PI_2 : float = float::PI / 2.;

pub trait FloatConstant { const PI : Self; }
macro_rules! impl_float_constant{
    ($primitive_name: ident) => 
    { impl FloatConstant for $primitive_name { const PI : Self =  std::$primitive_name::consts::PI; } };
}
map_on_floating!(impl_float_constant);


/// Same as float, but just to mark the different use case.
/// Generally, a coef is between `[0.0, 1.0]`
pub type Coef = float;

/// Iter from `[0.0, 1.0]`. Always start at 0
pub struct CoefIterator
{
    pub nb_step  : uint,
    pub div      : float,
    pub i        : uint,
}
impl Iterator for CoefIterator
{
    type Item=Coef;

    fn next(&mut self) -> Option<Self::Item> 
    {
        let old_i = self.i;
        self.i += 1;
        if old_i >= self.nb_step { None } else { Some(old_i.to_float() / self.div) }
    }
}
impl CoefIterator 
{
    /// `[0.0, 1.0[`
    pub fn new_start_at_zero_excluded(nb_step : uint) -> Self { Self::new_full(nb_step, 0, nb_step.to_float().max(1.) )}
    /// `[0.0, 1.0?]`
    pub fn new_start_at_zero(nb_step : uint) -> Self { Self::new_full(nb_step, 0, (nb_step - 1).to_float().max(1.) )}
    /// `[0.0?, 1.0]`
    pub fn new_finish_at_one(nb_step : uint) -> Self 
    { 
        if nb_step <= 1
        {
            Self::new_full(2, 1, 1.0)
        }else
        {
            Self::new_start_at_zero(nb_step)
        }
    }
    pub fn new_full(nb_step : uint, i : uint, div : float) -> Self { Self { nb_step, div, i }}
}

pub trait CoefIter : Sized
{ 
    /// From `[0.0?, 1.0]`. Always finish with 1
    fn iter_coef_finish_at_one(self) -> CoefIterator; 

    /// From `[0.0, 1.0?]`. Always start with 0
    fn iter_coef_start_at_zero(self) -> CoefIterator;

    /// From `[0.0, 1.0[`. Always start with 0
    fn iter_coef(self) -> CoefIterator;

    /// From `]0.0, 1.0[`
    fn iter_coef_skip_zero(self) -> impl Iterator<Item = float>;
    fn iter_coef_skip_zero_finish_at_one(self) -> impl Iterator<Item = float>;
}
impl<T> CoefIter for T where T : ToUint + HaveOne + Add<Self, Output = Self>
{
    fn iter_coef_finish_at_one(self) -> CoefIterator { CoefIterator::new_finish_at_one(self.to_uint()) }
    fn iter_coef_start_at_zero(self) -> CoefIterator { CoefIterator::new_start_at_zero(self.to_uint()) }
    
    fn iter_coef(self) -> CoefIterator { CoefIterator::new_start_at_zero_excluded(self.to_uint()) }
    fn iter_coef_skip_zero(self) -> impl Iterator<Item = float> { (self + Self::ONE).iter_coef().skip(1) }
    fn iter_coef_skip_zero_finish_at_one(self) -> impl Iterator<Item = float> { (self + Self::ONE).iter_coef_finish_at_one().skip(1) }
}
/* 
macro_rules! impl_coef_iter{
    ($primitive_name: ty) => 
    { impl CoefIter for $primitive_name { fn iter_coef(self) -> CoefIterator { CoefIterator::new(self.to_uint()) }} };
}
map_on_integer!(impl_coef_iter);*/