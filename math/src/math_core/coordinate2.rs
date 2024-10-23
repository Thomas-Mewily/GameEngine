use crate::*;

type C<T> = C2<T>;

macro_rules! op_with_myself { 
    ($myself : ident, $fn_name : ident, $other : expr) => 
    { Self::new($myself.x.$fn_name($other.x), $myself.y.$fn_name($other.y)) };
}
macro_rules! assign_with_myself {
    ($myself : ident, $fn_name : ident, $other : expr) =>
    { { $myself.x.$fn_name($other.x); $myself.y.$fn_name($other.y); } };
}



macro_rules! op_with_scalar {
    ($myself : ident, $fn_name : ident, $other : expr) => 
    { Self::Output::new($myself.x.$fn_name($other), $myself.y.$fn_name($other)) };
}
macro_rules! assign_with_scalar {
    ($myself : ident, $fn_name : ident, $other : expr) =>
    { { $myself.x.$fn_name($other); $myself.y.$fn_name($other); } };
}


macro_rules! op_component {
    ($myself : ident, $op_name : tt) =>
    {  $myself.x $op_name $myself.y };
}


/// 2D Coordinate
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct C2<T>
{
    pub x : T,
    pub y : T,
}

pub trait Splat2 : Sized + Copy { fn splat2(self) -> C<Self> { C::splat_const(self) }}
impl<T:Copy> Splat2 for T {}

impl<T> From<(T,T)> for C<T> { fn from(value: (T,T)) -> Self { Self::new(value.0, value.1) }}
impl<T> From<C<T>> for (T,T) { fn from(value: C<T>) -> Self { (value.x, value.y) }}

impl<T : Copy> From<[T;2]> for C<T> { fn from(value: [T;2]) -> Self { Self::new(value[0], value[1]) }}
impl<T> From<C<T>> for [T;2] { fn from(value: C<T>) -> Self { [value.x, value.y] }}

impl<T> From<C3<T>> for C<T> { fn from(value: C3<T>) -> Self { Self::new(value.x, value.y) }}
impl<T> From<C4<T>> for C<T> { fn from(value: C4<T>) -> Self { Self::new(value.x, value.y) }}

impl<T : Integer + ToFloat> From<C<T>> for C<float> { fn from(value: C<T>) -> Self { value.map(|e| e.to_float()) }}

impl<T : Debug>   Debug for C<T>   { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({:?}, {:?})", self.x, self.y) }}
impl<T : Display> Display for C<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})", self.x, self.y) } }



// Point Related
type CIter<T> = C2Iter<T>;
pub struct C2Iter<T>
{
    cur : C<T>,
    end : C<T>,
}

impl<T : Integer> Iterator for CIter<T>
{
    type Item=C<T>;

    fn next(&mut self) -> Option<Self::Item> 
    {
        if self.cur.y >= self.end.y { return None; }
        let old = self.cur;

        self.cur.x += one();
        if self.cur.x < self.end.x { return Some(old); }

        self.cur.x  = zero();
        self.cur.y += one();

        Some(old)
    }
}

impl<T : Copy> C<T> 
{
    pub const fn splat_const(xy : T) -> Self { Self::new(xy, xy) }
}

impl<T> C<T> 
{
    pub const fn new(x : T, y : T) -> Self { Self { x, y }}
}

impl<T : Reversable + HaveZero + Copy> C<T>
{
    pub fn x_rx(self) -> Self { Self::new(self.x, self.x.reverse()) }
    pub fn x_ry(self) -> Self { Self::new(self.x, self.y.reverse()) }
    pub fn y_rx(self) -> Self { Self::new(self.y, self.x.reverse()) }
    pub fn y_ry(self) -> Self { Self::new(self.y, self.y.reverse()) }
    pub fn rx_x(self) -> Self { Self::new(self.x.reverse(), self.x) }
    pub fn rx_y(self) -> Self { Self::new(self.x.reverse(), self.y) }
    pub fn rx_rx(self) -> Self { Self::new(self.x.reverse(), self.x.reverse()) }
    pub fn rx_ry(self) -> Self { Self::new(self.x.reverse(), self.y.reverse()) }
    pub fn rx_0(self) -> Self { Self::new(self.x.reverse(), T::ZERO) }
    pub fn ry_x(self) -> Self { Self::new(self.y.reverse(), self.x) }
    pub fn ry_y(self) -> Self { Self::new(self.y.reverse(), self.y) }
    pub fn ry_rx(self) -> Self { Self::new(self.y.reverse(), self.x.reverse()) }
    pub fn ry_ry(self) -> Self { Self::new(self.y.reverse(), self.y.reverse()) }
    pub fn ry_0(self) -> Self { Self::new(self.y.reverse(), T::ZERO) }
    pub fn _0_rx(self) -> Self { Self::new(T::ZERO, self.x.reverse()) }
    pub fn _0_ry(self) -> Self { Self::new(T::ZERO, self.y.reverse()) }
}

impl<T : HaveZero + Copy> C<T> 
{ 
    pub fn x_x(self) -> Self { Self::new(self.x, self.x) }
    pub fn x_y(self) -> Self { Self::new(self.x, self.y) }
    pub fn x_0(self) -> Self { Self::new(self.x, T::ZERO) }
    pub fn y_x(self) -> Self { Self::new(self.y, self.x) }
    pub fn y_y(self) -> Self { Self::new(self.y, self.y) }
    pub fn y_0(self) -> Self { Self::new(self.y, T::ZERO) }
    pub fn _0_x(self) -> Self { Self::new(T::ZERO, self.x) }
    pub fn _0_y(self) -> Self { Self::new(T::ZERO, self.y) }
    pub fn _0_0(self) -> Self { Self::new(T::ZERO, T::ZERO) }
}

impl<T : ScalarPositive + CoordinateRequirements> CoordinateScalarPositive for C<T>
{
    fn length_squared(self) -> T { self.x*self.x + self.y*self.y }
    fn area(self) -> T { (op_component!(self, *)).abs() }

    #[inline] fn min_element(self) -> T { self.x.min_partial(self.y) }
    #[inline] fn max_element(self) -> T { self.x.max_partial(self.y) }
}

impl<T : CoordinateRequirements> Coordinate for C<T>
{
    type Precision = T;
    type SELF<Component> = C<Component>;
    const DIMENSION : usize = 2;

    #[inline] fn splat(xy : T) -> Self { Self::splat_const(xy) }
        
    #[inline] fn x(self) -> T { self.x }
    #[inline] fn y(self) -> T { self.y }
    #[inline] fn z(self) -> T { ___() }
    #[inline] fn w(self) -> T { ___() }

    #[inline] fn set_x(&mut self, x : T) -> &mut Self { self.x = x; self }
    #[inline] fn set_y(&mut self, y : T) -> &mut Self { self.y = y; self }
    #[inline] fn set_z(&mut self, _z : T) -> &mut Self { self }
    #[inline] fn set_w(&mut self, _w : T) -> &mut Self { self }
    
    #[inline] fn with_x(mut self, x : T) -> Self { self.x = x; self }
    #[inline] fn with_y(mut self, y : T) -> Self { self.y = y; self }
    #[inline] fn with_z(self, _z : T) -> Self { self }
    #[inline] fn with_w(self, _w : T) -> Self { self }

    #[inline] fn any<P>(self, p : P) -> bool where P : Fn(T) -> bool { p(self.x) || p(self.y) }
    #[inline] fn all<P>(self, p : P) -> bool where P : Fn(T) -> bool { p(self.x) && p(self.y) }

    #[inline] fn cmp_any<P>(self, other : Self, p : P) -> bool where P : Fn(T, T) -> bool 
    { p(self.x, other.x) || p(self.y, other.y) }
    #[inline] fn cmp_all<P>(self, other : Self, p : P) -> bool where P : Fn(T, T) -> bool
    { p(self.x, other.x) && p(self.y, other.y) }
    #[inline] fn cmp_map<F>(self, other : Self, f : F) -> Self where F : Fn(T, T) -> T
    { Self::new(f(self.x, other.x), f(self.y, other.y)) }

    #[inline] fn new_2d(x : T, y : T) -> Self { Self::new(x, y) }
    #[inline] fn new_3d(x : T, y : T, _z : T) -> Self { Self::new(x, y) }
    #[inline] fn new_4d(x : T, y : T, _z : T, _w : T) -> Self { Self::new(x, y) }    

    #[inline] fn new_with<F>(f : F) -> Self where F : Fn(usize) -> T
    { Self::new(f(0), f(1)) }
    
    //fn iter(&self) -> impl Iterator<Item = &T> + '_ { CComponentIter{ coordinate: self, idx: 0 }}
    #[inline] fn map_intern<F>(self, f : F) -> Self where F : Fn(Self::Precision) -> Self::Precision { self.map(f) }
    #[inline] fn map<Z, F>(self, f : F) -> Self::SELF<Z> where F : Fn(T) -> Z { C::<Z>::new( f(self.x), f(self.y)) }
}

impl<T : Scalar> Moveable<T> for C<T>
{
    fn move_by(&mut self, v : impl Into<C4<T>>) -> &mut Self {
        let c : C4<T> = v.into();
        *self += c.to_c2();
        self
    }
}

impl<T : UnitArithmeticPositive> CoordinateUnitPositive for C<T>
{
    fn sum_axis(self) -> T { op_component!(self, +) }
    fn length_manhattan(self) -> T { self.abs().sum_axis() }
    
    const X : Self = Self::new(T::ONE, T::ZERO);
    const Y : Self = Self::new(T::ZERO, T::ONE);
    const Z : Self = Self::new(T::ZERO, T::ZERO);
    const W : Self = Self::new(T::ZERO, T::ZERO);
}

impl<T : UnitArithmetic> C<T>
{
    const ADJACENT_DELTA : [Self; 4] = 
    [
        Self::new(T::ONE, T::ZERO), Self::new(T::MINUS_ONE, T::ZERO), 
        Self::new(T::ZERO, T::ONE), Self::new(T::ZERO, T::MINUS_ONE),
    ];
    const DIAGONAL_DELTA : [Self; 4] = 
    [
        Self::new(T::ONE, T::ONE),
        Self::new(T::ONE, T::MINUS_ONE),
        Self::new(T::MINUS_ONE, T::ONE),
        Self::new(T::MINUS_ONE, T::MINUS_ONE),
    ];
    /// ADJACENT_DELTA + DIAGONAL_DELTA
    const ORTHO_DIAGONAL_DELTA : [Self; 8] = 
    [
        Self::new(T::ONE, T::ZERO), Self::new(T::MINUS_ONE, T::ZERO), 
        Self::new(T::ZERO, T::ONE), Self::new(T::ZERO, T::MINUS_ONE),

        Self::new(T::ONE, T::ONE),
        Self::new(T::ONE, T::MINUS_ONE),
        Self::new(T::MINUS_ONE, T::ONE),
        Self::new(T::MINUS_ONE, T::MINUS_ONE),
    ];
}

impl<T : UnitArithmetic> CoordinateUnit for C<T>
{
    const LEFT  : Self = Self::new(T::MINUS_ONE, T::ZERO);
    const DOWN  : Self = Self::new(T::ZERO, T::MINUS_ONE);
    const BACKWARD  : Self = Self::new(T::ZERO, T::ZERO);
    
    fn orthogonal_delta() -> Self::OrthogonalArray { Self::ADJACENT_DELTA }
    type OrthogonalArray = [Self; 4];
    
    fn diagonal_delta() -> Self::DiagonalArray { Self::DIAGONAL_DELTA }
    type DiagonalArray = [Self; 4];
    
    fn ortho_diagonal_delta() -> Self::OrthoDiagonalArray { Self::ORTHO_DIAGONAL_DELTA }
    type OrthoDiagonalArray = [Self; 4 + 4];
}

// The rest of the code is the same code for all coordinate type :

impl CoordinateFloating for C<float>
{
    /// set the angle of the `(x, y)` axis
    fn set_angle(&mut self, angle : Angle) -> &mut Self { let a = angle.to_vec2(self.length()); self.set_x(a.x).set_y(a.y); self }
    /// Using the `(x, y)` axis
    fn angle(self) -> Angle { Angle::from_radian(self.y().atan2(self.x())) }
}

impl<T:CoordinateRequirements> From<T> for C<T> { fn from(value: T) -> Self { C::<T>::splat_const(value) }}
impl<T:CoordinateRequirements> From<C<T>> for () { fn from(_value: C<T>) -> Self { () }}

impl<T : HaveHalf + CoordinateRequirements> HaveHalf for C<T>  { const HALF : Self = Self::splat_const(T::HALF); }
impl<T : HaveZero + CoordinateRequirements> HaveZero for C<T> { const ZERO : Self = Self::splat_const(T::ZERO); }
impl<T : HaveOne + CoordinateRequirements> HaveOne for C<T>  { const ONE  : Self = Self::splat_const(T::ONE); }
impl<T : HaveMinusOne + CoordinateRequirements> HaveMinusOne for C<T>  { const MINUS_ONE  : Self = Self::splat_const(T::MINUS_ONE); }
impl<T : Reversable + CoordinateRequirements> Reversable for C<T> { fn reverse(self) -> Self { self.map(|v| v.reverse()) } }

impl<T : Integer> AreaIter for C<T> 
{ 
    type Index = Self;
    fn iter_area(self) -> impl Iterator<Item=Self::Index> 
    { 
        // If a vector have some coordinate that are 0, like (1000, 0), then we should not iterate over it, because it is "empty" 
        CIter { cur: zero(), end: if self.any(|e| e.is_zero()) { zero() } else { self } }
    }
    fn iter_idx(self) -> impl Iterator<Item=Self::Index>
    {
        let end = if self.any(|e| e.is_zero()) { zero() } else { self - Self::ONE };
        CIter { cur: zero(), end }
    }
}


impl<T : Absolute + CoordinateRequirements> Absolute for C<T> { fn abs(self) -> Self { self.map(|e| e.abs()) }}


impl<T : Add<T,Output = T> + Copy> Add<T> for C<T> { type Output=C<T>; fn add(self, rhs: T) -> Self::Output { op_with_scalar!(self,add,rhs)  }}
impl<T : AddAssign<T> + Copy> AddAssign<T> for C<T> { fn add_assign(&mut self, rhs: T) { assign_with_scalar!(self,add_assign,rhs) }}

impl<T : Sub<T,Output = T> + Copy> Sub<T> for C<T> { type Output=C<T>; fn sub(self, rhs: T) -> Self::Output { op_with_scalar!(self,sub,rhs)  }}
impl<T : SubAssign<T> + Copy> SubAssign<T> for C<T> { fn sub_assign(&mut self, rhs: T) { assign_with_scalar!(self,sub_assign,rhs) }}

impl<T : Mul<T,Output = T> + Copy> Mul<T> for C<T> { type Output=C<T>; fn mul(self, rhs: T) -> Self::Output { op_with_scalar!(self,mul,rhs)  }}
impl<T : MulAssign<T> + Copy> MulAssign<T> for C<T> { fn mul_assign(&mut self, rhs: T) { assign_with_scalar!(self,mul_assign,rhs) }}

impl<T : Div<T,Output = T> + Copy> Div<T> for C<T> { type Output=C<T>; fn div(self, rhs: T) -> Self::Output { op_with_scalar!(self,div,rhs)  }}
impl<T : DivAssign<T> + Copy> DivAssign<T> for C<T> { fn div_assign(&mut self, rhs: T) { assign_with_scalar!(self,div_assign,rhs) }}

impl<T : Rem<T,Output = T> + Copy> Rem<T> for C<T> { type Output=C<T>; fn rem(self, rhs: T) -> Self::Output { op_with_scalar!(self,rem,rhs)  }}
impl<T : RemAssign<T> + Copy> RemAssign<T> for C<T> { fn rem_assign(&mut self, rhs: T) { assign_with_scalar!(self,rem_assign,rhs) }}

impl<T : Neg<Output=T> + CoordinateRequirements> Neg for C<T> { type Output = Self; fn neg(self) -> Self::Output { self.map(|v| v.neg()) }}



impl<T : Add<T,Output = T>> Add<Self> for C<T> { type Output=C<T>; fn add(self, rhs: Self) -> Self::Output { op_with_myself!(self,add,rhs) }}
impl<T : AddAssign> AddAssign<Self> for C<T> { fn add_assign(&mut self, rhs: Self) { assign_with_myself!(self,add_assign,rhs) }}

impl<T : Sub<T,Output = T>> Sub<Self> for C<T> { type Output=C<T>; fn sub(self, rhs: Self) -> Self::Output  { op_with_myself!(self,sub,rhs) }}
impl<T : SubAssign> SubAssign<Self> for C<T> { fn sub_assign(&mut self, rhs: Self) { assign_with_myself!(self,sub_assign,rhs) }}

impl<T : Mul<T,Output = T>> Mul<Self> for C<T> { type Output=C<T>; fn mul(self, rhs: Self) -> Self::Output { op_with_myself!(self,mul,rhs) }}
impl<T : MulAssign> MulAssign<Self> for C<T>{ fn mul_assign(&mut self, rhs: Self) { assign_with_myself!(self,mul_assign,rhs) }}

impl<T : Div<T,Output = T>> Div<Self> for C<T> { type Output=C<T>; fn div(self, rhs: Self) -> Self::Output { op_with_myself!(self,div,rhs) }}
impl<T : DivAssign> DivAssign<Self> for C<T>{ fn div_assign(&mut self, rhs: Self) { assign_with_myself!(self,div_assign,rhs) }}

impl<T : Rem<T,Output = T>> Rem<Self> for C<T> { type Output=C<T>; fn rem(self, rhs: Self) -> Self::Output { op_with_myself!(self,rem,rhs) }}
impl<T : RemAssign> RemAssign<Self> for C<T>{ fn rem_assign(&mut self, rhs: Self) { assign_with_myself!(self,rem_assign,rhs) }}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_const_and_splat() {
        assert_eq!(C::MINUS_ONE, C::splat(-1));
        assert_eq!(C::ONE, C::splat(1));
        assert_eq!(C::ZERO, C::splat(0));
        assert_eq!(C::splat(1) + C::splat(1), C::splat(2));
        assert_eq!(C::splat(1) * 2, C::splat(2));
    }

    macro_rules! check_vec_const {
        ($name : ident, $val : expr) => {
            assert_eq!(C::<f32>::$name.x(), if C::<f32>::DIMENSION >= 1 { $val } else { 0. });
            assert_eq!(C::<f32>::$name.y(), if C::<f32>::DIMENSION >= 2 { $val } else { 0. });
            assert_eq!(C::<f32>::$name.z(), if C::<f32>::DIMENSION >= 3 { $val } else { 0. });
            assert_eq!(C::<f32>::$name.w(), if C::<f32>::DIMENSION >= 4 { $val } else { 0. });
        };
    }

    #[test]
    fn test_dimension() 
    {
        check_vec_const!(ZERO, 0.);
        check_vec_const!(ONE, 1.);
        check_vec_const!(MINUS_ONE, -1.);
        check_vec_const!(HALF, 0.5);
        
        assert_eq!(C::new_4d(1, 2, 3, 4).x(), if C::<i32>::DIMENSION >= 1 { 1 } else { 0 });
        assert_eq!(C::new_4d(1, 2, 3, 4).y(), if C::<i32>::DIMENSION >= 2 { 2 } else { 0 });
        assert_eq!(C::new_4d(1, 2, 3, 4).z(), if C::<i32>::DIMENSION >= 3 { 3 } else { 0 });
        assert_eq!(C::new_4d(1, 2, 3, 4).w(), if C::<i32>::DIMENSION >= 4 { 4 } else { 0 });
    }

    #[test]
    fn len_test() 
    {
        assert_eq!(C::new_2d(0, 0).length_squared(), 0);
        assert_eq!(C::new_2d(1, 0).length_squared(), 1);
        assert_eq!(C::new_2d(0, 1).length_squared(), 1);
        assert_eq!(C::new_2d(1, 1).length_squared(), 2);
        assert_eq!(C::new_2d(10, 5).length_squared(), 125);

        assert_eq!(C::new_2d(5, 3).length_squared(), 34);
        
        assert_eq!(C::new_2d(3, 3).length_manhattan(), 6);
    }

    #[test]
    fn vec_util() {
        assert_eq!(C::new_4d(1, 2, 3, 4).max_element(), C::<i32>::DIMENSION as i32);
        assert_eq!((C::new_4d(1, 2, 3, 4) * -1).min_element(), -(C::<i32>::DIMENSION as i32));
        assert_eq!(C::new_4d(-1, -2, -3, -4), -C::new_4d(1, 2, 3, 4));

        assert_eq!(C::new_4d(1, 2, 3, 4).map(|e| 2 * e), C::new_4d(2, 4, 6, 8));

        assert_eq!(C::new_4d(1, 2, 3, 4).cmp_all(C::new_4d(10, 20, 30, 40), |a,b| a < b), true);
        assert_eq!(C::new_4d(10, 20, 30, 40).cmp_all(C::new_4d(1, 2, 3, 4), |a,b| a > b), true);

        assert_eq!(C::new_4d(1, 1, 1, 1).cmp_any(C::new_4d(0, 0, 0, 0), |a,b| a == b), false);
        assert_eq!(C::new_4d(1, 0, 1, 0).cmp_any(C::new_4d(0, 1, 0, 1), |a,b| a == b), false);
        assert_eq!(C::new_4d(1, 0, 0, 0).cmp_any(C::new_4d(1, 0, 0, 0), |a,b| a == b), true);
        assert_eq!(C::new_4d(0, 1, 0, 0).cmp_any(C::new_4d(0, 1, 0, 0), |a,b| a == b), true);
        assert_eq!(C::new_4d(0, 0, 1, 0).cmp_any(C::new_4d(0, 0, 1, 0), |a,b| a == b), true);
        assert_eq!(C::new_4d(0, 0, 0, 1).cmp_any(C::new_4d(0, 0, 0, 1), |a,b| a == b), true);

        assert_eq!(C::new_4d(1, 2, 3, 4).cmp_map(C::new_4d(4, 3, 2, 1), |a,b| a + b), C::splat_const(5));

        assert_eq!(C::new_4d(1, 2, 3, 4).sum_axis(), C::<i32>::DIMENSION * (C::<i32>::DIMENSION+1) / 2);
    }
}