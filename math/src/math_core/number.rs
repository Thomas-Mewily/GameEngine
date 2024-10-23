use crate::*;

/// For type that have an unsigned equivalent
pub trait ToUnsigned
{
    /// The unsigned equivalence
    type UnsignedEquivalent : Copy + ToSigned;
    fn to_unsigned(self) -> Self::UnsignedEquivalent;
}
macro_rules! impl_signed
{
    ($primitive_name: ty, $unsigned_primitive_name: ty) => 
    { 
        impl ToUnsigned for $primitive_name  { type UnsignedEquivalent=$unsigned_primitive_name; #[inline] fn to_unsigned(self) -> Self::UnsignedEquivalent  {self as Self::UnsignedEquivalent }}
    };

    ($primitive_name: ty) => 
    { 
        impl ToUnsigned for $primitive_name  { type UnsignedEquivalent=$primitive_name; #[inline] fn to_unsigned(self) -> Self::UnsignedEquivalent  { self }}
    };
}
impl_signed!(i8 , u8);
impl_signed!(i16, u16);
impl_signed!(i32, u32);
impl_signed!(i64, u64);
impl_signed!(isize, usize);
map_on_unsigned!(impl_signed);
map_on_floating!(impl_signed);

/// For type that have a signed equivalent
pub trait ToSigned
{
    /// The signed equivalence
    type SignedEquivalent : Copy + ToUnsigned;
    fn to_signed(self) -> Self::SignedEquivalent;
}
macro_rules! impl_unsigned
{
    ($primitive_name: ty, $signed_primitive_name: ty) => 
    { 
        impl ToSigned for $primitive_name { type SignedEquivalent=$signed_primitive_name; #[inline] fn to_signed(self) -> Self::SignedEquivalent { self as Self::SignedEquivalent  }}
    };

    ($primitive_name: ty) => 
    { 
        impl ToSigned for $primitive_name  { type SignedEquivalent=$primitive_name; #[inline] fn to_signed(self) -> Self::SignedEquivalent  { self }}
    };
}
impl_unsigned!(u8 , i8);
impl_unsigned!(u16, i16);
impl_unsigned!(u32, i32);
impl_unsigned!(u64, i64);
impl_unsigned!(usize, isize);
map_on_signed!(impl_unsigned);
map_on_floating!(impl_unsigned);



pub trait Lerpable : Mul<float,Output=Self> + Add<Self,Output=Self> + Copy
{
    /// Not restricted between [0..1]
    fn lerp_unrestricted(self, dest : Self, coef : Coef) -> Self { self * (1. - coef) + dest * coef  }

    /// Restricted between [0..1]
    fn lerp(self, dest : Self, coef : Coef) -> Self { self.lerp_unrestricted(dest, coef.min(1.).max(0.)) }

    /// Restricted between [0..1]
    fn ease(self, dest : Self, coef : Coef, easing : Easing) -> Self { self.lerp(dest, easing.apply(coef)) }

    /// Not restricted between [0..1]
    fn ease_unrestricted(self, dest : Self, coef : Coef, easing : Easing) -> Self { self.lerp_unrestricted(dest, easing.apply(coef)) }
}
impl<T : Mul<float,Output=T> + Add<T,Output=T> + Copy> Lerpable for T {}


pub trait Percent  { fn percent(self) -> Coef; }
impl<T : ToFloat + Div<float>> Percent for T
{
    fn percent(self) -> Coef {
        self.to_float() / 100.
    }
}


pub trait Absolute { fn abs(self) -> Self; }
macro_rules! impl_abs_is_itself {
    ($primitive_name: ty) => 
    { impl Absolute for $primitive_name { #[inline] fn abs(self) -> Self { self }} };
}
map_on_unsigned!(impl_abs_is_itself);

macro_rules! impl_abs {
    ($primitive_name: ty) => 
    { impl Absolute for $primitive_name { #[inline] fn abs(self) -> Self { Self::abs(self) }} };
}
map_on_signed!(impl_abs);
map_on_floating!(impl_abs);


pub trait Rounding
{ 
    fn floor(self) -> Self; 
    fn round(self) -> Self; 
    fn ceil (self) -> Self;

    fn trunc(self) -> Self;

    fn round_toward_from_zero(self) -> Self where Self : FloatingNumber { if self.is_positive() { self.floor() } else { self.ceil() }}
    fn round_away_from_zero(self) -> Self where Self : FloatingNumber { if self.is_positive() { self.ceil() } else { self.floor() }}
}
macro_rules! impl_rounding {
    ($primitive_name: ty) => 
    { 
        impl Rounding for $primitive_name 
        { 
            #[inline] fn floor(self) -> Self { Self::floor(self) }
            #[inline] fn ceil (self) -> Self { Self::ceil(self) }
            #[inline] fn round(self) -> Self { Self::round(self) }
            #[inline] fn trunc(self) -> Self { Self::trunc(self) }
        } 
    };
}
map_on_floating!(impl_rounding);

macro_rules! impl_rounding_is_itself {
    ($primitive_name: ty) => 
    { 
        impl Rounding for $primitive_name 
        { 
            #[inline] fn floor(self) -> Self { self }
            #[inline] fn ceil (self) -> Self { self }
            #[inline] fn round(self) -> Self { self }
            #[inline] fn trunc(self) -> Self { self }
        } 
    };
}
map_on_integer!(impl_rounding_is_itself);



/// Convert the number to an uint
pub trait ToUint { fn to_uint(self) -> uint; }
macro_rules! impl_to_uint {
    ($primitive_name: ty) => 
    { impl ToUint for $primitive_name { #[inline] fn to_uint(self) -> uint { self as uint }} };
}
map_on_scalar!(impl_to_uint);
impl ToUint for bool { fn to_uint(self) -> uint { if self { 1 } else { 0 } }}


/// Convert the number to an int
pub trait ToInt { fn to_int(self) -> int; }
macro_rules! impl_to_int {
    ($primitive_name: ty) => 
    { impl ToInt for $primitive_name { #[inline] fn to_int(self) -> int { self as int }} };
}
map_on_scalar!(impl_to_int);
impl ToInt for bool { fn to_int(self) -> int { if self { 1 } else { 0 } }}



/// Convert the number to a float
pub trait ToFloat { fn to_float(self) -> float; }
macro_rules! impl_to_float {
    ($primitive_name: ty) => 
    { impl ToFloat for $primitive_name { #[inline] fn to_float(self) -> float { self as float }} };
}
map_on_scalar!(impl_to_float);
impl ToFloat for bool { fn to_float(self) -> float { if self { 1.0 } else { 0.0 } }}



pub trait FromFloat   { fn from_float(val : float) -> Self; }
macro_rules! impl_from_float {
    ($primitive_name: ty) => 
    { 
        impl FromFloat for $primitive_name 
        { #[inline] fn from_float(val : float) -> Self { val as Self }}
    }
}
map_on_scalar!(impl_from_float);
impl FromFloat for bool { fn from_float(val : float) -> Self { val >= 0.5 }}



pub trait FromUint  : Sized
{ 
    fn from_uint (val : uint) -> Self { Self::from_usize(val) }
    fn from_usize(val : usize) -> Self;

    fn from_u64(val : u64) -> Self;
    fn from_u32(val : u32) -> Self { Self::from_u64(val as u64) }
}
macro_rules! impl_from_uint {
    ($primitive_name: ty) => 
    { 
        impl FromUint for $primitive_name 
        { 
            #[inline] fn from_usize(val : uint) -> Self { val as Self }
            #[inline] fn from_u64(val : u64) -> Self { val as Self }
        }
    }
}
map_on_scalar!(impl_from_uint);
impl FromUint for bool 
{ 
    fn from_usize(val : usize) -> Self { val >= 1 }
    fn from_u64(val : u64) -> Self { val >= 1 }
}


pub trait FromInt : Sized
{ 
    fn from_int(val : int) -> Self { Self::from_isize(val) }
    fn from_isize(val : isize) -> Self;

    fn from_i64(val : i64) -> Self;
    fn from_i32(val : i32) -> Self { Self::from_i64(val as i64) }
}
macro_rules! impl_from_int {
    ($primitive_name: ty) => 
    { 
        impl FromInt for $primitive_name 
        { 
            #[inline] fn from_isize(val : isize) -> Self { val as Self }
            #[inline] fn from_i64(val : i64) -> Self { val as Self }
        }
    }
}
map_on_scalar!(impl_from_int);
impl FromInt for bool 
{ 
    fn from_isize(val : isize) -> Self { val >= 1 }
    fn from_i64(val : i64) -> Self { val >= 1 }
}

/// Define the `0` representation
pub trait HaveZero
{ 
    const ZERO : Self;
    fn is_zero(self) -> bool where Self : PartialEq<Self> + Copy { self == Self::ZERO }
    fn is_non_zero(&self) -> bool where Self : PartialEq<Self> + Copy { !self.is_zero() }
}
macro_rules! impl_have_zero {
    ($primitive_name: ty) => 
    { impl HaveZero for $primitive_name { const ZERO : Self = 0 as Self; } };
}
map_on_scalar!(impl_have_zero);
impl HaveZero for bool { const ZERO : Self = false; }
pub const fn zero<T : HaveZero>() -> T { T::ZERO }

/// Define the half `0.5` representation
pub trait HaveHalf  
{ 
    const HALF : Self;
    fn half(self) -> Self where Self : Mul<Self,Output=Self> + Copy { self * Self::HALF }
    fn is_half(&self) -> bool where Self : PartialEq<Self> + Copy { *self == Self::HALF }
    fn is_non_half(&self) -> bool where Self : PartialEq<Self> + Copy { !self.is_half() }
}
macro_rules! impl_have_half {
    ($primitive_name: ty) => 
    { impl HaveHalf for $primitive_name { const HALF : Self = 0.5 as Self; } };
}
map_on_scalar!(impl_have_half);
pub const fn half<T : HaveHalf>() -> T { T::HALF }

/// Define the `1` representation for the number
pub trait HaveOne  
{ 
    const ONE  : Self; 
    fn is_one(self) -> bool where Self : PartialEq<Self> + Copy { self == Self::ONE }
    fn is_non_one(&self) -> bool where Self : PartialEq<Self> + Copy { !self.is_one() }
}
macro_rules! impl_have_one {
    ($primitive_name: ty) => 
    { impl HaveOne for $primitive_name { const ONE : Self = 1 as Self; } };
}
map_on_scalar!(impl_have_one);
impl HaveOne for bool { const ONE  : Self = true; }
pub const fn one<T : HaveOne>() -> T { T::ONE }


/// Define the `-1` representation for the number
pub trait HaveMinusOne  
{ 
    const MINUS_ONE  : Self;
    fn is_minus_one(self) -> bool where Self : PartialEq<Self> + Copy { self == Self::MINUS_ONE }
    fn is_non_minus_one(&self) -> bool where Self : PartialEq<Self> + Copy { !self.is_minus_one() }
}
macro_rules! impl_have_minus_one {
    ($primitive_name: ty) => 
    { impl HaveMinusOne for $primitive_name { const MINUS_ONE : Self = -1 as Self; } };
}
map_on_floating!(impl_have_minus_one);
map_on_signed!(impl_have_minus_one);
pub const fn minus_one<T : HaveMinusOne>() -> T { T::MINUS_ONE }



/// Define the smallest reasonable increment. `1` for integer, 0 for float
pub trait SmallestIncrement  { const SMALL_INC  : Self; }
macro_rules! impl_smallest_inc_integer {
    ($primitive_name: ty) => 
    { impl SmallestIncrement for $primitive_name { const SMALL_INC : Self = Self::ONE; } };
}
map_on_integer!(impl_smallest_inc_integer);
macro_rules! impl_smallest_inc_float {
    ($primitive_name: ty) => 
    { impl SmallestIncrement for $primitive_name { const SMALL_INC : Self = Self::ZERO; } };
}
map_on_floating!(impl_smallest_inc_float);


pub trait PositiveOrNegative : HaveZero + PartialOrd<Self> + Sized
{ 
    /// Is >= 0
    /// ```
    /// use math::*;
    /// debug_assert_eq!((-1).is_positive(), false);
    /// debug_assert_eq!(   0.is_positive(), true);
    /// debug_assert_eq!(   1.is_positive(), true);
    /// ```
    fn is_positive(&self) -> bool { self >= &Self::ZERO }


    /// Is <= 0
    /// ```
    /// use math::*;
    /// debug_assert_eq!((-1).is_negative(), true);
    /// debug_assert_eq!(   0.is_negative(), true);
    /// debug_assert_eq!(   1.is_negative(), false);
    /// ```
    fn is_negative(&self) -> bool { self <= &Self::ZERO }

    /// Is > 0
    /// ```
    /// use math::*;
    /// debug_assert_eq!((-1).is_strictly_positive(), false);
    /// debug_assert_eq!(   0.is_strictly_positive(), false);
    /// debug_assert_eq!(   1.is_strictly_positive(), true);
    /// ```
    fn is_strictly_positive(&self) -> bool { self > &Self::ZERO }

    /// Is < 0
    /// ```
    /// use math::*;
    /// debug_assert_eq!((-1).is_strictly_negative(), true);
    /// debug_assert_eq!( 0.is_strictly_negative(), false);
    /// debug_assert_eq!( 1.is_strictly_negative(), false);
    /// ```
    fn is_strictly_negative(&self) -> bool { self < &Self::ZERO }
}
impl<T : HaveZero + PartialOrd<T> + Sized> PositiveOrNegative for T {}


/// For floating point only : `f32`, `f64` currently
pub trait FloatingNumber : HaveMinusOne + HaveZero + HaveHalf + HaveOne + Rounding + Scalar // + HaveHalf + Rounding
{ 
    fn sqrt(self) -> Self;

    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn tan(self) -> Self;

    fn atan2(self, other : Self) -> Self;

    /* 
    fn from_usize(val : usize) -> Self;
    fn from_isize(val : isize) -> Self;

    fn from_u64(val : u64) -> Self;
    fn from_i64(val : i64) -> Self;

    fn from_u32(val : u32) -> Self;
    fn from_i32(val : i32) -> Self;
    */

    /// between [0., 1.]
    fn normalize(self) -> Self { self.min_partial(Self::ONE).max_partial(Self::ZERO) }

    // add more if needed
}
macro_rules! impl_floating_number {
    ($primitive_name: ty) => 
    { 
        impl FloatingNumber for $primitive_name 
        { 
            fn sqrt(self) -> Self { self.sqrt() }
            fn cos(self) -> Self { self.cos() }
            fn sin(self) -> Self { self.sin() }
            fn tan(self) -> Self { self.tan() }

            fn atan2(self, other : Self) -> Self { self.atan2(other) }
            /* 
            fn from_usize(val : usize) -> Self { val as Self }
            fn from_isize(val : isize) -> Self { val as Self }
            fn from_u64(val : u64) -> Self { val as Self }
            fn from_i64(val : i64) -> Self { val as Self }
            fn from_u32(val : u32) -> Self { val as Self }
            fn from_i32(val : i32) -> Self { val as Self }*/
        }
    };
}
map_on_floating!(impl_floating_number);

pub trait BitArithmetic : 
    Sized +
    Shl<Self, Output=Self> + ShlAssign<Self> +
    Shr<Self, Output=Self>  + ShlAssign<Self> +
    BitOr<Self, Output=Self> + BitOrAssign<Self> +
    BitAnd<Self, Output=Self> + BitAndAssign<Self> + 
    BitXor<Self, Output=Self> + BitXorAssign<Self> +
    Not<Output = Self>
{}
impl<T> BitArithmetic for T
    where T :
        Sized +
        Shl<Self, Output=Self> + ShlAssign<Self> +
        Shr<Self, Output=Self>  + ShlAssign<Self> +
        BitOr<Self, Output=Self> + BitOrAssign<Self> +
        BitAnd<Self, Output=Self> + BitAndAssign<Self> + 
        BitXor<Self, Output=Self> + BitXorAssign<Self> +
        Not<Output = Self>
{}

/// For unsigned integer : `u8`, `u16`, `u32`, `u64`, `usize`
pub trait UnsignedInteger : Eq + Ord + UnitArithmeticPositive + BitArithmetic
{
    fn count_ones(self) -> u32;
    fn count_zeros(self) -> u32;
}
macro_rules! impl_unsigned_integer 
{
    ($primitive_name: ty) => 
    { 
        impl UnsignedInteger for $primitive_name 
        {
            fn count_ones(self) -> u32 { Self::count_ones(self) }
            fn count_zeros(self) -> u32 { Self::count_zeros(self) }
        }
    };
}
map_on_unsigned!(impl_unsigned_integer);


/// For signed integer : `i8`, `i16`, `i32`, `i64`, `isize`
pub trait SignedInteger : Eq + Ord + UnitArithmeticPositive {}
macro_rules! impl_signed_integer {
    ($primitive_name: ty) => 
    { impl SignedInteger for $primitive_name {} };
}
map_on_signed!(impl_signed_integer);


pub trait HaveMinMaxValue { const MIN_VAL : Self; const MAX_VAL : Self; }
macro_rules! impl_have_min_max_value {
    ($primitive_name: ty) => 
    { impl HaveMinMaxValue for $primitive_name 
        { 
            const MIN_VAL : Self = Self::MIN;
            const MAX_VAL : Self = Self::MAX;
        }
    };
}
map_on_scalar!(impl_have_min_max_value);


/// For floating the range is : `[0., 1.]`. For integers the range is : `[0, MAX]`, even for signed
pub trait HaveDefaultRange : HaveMinMaxValue + ToFloat + FromFloat + UnitArithmeticPositive
{ 
    const MIN_RANGE_VAL : Self;
    const MAX_RANGE_VAL : Self;
    const RANGE : Self;

    #[inline] fn to_coef(self) -> Coef { (self - Self::MIN_RANGE_VAL).to_float() / Self::RANGE.to_float()  }
    /// based on the HaveDefaultRange trait
    #[inline] fn from_coef(c : Coef) -> Self { Self::from_float(Self::RANGE.to_float() * c + Self::MIN_RANGE_VAL.to_float()) }
}
macro_rules! impl_have_min_max {
    ($primitive_name: ty) => 
    { impl HaveDefaultRange for $primitive_name 
        { 
            const MIN_RANGE_VAL : Self = Self::ZERO;
            const MAX_RANGE_VAL : Self = Self::MAX_VAL;
            const RANGE         : Self = Self::MAX_RANGE_VAL - Self::MIN_RANGE_VAL;
        }
    };
}
map_on_integer!(impl_have_min_max);

macro_rules! impl_have_min_max_for_float {
    ($primitive_name: ty) => 
    { impl HaveDefaultRange for $primitive_name 
        { 
            const MIN_RANGE_VAL : Self = Self::ZERO;
            const MAX_RANGE_VAL : Self = Self::ONE;
            const RANGE         : Self = Self::MAX_RANGE_VAL - Self::MIN_RANGE_VAL;
        }
    };
}
map_on_floating!(impl_have_min_max_for_float);


/// For signed and unsigned integer : (`u8`, `u16`, `u32`, `u64`, `usize`) + (`i8`, `i16`, `i32`, `i64`, `isize`)
pub trait Integer : Eq + Ord + ScalarPositive {}
macro_rules! impl_integer {
    ($primitive_name: ty) => 
    { impl Integer for $primitive_name {} };
}
map_on_integer!(impl_integer);


pub trait Comparable : PartialEq + PartialOrdComparison {}
impl<T> Comparable for T
    where T : PartialEq + PartialOrdComparison
{}


/// Support any kind of number that can be positive : unsigned, signed, floating...
pub trait UnitArithmeticPositive :
    Copy + Clone +
    Add<Self, Output=Self> + Sub<Self, Output=Self> +
    AddAssign<Self> + SubAssign<Self> +
    HaveZero + HaveOne + 
    PartialEq +
    Absolute +
    Debug + Default
{}
impl<T> UnitArithmeticPositive for T
    where T : 
    Copy + Clone +
    Add<T, Output=Self> + Sub<T, Output=Self> +
    AddAssign<Self> + SubAssign<Self> +
    HaveZero + HaveOne + 
    PartialEq +
    Absolute +
    Debug + Default
{}

/// Support any kind of number that can be positive and negative : signed and floating...
/// 
/// For any mesurable things, like distance in (meter, km...), time (s, ms, mins, hours...)
pub trait UnitArithmetic : UnitArithmeticPositive + Neg<Output=Self> + HaveMinusOne {}
impl<T> UnitArithmetic for T
    where T : 
    UnitArithmeticPositive + Neg<Output=Self> + HaveMinusOne
{}


pub trait UnitPositive : UnitArithmeticPositive + Comparable + SmallestIncrement { }
impl<T> UnitPositive for T 
    where T : UnitArithmeticPositive + Comparable + SmallestIncrement
{}

pub trait Unit : UnitPositive + UnitArithmetic + Comparable { }
impl<T> Unit for T 
    where T : UnitPositive + UnitArithmetic + Comparable
{}

pub trait NumberArithmeticPositive :
    UnitArithmeticPositive + 
    Mul<Self, Output=Self> + Div<Self, Output=Self> + Rem<Self, Output=Self> +
    MulAssign<Self> + DivAssign<Self> + RemAssign<Self>
{}
impl<T> NumberArithmeticPositive for T 
    where T : 
    UnitArithmeticPositive + 
    Mul<T, Output=T> + Div<T, Output=T> + Rem<T, Output=T> +
    MulAssign<T> + DivAssign<T> + RemAssign<T>
{}

pub trait NumberArithmetic : NumberArithmeticPositive + UnitArithmetic {}
impl<T> NumberArithmetic for T 
    where T : 
    NumberArithmeticPositive + UnitArithmetic
{}


/// For signed + unsigned + floating and user defined type
pub trait NumberPositive : UnitPositive + NumberArithmeticPositive + HaveDefaultRange {}
impl<T> NumberPositive for T 
    where T : UnitPositive + NumberArithmeticPositive + HaveDefaultRange
{}

/// For signed + floating...
pub trait Number : NumberPositive + Unit + NumberArithmetic {}
impl<T> Number for T 
    where T : NumberPositive + Unit + NumberArithmetic
{}




/// For unsigned + signed + floating only
pub trait ScalarPositive : NumberPositive + ToFloat + FromFloat + ToInt + FromInt + ToUint + FromUint + HaveMinMaxValue + Rounding + Debug + ToUnsigned + ToSigned {}
impl<T> ScalarPositive for T
    where T : NumberPositive + ToFloat + FromFloat + ToInt + FromInt + ToUint + FromUint + HaveMinMaxValue + Rounding + Debug + ToUnsigned + ToSigned
{}

/// For signed + floating only...
pub trait Scalar : ScalarPositive + Number {}
impl<T> Scalar for T
    where T :  ScalarPositive + Number
{}

/// For floating / signed integer : `-x` / `x*-1`
/// 
/// For boolean : `!b`
pub trait Reversable : Copy
{ 
    fn reverse(self) -> Self;
}
macro_rules! impl_rev_nb {
    ($primitive_name: ty) => 
    { impl Reversable for $primitive_name { fn reverse(self) -> Self { -self }} };
}
map_on_floating!(impl_rev_nb);
map_on_signed!(impl_rev_nb);
impl Reversable for bool { fn reverse(self) -> Self { !self }}

pub trait PartialOrdComparison : PartialOrd
{
    #[inline] fn max_partial(self, other: Self) -> Self where Self: Sized { if self >= other { self } else { other } }
    #[inline] fn min_partial(self, other: Self) -> Self where Self: Sized { if self <= other { self } else { other } }

    #[inline] fn clamp_partial(self, min: Self, max: Self) -> Self where Self: Sized
    {
        // copied from rust std
        assert!(min <= max);
             if self < min { min } 
        else if self > max { max } 
        else               { self }
    }
}
impl<T : PartialOrd> PartialOrdComparison for T{}

pub trait Clamp : Sized + Copy
{
    /// Max is included
    fn clamped(&mut self, max_included : Self) -> &mut Self { *self = self.clamp(max_included); self } 
    /// Max is included
    fn clamp(self, max_included : Self) -> Self;
}