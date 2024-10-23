use crate::*;

pub trait AngleExtension
{
    fn degree(self) -> Angle;
    fn radian(self) -> Angle;
    fn turn  (self) -> Angle;
}
impl<T:ToFloat> AngleExtension for T
{
    fn degree(self) -> Angle { Angle::from_degree(self.to_float()) }
    fn radian(self) -> Angle { Angle::from_radian(self.to_float()) }
    fn turn  (self) -> Angle { Angle::from_turn(self.to_float()) }
}

/// 2D Angle, support degree, radian, turn...
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle
{
    _radian : float,
}

pub const ANGLE_ZERO : Angle = Angle::ZERO;
pub const ANGLE_FULL : Angle = Angle::FULL;
pub const ANGLE_HALF : Angle = Angle::HALF;
pub const ANGLE_FLAT : Angle = Angle::FLAT;

impl HaveZero for Angle { const ZERO : Self = Angle { _radian : 0. }; }
impl Angle
{
    /// `360°`
    pub const FULL : Angle = Angle { _radian : float::PI * 2. };
    /// `180°`
    pub const HALF : Angle = Angle { _radian : float::PI };
    /// `180°`
    pub const FLAT : Angle = Angle { _radian : float::PI / 2. };
    /// `90°`
    pub const RIGHT : Angle = Angle { _radian : float::PI / 2. };

    pub const ZERO_RADIAN : float = 0.;
    pub const FULL_RADIAN : float = float::PI * 2.;
    pub const HALF_RADIAN : float = float::PI;
    pub const FLAT_RADIAN : float = Self::HALF_RADIAN;
    pub const RIGHT_RADIAN : float = Self::HALF_RADIAN / 2.;

    pub const ZERO_DEGREE : float = 0.;
    pub const FULL_DEGREE : float = 360.;
    pub const HALF_DEGREE : float = Self::FULL_DEGREE / 2.;
    pub const FLAT_DEGREE : float = Self::HALF_DEGREE;
    pub const RIGHT_DEGREE : float = Self::HALF_DEGREE / 2.;

    pub const ZERO_TURN : float = 0.;
    pub const FULL_TURN : float = 1.;
    pub const HALF_TURN : float = Self::FULL_TURN / 2.;
    pub const FLAT_TURN : float = Self::HALF_TURN;
    pub const RIGHT_TURN : float = Self::HALF_TURN / 2.;

    pub fn from_radian(val  : float) -> Self { Self { _radian: val }}
    pub fn from_degree(val  : float) -> Self { Self { _radian: val * (Self::FULL_RADIAN / Self::FULL_DEGREE)  }}
    pub fn from_turn  (coef : Coef) -> Self { Self { _radian: coef * Self::FULL_RADIAN  }}

    fn new_native(val : float) -> Self { Self::from_radian(val) }

    pub fn radian(self) -> float { self._radian }
    pub fn degree(self) -> float { self._radian * (Self::FULL_DEGREE / Self::FULL_RADIAN) }
    pub fn turn  (self) -> Coef { self._radian  / Self::FULL_RADIAN }

    // Todo : check if better way to do it
    /// `[0, 2PI[`
    pub fn normalized_positive(self) -> Self { Self::from_radian((self._radian % Self::FULL_RADIAN + Self::FULL_RADIAN) % Self::FULL_RADIAN)  }
    
    // Todo : check if better way to do it
    /// `]PI; PI]`
    pub fn normalized(self) -> Self 
    {
        let tmp = self.normalized_positive();
        if tmp < Self::HALF { tmp } else { tmp - Self::FULL }
    }

    #[inline] pub fn cos_sin(self) -> Vec2 { vec2(self.cos(), self.sin()) }
    #[inline] pub fn sin_cos(self) -> Vec2 { vec2(self.sin(), self.cos()) }
    #[inline] pub fn cos_cos(self) -> Vec2 { vec2(self.cos(), self.cos()) }
    #[inline] pub fn sin_sin(self) -> Vec2 { vec2(self.sin(), self.sin()) }


    #[inline] pub fn cos(self) -> float { self._radian.cos() }
    #[inline] pub fn cosh(self) -> float { self._radian.cosh() }

    #[inline] pub fn sin(self) -> float { self._radian.sin() }
    #[inline] pub fn sinh(self) -> float { self._radian.sinh() }

    #[inline] pub fn tan(self) -> float { self._radian.tan() }
    #[inline] pub fn tanh(self) -> float { self._radian.tanh() }

    /// Return a normalized (lenght = 1) vector with the same angle
    #[inline] pub fn to_vec2_normalized(self) -> Vec2 { self.to_vec2(1.) }
    #[inline] pub fn to_vec2(self, length : float) -> Vec2 { Vec2::new(self.cos() * length, self.sin()* length) }

    pub fn inside_range(self, begin : Angle, end : Angle) -> bool
    {
        let self_normalized = self.normalized_positive();
        let begin_normalized = begin.normalized_positive();
        let end_normalized   = end.normalized_positive();

        if begin_normalized._radian <= end_normalized._radian
        {
            self_normalized._radian >= begin_normalized._radian && self_normalized._radian <= end_normalized._radian
        }
        else
        {
            self_normalized._radian >= begin_normalized._radian || self_normalized._radian <= end_normalized._radian
        }
    }
}


impl Debug for Angle
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult { write!(f, "{}°", self.degree()) }
}

impl Angle
{
    pub fn fmt_degree_full(self, precision : Option<float>) -> DisplayAngleDegree { DisplayAngleDegree { angle: self, precision }}
    pub fn fmt_degree_with_precision(self, precision : float) -> DisplayAngleDegree { self.fmt_degree_full(Some(precision)) }
    pub fn fmt_degree(self) -> DisplayAngleDegree { self.fmt_degree_full(None) }
}
impl Display for Angle { fn fmt(&self, f: &mut Formatter<'_>) -> DResult { self.fmt_degree_with_precision(360.).fmt(f) }}

#[derive(Clone, Copy)]
pub struct DisplayAngleDegree{ angle : Angle, precision : Option<float> }
impl Display for DisplayAngleDegree {
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult 
    {
        write!(f, "{}°", 
        {
            match self.precision
            {
                Some(p) => (self.angle.degree() / p) as i32 as float * p,
                None => self.angle.degree(),
            }            
        })
    }
}

pub trait ToAngle
{
    fn degree(&self) -> Angle;
    fn radian(&self) -> Angle;
    fn turn(&self) -> Angle;
}

impl ToAngle for float
{
    fn degree(&self) -> Angle { Angle::from_degree(*self) }
    fn radian(&self) -> Angle { Angle::from_radian(*self) }
    fn turn(&self) -> Angle   { Angle::from_turn(*self) }
}


macro_rules! impl_op {
    ($primitive_name : ty) => 
    {
        impl Mul<$primitive_name> for Angle { type Output=Angle; fn mul(self, rhs: $primitive_name) -> Self::Output { Self::new_native(self._radian * rhs.to_float()) }}
        impl MulAssign<$primitive_name> for Angle { fn mul_assign(&mut self, rhs: $primitive_name) { self._radian.mul_assign(rhs.to_float()); }}
    
        impl Div<$primitive_name> for Angle { type Output=Angle; fn div(self, rhs: $primitive_name) -> Self::Output { Self::new_native(self._radian/ rhs.to_float()) }}
        impl DivAssign<$primitive_name> for Angle { fn div_assign(&mut self, rhs: $primitive_name) { self._radian.div_assign(rhs.to_float()); }}
    };
}

map_on_scalar!(impl_op);

impl Neg for Angle { type Output=Angle; fn neg(self) -> Self::Output { Self::new_native(self._radian.neg()) }}

impl Add<Angle> for Angle { type Output=Angle; fn add(self, rhs: Angle) -> Self::Output { Self::new_native(self._radian.add(rhs._radian)) }}
impl AddAssign<Angle> for Angle { fn add_assign(&mut self, rhs: Angle) { self._radian.add_assign(rhs._radian); }}

impl Sub<Angle> for Angle { type Output=Angle; fn sub(self, rhs: Angle) -> Self::Output { Self::new_native(self._radian.sub(rhs._radian)) }}
impl SubAssign<Angle> for Angle { fn sub_assign(&mut self, rhs: Angle) { self._radian.sub_assign(rhs._radian); }}

impl Div<Angle> for Angle { type Output=Angle; fn div(self, rhs: Angle) -> Self::Output { Self::new_native(self._radian.div(rhs._radian)) }}
impl DivAssign<Angle> for Angle { fn div_assign(&mut self, rhs: Angle) { self._radian.div_assign(rhs._radian); }}

impl Rem<Angle> for Angle { type Output=Angle; fn rem(self, rhs: Angle) -> Self::Output { Self::new_native(self._radian.rem(rhs._radian)) }}
impl RemAssign<Angle> for Angle { fn rem_assign(&mut self, rhs: Angle) { self._radian.rem_assign(rhs._radian); }}