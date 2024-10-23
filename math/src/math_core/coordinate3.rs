use crate::*;

type C<T> = C3<T>;

macro_rules! op_with_myself { 
    ($myself : ident, $fn_name : ident, $other : expr) => 
    { Self::new($myself.x.$fn_name($other.x), $myself.y.$fn_name($other.y), $myself.z.$fn_name($other.z)) };
}
macro_rules! assign_with_myself {
    ($myself : ident, $fn_name : ident, $other : expr) =>
    { { $myself.x.$fn_name($other.x); $myself.y.$fn_name($other.y); $myself.z.$fn_name($other.z); } };
}



macro_rules! op_with_scalar {
    ($myself : ident, $fn_name : ident, $other : expr) => 
    { Self::Output::new($myself.x.$fn_name($other), $myself.y.$fn_name($other), $myself.z.$fn_name($other)) };
}
macro_rules! assign_with_scalar {
    ($myself : ident, $fn_name : ident, $other : expr) =>
    { { $myself.x.$fn_name($other); $myself.y.$fn_name($other); $myself.z.$fn_name($other); } };
}


macro_rules! op_component {
    ($myself : ident, $op_name : tt) =>
    {  $myself.x $op_name $myself.y $op_name $myself.z };
}

/// 3D Coordinate
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct C3<T>
{
    pub x : T,
    pub y : T,
    pub z : T,
}

pub trait Splat3 : Sized + Copy { fn splat3(self) -> C<Self> { C::splat_const(self) }}
impl<T:Copy> Splat3 for T {}

impl<T> From<(T,T,T)> for C<T> { fn from(value: (T,T,T)) -> Self { Self::new(value.0, value.1, value.2) }}
impl<T> From<C<T>> for (T,T,T) { fn from(value: C<T>) -> Self { (value.x, value.y, value.z) }}

impl<T : Copy> From<[T;3]> for C<T> { fn from(value: [T;3]) -> Self { Self::new(value[0], value[1], value[2]) }}
impl<T> From<C<T>> for [T;3] { fn from(value: C<T>) -> Self { [value.x, value.y, value.z] }}

impl<T : Default> From<C2<T>> for C<T> { fn from(value: C2<T>) -> Self { Self::new(value.x, value.y, ___()) }}
impl<T> From<C4<T>> for C<T> { fn from(value: C4<T>) -> Self { Self::new(value.x, value.y, value.z) }}

impl<T : Integer + ToFloat> From<C<T>> for C<float> { fn from(value: C<T>) -> Self { value.map(|e| e.to_float()) }}

impl<T : Debug>   Debug for C<T>   { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z) }}
impl<T : Display> Display for C<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {}, {})", self.x, self.y, self.z) } }

// Point Related
type CIter<T> = C3Iter<T>;
pub struct C3Iter<T>
{
    cur : C<T>,
    end : C<T>,
}

impl<T : Integer> Iterator for CIter<T>
{
    type Item=C<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.z >= self.end.z { return None; }
        let old = self.cur;

        self.cur.x += one();
        if self.cur.x < self.end.x { return Some(old); }

        self.cur.x  = zero();
        self.cur.y += one();
        if self.cur.y < self.end.y { return Some(old); }

        self.cur.y  = zero();
        self.cur.z += one();

        Some(old)
    }
}

// Vector Related
impl C<float>
{
    /// set the angle of the `(x, y)` axis
    pub fn set_angle(&mut self, angle : Angle) { *self = angle.to_vec2(self.length()).to_vec3().with_z(self.z) }
}

impl<T : Copy> C<T> 
{
    pub const fn splat_const(xyz : T) -> Self { Self::new(xyz, xyz, xyz) }
}

impl<T> C<T> 
{
    pub const fn new(x : T, y : T, z : T) -> Self { Self { x, y, z }}
}

impl<T : Reversable + HaveZero + Copy> C<T>
{
    pub fn x_x_rx(self) -> Self { Self::new(self.x, self.x, self.x.reverse()) }
    pub fn x_x_ry(self) -> Self { Self::new(self.x, self.x, self.y.reverse()) }
    pub fn x_x_rz(self) -> Self { Self::new(self.x, self.x, self.z.reverse()) }
    pub fn x_y_rx(self) -> Self { Self::new(self.x, self.y, self.x.reverse()) }
    pub fn x_y_ry(self) -> Self { Self::new(self.x, self.y, self.y.reverse()) }
    pub fn x_y_rz(self) -> Self { Self::new(self.x, self.y, self.z.reverse()) }
    pub fn x_z_rx(self) -> Self { Self::new(self.x, self.z, self.x.reverse()) }
    pub fn x_z_ry(self) -> Self { Self::new(self.x, self.z, self.y.reverse()) }
    pub fn x_z_rz(self) -> Self { Self::new(self.x, self.z, self.z.reverse()) }
    pub fn x_rx_x(self) -> Self { Self::new(self.x, self.x.reverse(), self.x) }
    pub fn x_rx_y(self) -> Self { Self::new(self.x, self.x.reverse(), self.y) }
    pub fn x_rx_z(self) -> Self { Self::new(self.x, self.x.reverse(), self.z) }
    pub fn x_rx_rx(self) -> Self { Self::new(self.x, self.x.reverse(), self.x.reverse()) }
    pub fn x_rx_ry(self) -> Self { Self::new(self.x, self.x.reverse(), self.y.reverse()) }
    pub fn x_rx_rz(self) -> Self { Self::new(self.x, self.x.reverse(), self.z.reverse()) }
    pub fn x_rx_0(self) -> Self { Self::new(self.x, self.x.reverse(), T::ZERO) }
    pub fn x_ry_x(self) -> Self { Self::new(self.x, self.y.reverse(), self.x) }
    pub fn x_ry_y(self) -> Self { Self::new(self.x, self.y.reverse(), self.y) }
    pub fn x_ry_z(self) -> Self { Self::new(self.x, self.y.reverse(), self.z) }
    pub fn x_ry_rx(self) -> Self { Self::new(self.x, self.y.reverse(), self.x.reverse()) }
    pub fn x_ry_ry(self) -> Self { Self::new(self.x, self.y.reverse(), self.y.reverse()) }
    pub fn x_ry_rz(self) -> Self { Self::new(self.x, self.y.reverse(), self.z.reverse()) }
    pub fn x_ry_0(self) -> Self { Self::new(self.x, self.y.reverse(), T::ZERO) }
    pub fn x_rz_x(self) -> Self { Self::new(self.x, self.z.reverse(), self.x) }
    pub fn x_rz_y(self) -> Self { Self::new(self.x, self.z.reverse(), self.y) }
    pub fn x_rz_z(self) -> Self { Self::new(self.x, self.z.reverse(), self.z) }
    pub fn x_rz_rx(self) -> Self { Self::new(self.x, self.z.reverse(), self.x.reverse()) }
    pub fn x_rz_ry(self) -> Self { Self::new(self.x, self.z.reverse(), self.y.reverse()) }
    pub fn x_rz_rz(self) -> Self { Self::new(self.x, self.z.reverse(), self.z.reverse()) }
    pub fn x_rz_0(self) -> Self { Self::new(self.x, self.z.reverse(), T::ZERO) }
    pub fn x_0_rx(self) -> Self { Self::new(self.x, T::ZERO, self.x.reverse()) }
    pub fn x_0_ry(self) -> Self { Self::new(self.x, T::ZERO, self.y.reverse()) }
    pub fn x_0_rz(self) -> Self { Self::new(self.x, T::ZERO, self.z.reverse()) }
    pub fn y_x_rx(self) -> Self { Self::new(self.y, self.x, self.x.reverse()) }
    pub fn y_x_ry(self) -> Self { Self::new(self.y, self.x, self.y.reverse()) }
    pub fn y_x_rz(self) -> Self { Self::new(self.y, self.x, self.z.reverse()) }
    pub fn y_y_rx(self) -> Self { Self::new(self.y, self.y, self.x.reverse()) }
    pub fn y_y_ry(self) -> Self { Self::new(self.y, self.y, self.y.reverse()) }
    pub fn y_y_rz(self) -> Self { Self::new(self.y, self.y, self.z.reverse()) }
    pub fn y_z_rx(self) -> Self { Self::new(self.y, self.z, self.x.reverse()) }
    pub fn y_z_ry(self) -> Self { Self::new(self.y, self.z, self.y.reverse()) }
    pub fn y_z_rz(self) -> Self { Self::new(self.y, self.z, self.z.reverse()) }
    pub fn y_rx_x(self) -> Self { Self::new(self.y, self.x.reverse(), self.x) }
    pub fn y_rx_y(self) -> Self { Self::new(self.y, self.x.reverse(), self.y) }
    pub fn y_rx_z(self) -> Self { Self::new(self.y, self.x.reverse(), self.z) }
    pub fn y_rx_rx(self) -> Self { Self::new(self.y, self.x.reverse(), self.x.reverse()) }
    pub fn y_rx_ry(self) -> Self { Self::new(self.y, self.x.reverse(), self.y.reverse()) }
    pub fn y_rx_rz(self) -> Self { Self::new(self.y, self.x.reverse(), self.z.reverse()) }
    pub fn y_rx_0(self) -> Self { Self::new(self.y, self.x.reverse(), T::ZERO) }
    pub fn y_ry_x(self) -> Self { Self::new(self.y, self.y.reverse(), self.x) }
    pub fn y_ry_y(self) -> Self { Self::new(self.y, self.y.reverse(), self.y) }
    pub fn y_ry_z(self) -> Self { Self::new(self.y, self.y.reverse(), self.z) }
    pub fn y_ry_rx(self) -> Self { Self::new(self.y, self.y.reverse(), self.x.reverse()) }
    pub fn y_ry_ry(self) -> Self { Self::new(self.y, self.y.reverse(), self.y.reverse()) }
    pub fn y_ry_rz(self) -> Self { Self::new(self.y, self.y.reverse(), self.z.reverse()) }
    pub fn y_ry_0(self) -> Self { Self::new(self.y, self.y.reverse(), T::ZERO) }
    pub fn y_rz_x(self) -> Self { Self::new(self.y, self.z.reverse(), self.x) }
    pub fn y_rz_y(self) -> Self { Self::new(self.y, self.z.reverse(), self.y) }
    pub fn y_rz_z(self) -> Self { Self::new(self.y, self.z.reverse(), self.z) }
    pub fn y_rz_rx(self) -> Self { Self::new(self.y, self.z.reverse(), self.x.reverse()) }
    pub fn y_rz_ry(self) -> Self { Self::new(self.y, self.z.reverse(), self.y.reverse()) }
    pub fn y_rz_rz(self) -> Self { Self::new(self.y, self.z.reverse(), self.z.reverse()) }
    pub fn y_rz_0(self) -> Self { Self::new(self.y, self.z.reverse(), T::ZERO) }
    pub fn y_0_rx(self) -> Self { Self::new(self.y, T::ZERO, self.x.reverse()) }
    pub fn y_0_ry(self) -> Self { Self::new(self.y, T::ZERO, self.y.reverse()) }
    pub fn y_0_rz(self) -> Self { Self::new(self.y, T::ZERO, self.z.reverse()) }
    pub fn z_x_rx(self) -> Self { Self::new(self.z, self.x, self.x.reverse()) }
    pub fn z_x_ry(self) -> Self { Self::new(self.z, self.x, self.y.reverse()) }
    pub fn z_x_rz(self) -> Self { Self::new(self.z, self.x, self.z.reverse()) }
    pub fn z_y_rx(self) -> Self { Self::new(self.z, self.y, self.x.reverse()) }
    pub fn z_y_ry(self) -> Self { Self::new(self.z, self.y, self.y.reverse()) }
    pub fn z_y_rz(self) -> Self { Self::new(self.z, self.y, self.z.reverse()) }
    pub fn z_z_rx(self) -> Self { Self::new(self.z, self.z, self.x.reverse()) }
    pub fn z_z_ry(self) -> Self { Self::new(self.z, self.z, self.y.reverse()) }
    pub fn z_z_rz(self) -> Self { Self::new(self.z, self.z, self.z.reverse()) }
    pub fn z_rx_x(self) -> Self { Self::new(self.z, self.x.reverse(), self.x) }
    pub fn z_rx_y(self) -> Self { Self::new(self.z, self.x.reverse(), self.y) }
    pub fn z_rx_z(self) -> Self { Self::new(self.z, self.x.reverse(), self.z) }
    pub fn z_rx_rx(self) -> Self { Self::new(self.z, self.x.reverse(), self.x.reverse()) }
    pub fn z_rx_ry(self) -> Self { Self::new(self.z, self.x.reverse(), self.y.reverse()) }
    pub fn z_rx_rz(self) -> Self { Self::new(self.z, self.x.reverse(), self.z.reverse()) }
    pub fn z_rx_0(self) -> Self { Self::new(self.z, self.x.reverse(), T::ZERO) }
    pub fn z_ry_x(self) -> Self { Self::new(self.z, self.y.reverse(), self.x) }
    pub fn z_ry_y(self) -> Self { Self::new(self.z, self.y.reverse(), self.y) }
    pub fn z_ry_z(self) -> Self { Self::new(self.z, self.y.reverse(), self.z) }
    pub fn z_ry_rx(self) -> Self { Self::new(self.z, self.y.reverse(), self.x.reverse()) }
    pub fn z_ry_ry(self) -> Self { Self::new(self.z, self.y.reverse(), self.y.reverse()) }
    pub fn z_ry_rz(self) -> Self { Self::new(self.z, self.y.reverse(), self.z.reverse()) }
    pub fn z_ry_0(self) -> Self { Self::new(self.z, self.y.reverse(), T::ZERO) }
    pub fn z_rz_x(self) -> Self { Self::new(self.z, self.z.reverse(), self.x) }
    pub fn z_rz_y(self) -> Self { Self::new(self.z, self.z.reverse(), self.y) }
    pub fn z_rz_z(self) -> Self { Self::new(self.z, self.z.reverse(), self.z) }
    pub fn z_rz_rx(self) -> Self { Self::new(self.z, self.z.reverse(), self.x.reverse()) }
    pub fn z_rz_ry(self) -> Self { Self::new(self.z, self.z.reverse(), self.y.reverse()) }
    pub fn z_rz_rz(self) -> Self { Self::new(self.z, self.z.reverse(), self.z.reverse()) }
    pub fn z_rz_0(self) -> Self { Self::new(self.z, self.z.reverse(), T::ZERO) }
    pub fn z_0_rx(self) -> Self { Self::new(self.z, T::ZERO, self.x.reverse()) }
    pub fn z_0_ry(self) -> Self { Self::new(self.z, T::ZERO, self.y.reverse()) }
    pub fn z_0_rz(self) -> Self { Self::new(self.z, T::ZERO, self.z.reverse()) }
    pub fn rx_x_x(self) -> Self { Self::new(self.x.reverse(), self.x, self.x) }
    pub fn rx_x_y(self) -> Self { Self::new(self.x.reverse(), self.x, self.y) }
    pub fn rx_x_z(self) -> Self { Self::new(self.x.reverse(), self.x, self.z) }
    pub fn rx_x_rx(self) -> Self { Self::new(self.x.reverse(), self.x, self.x.reverse()) }
    pub fn rx_x_ry(self) -> Self { Self::new(self.x.reverse(), self.x, self.y.reverse()) }
    pub fn rx_x_rz(self) -> Self { Self::new(self.x.reverse(), self.x, self.z.reverse()) }
    pub fn rx_x_0(self) -> Self { Self::new(self.x.reverse(), self.x, T::ZERO) }
    pub fn rx_y_x(self) -> Self { Self::new(self.x.reverse(), self.y, self.x) }
    pub fn rx_y_y(self) -> Self { Self::new(self.x.reverse(), self.y, self.y) }
    pub fn rx_y_z(self) -> Self { Self::new(self.x.reverse(), self.y, self.z) }
    pub fn rx_y_rx(self) -> Self { Self::new(self.x.reverse(), self.y, self.x.reverse()) }
    pub fn rx_y_ry(self) -> Self { Self::new(self.x.reverse(), self.y, self.y.reverse()) }
    pub fn rx_y_rz(self) -> Self { Self::new(self.x.reverse(), self.y, self.z.reverse()) }
    pub fn rx_y_0(self) -> Self { Self::new(self.x.reverse(), self.y, T::ZERO) }
    pub fn rx_z_x(self) -> Self { Self::new(self.x.reverse(), self.z, self.x) }
    pub fn rx_z_y(self) -> Self { Self::new(self.x.reverse(), self.z, self.y) }
    pub fn rx_z_z(self) -> Self { Self::new(self.x.reverse(), self.z, self.z) }
    pub fn rx_z_rx(self) -> Self { Self::new(self.x.reverse(), self.z, self.x.reverse()) }
    pub fn rx_z_ry(self) -> Self { Self::new(self.x.reverse(), self.z, self.y.reverse()) }
    pub fn rx_z_rz(self) -> Self { Self::new(self.x.reverse(), self.z, self.z.reverse()) }
    pub fn rx_z_0(self) -> Self { Self::new(self.x.reverse(), self.z, T::ZERO) }
    pub fn rx_rx_x(self) -> Self { Self::new(self.x.reverse(), self.x.reverse(), self.x) }
    pub fn rx_rx_y(self) -> Self { Self::new(self.x.reverse(), self.x.reverse(), self.y) }
    pub fn rx_rx_z(self) -> Self { Self::new(self.x.reverse(), self.x.reverse(), self.z) }
    pub fn rx_rx_rx(self) -> Self { Self::new(self.x.reverse(), self.x.reverse(), self.x.reverse()) }
    pub fn rx_rx_ry(self) -> Self { Self::new(self.x.reverse(), self.x.reverse(), self.y.reverse()) }
    pub fn rx_rx_rz(self) -> Self { Self::new(self.x.reverse(), self.x.reverse(), self.z.reverse()) }
    pub fn rx_rx_0(self) -> Self { Self::new(self.x.reverse(), self.x.reverse(), T::ZERO) }
    pub fn rx_ry_x(self) -> Self { Self::new(self.x.reverse(), self.y.reverse(), self.x) }
    pub fn rx_ry_y(self) -> Self { Self::new(self.x.reverse(), self.y.reverse(), self.y) }
    pub fn rx_ry_z(self) -> Self { Self::new(self.x.reverse(), self.y.reverse(), self.z) }
    pub fn rx_ry_rx(self) -> Self { Self::new(self.x.reverse(), self.y.reverse(), self.x.reverse()) }
    pub fn rx_ry_ry(self) -> Self { Self::new(self.x.reverse(), self.y.reverse(), self.y.reverse()) }
    pub fn rx_ry_rz(self) -> Self { Self::new(self.x.reverse(), self.y.reverse(), self.z.reverse()) }
    pub fn rx_ry_0(self) -> Self { Self::new(self.x.reverse(), self.y.reverse(), T::ZERO) }
    pub fn rx_rz_x(self) -> Self { Self::new(self.x.reverse(), self.z.reverse(), self.x) }
    pub fn rx_rz_y(self) -> Self { Self::new(self.x.reverse(), self.z.reverse(), self.y) }
    pub fn rx_rz_z(self) -> Self { Self::new(self.x.reverse(), self.z.reverse(), self.z) }
    pub fn rx_rz_rx(self) -> Self { Self::new(self.x.reverse(), self.z.reverse(), self.x.reverse()) }
    pub fn rx_rz_ry(self) -> Self { Self::new(self.x.reverse(), self.z.reverse(), self.y.reverse()) }
    pub fn rx_rz_rz(self) -> Self { Self::new(self.x.reverse(), self.z.reverse(), self.z.reverse()) }
    pub fn rx_rz_0(self) -> Self { Self::new(self.x.reverse(), self.z.reverse(), T::ZERO) }
    pub fn rx_0_x(self) -> Self { Self::new(self.x.reverse(), T::ZERO, self.x) }
    pub fn rx_0_y(self) -> Self { Self::new(self.x.reverse(), T::ZERO, self.y) }
    pub fn rx_0_z(self) -> Self { Self::new(self.x.reverse(), T::ZERO, self.z) }
    pub fn rx_0_rx(self) -> Self { Self::new(self.x.reverse(), T::ZERO, self.x.reverse()) }
    pub fn rx_0_ry(self) -> Self { Self::new(self.x.reverse(), T::ZERO, self.y.reverse()) }
    pub fn rx_0_rz(self) -> Self { Self::new(self.x.reverse(), T::ZERO, self.z.reverse()) }
    pub fn rx_0_0(self) -> Self { Self::new(self.x.reverse(), T::ZERO, T::ZERO) }
    pub fn ry_x_x(self) -> Self { Self::new(self.y.reverse(), self.x, self.x) }
    pub fn ry_x_y(self) -> Self { Self::new(self.y.reverse(), self.x, self.y) }
    pub fn ry_x_z(self) -> Self { Self::new(self.y.reverse(), self.x, self.z) }
    pub fn ry_x_rx(self) -> Self { Self::new(self.y.reverse(), self.x, self.x.reverse()) }
    pub fn ry_x_ry(self) -> Self { Self::new(self.y.reverse(), self.x, self.y.reverse()) }
    pub fn ry_x_rz(self) -> Self { Self::new(self.y.reverse(), self.x, self.z.reverse()) }
    pub fn ry_x_0(self) -> Self { Self::new(self.y.reverse(), self.x, T::ZERO) }
    pub fn ry_y_x(self) -> Self { Self::new(self.y.reverse(), self.y, self.x) }
    pub fn ry_y_y(self) -> Self { Self::new(self.y.reverse(), self.y, self.y) }
    pub fn ry_y_z(self) -> Self { Self::new(self.y.reverse(), self.y, self.z) }
    pub fn ry_y_rx(self) -> Self { Self::new(self.y.reverse(), self.y, self.x.reverse()) }
    pub fn ry_y_ry(self) -> Self { Self::new(self.y.reverse(), self.y, self.y.reverse()) }
    pub fn ry_y_rz(self) -> Self { Self::new(self.y.reverse(), self.y, self.z.reverse()) }
    pub fn ry_y_0(self) -> Self { Self::new(self.y.reverse(), self.y, T::ZERO) }
    pub fn ry_z_x(self) -> Self { Self::new(self.y.reverse(), self.z, self.x) }
    pub fn ry_z_y(self) -> Self { Self::new(self.y.reverse(), self.z, self.y) }
    pub fn ry_z_z(self) -> Self { Self::new(self.y.reverse(), self.z, self.z) }
    pub fn ry_z_rx(self) -> Self { Self::new(self.y.reverse(), self.z, self.x.reverse()) }
    pub fn ry_z_ry(self) -> Self { Self::new(self.y.reverse(), self.z, self.y.reverse()) }
    pub fn ry_z_rz(self) -> Self { Self::new(self.y.reverse(), self.z, self.z.reverse()) }
    pub fn ry_z_0(self) -> Self { Self::new(self.y.reverse(), self.z, T::ZERO) }
    pub fn ry_rx_x(self) -> Self { Self::new(self.y.reverse(), self.x.reverse(), self.x) }
    pub fn ry_rx_y(self) -> Self { Self::new(self.y.reverse(), self.x.reverse(), self.y) }
    pub fn ry_rx_z(self) -> Self { Self::new(self.y.reverse(), self.x.reverse(), self.z) }
    pub fn ry_rx_rx(self) -> Self { Self::new(self.y.reverse(), self.x.reverse(), self.x.reverse()) }
    pub fn ry_rx_ry(self) -> Self { Self::new(self.y.reverse(), self.x.reverse(), self.y.reverse()) }
    pub fn ry_rx_rz(self) -> Self { Self::new(self.y.reverse(), self.x.reverse(), self.z.reverse()) }
    pub fn ry_rx_0(self) -> Self { Self::new(self.y.reverse(), self.x.reverse(), T::ZERO) }
    pub fn ry_ry_x(self) -> Self { Self::new(self.y.reverse(), self.y.reverse(), self.x) }
    pub fn ry_ry_y(self) -> Self { Self::new(self.y.reverse(), self.y.reverse(), self.y) }
    pub fn ry_ry_z(self) -> Self { Self::new(self.y.reverse(), self.y.reverse(), self.z) }
    pub fn ry_ry_rx(self) -> Self { Self::new(self.y.reverse(), self.y.reverse(), self.x.reverse()) }
    pub fn ry_ry_ry(self) -> Self { Self::new(self.y.reverse(), self.y.reverse(), self.y.reverse()) }
    pub fn ry_ry_rz(self) -> Self { Self::new(self.y.reverse(), self.y.reverse(), self.z.reverse()) }
    pub fn ry_ry_0(self) -> Self { Self::new(self.y.reverse(), self.y.reverse(), T::ZERO) }
    pub fn ry_rz_x(self) -> Self { Self::new(self.y.reverse(), self.z.reverse(), self.x) }
    pub fn ry_rz_y(self) -> Self { Self::new(self.y.reverse(), self.z.reverse(), self.y) }
    pub fn ry_rz_z(self) -> Self { Self::new(self.y.reverse(), self.z.reverse(), self.z) }
    pub fn ry_rz_rx(self) -> Self { Self::new(self.y.reverse(), self.z.reverse(), self.x.reverse()) }
    pub fn ry_rz_ry(self) -> Self { Self::new(self.y.reverse(), self.z.reverse(), self.y.reverse()) }
    pub fn ry_rz_rz(self) -> Self { Self::new(self.y.reverse(), self.z.reverse(), self.z.reverse()) }
    pub fn ry_rz_0(self) -> Self { Self::new(self.y.reverse(), self.z.reverse(), T::ZERO) }
    pub fn ry_0_x(self) -> Self { Self::new(self.y.reverse(), T::ZERO, self.x) }
    pub fn ry_0_y(self) -> Self { Self::new(self.y.reverse(), T::ZERO, self.y) }
    pub fn ry_0_z(self) -> Self { Self::new(self.y.reverse(), T::ZERO, self.z) }
    pub fn ry_0_rx(self) -> Self { Self::new(self.y.reverse(), T::ZERO, self.x.reverse()) }
    pub fn ry_0_ry(self) -> Self { Self::new(self.y.reverse(), T::ZERO, self.y.reverse()) }
    pub fn ry_0_rz(self) -> Self { Self::new(self.y.reverse(), T::ZERO, self.z.reverse()) }
    pub fn ry_0_0(self) -> Self { Self::new(self.y.reverse(), T::ZERO, T::ZERO) }
    pub fn rz_x_x(self) -> Self { Self::new(self.z.reverse(), self.x, self.x) }
    pub fn rz_x_y(self) -> Self { Self::new(self.z.reverse(), self.x, self.y) }
    pub fn rz_x_z(self) -> Self { Self::new(self.z.reverse(), self.x, self.z) }
    pub fn rz_x_rx(self) -> Self { Self::new(self.z.reverse(), self.x, self.x.reverse()) }
    pub fn rz_x_ry(self) -> Self { Self::new(self.z.reverse(), self.x, self.y.reverse()) }
    pub fn rz_x_rz(self) -> Self { Self::new(self.z.reverse(), self.x, self.z.reverse()) }
    pub fn rz_x_0(self) -> Self { Self::new(self.z.reverse(), self.x, T::ZERO) }
    pub fn rz_y_x(self) -> Self { Self::new(self.z.reverse(), self.y, self.x) }
    pub fn rz_y_y(self) -> Self { Self::new(self.z.reverse(), self.y, self.y) }
    pub fn rz_y_z(self) -> Self { Self::new(self.z.reverse(), self.y, self.z) }
    pub fn rz_y_rx(self) -> Self { Self::new(self.z.reverse(), self.y, self.x.reverse()) }
    pub fn rz_y_ry(self) -> Self { Self::new(self.z.reverse(), self.y, self.y.reverse()) }
    pub fn rz_y_rz(self) -> Self { Self::new(self.z.reverse(), self.y, self.z.reverse()) }
    pub fn rz_y_0(self) -> Self { Self::new(self.z.reverse(), self.y, T::ZERO) }
    pub fn rz_z_x(self) -> Self { Self::new(self.z.reverse(), self.z, self.x) }
    pub fn rz_z_y(self) -> Self { Self::new(self.z.reverse(), self.z, self.y) }
    pub fn rz_z_z(self) -> Self { Self::new(self.z.reverse(), self.z, self.z) }
    pub fn rz_z_rx(self) -> Self { Self::new(self.z.reverse(), self.z, self.x.reverse()) }
    pub fn rz_z_ry(self) -> Self { Self::new(self.z.reverse(), self.z, self.y.reverse()) }
    pub fn rz_z_rz(self) -> Self { Self::new(self.z.reverse(), self.z, self.z.reverse()) }
    pub fn rz_z_0(self) -> Self { Self::new(self.z.reverse(), self.z, T::ZERO) }
    pub fn rz_rx_x(self) -> Self { Self::new(self.z.reverse(), self.x.reverse(), self.x) }
    pub fn rz_rx_y(self) -> Self { Self::new(self.z.reverse(), self.x.reverse(), self.y) }
    pub fn rz_rx_z(self) -> Self { Self::new(self.z.reverse(), self.x.reverse(), self.z) }
    pub fn rz_rx_rx(self) -> Self { Self::new(self.z.reverse(), self.x.reverse(), self.x.reverse()) }
    pub fn rz_rx_ry(self) -> Self { Self::new(self.z.reverse(), self.x.reverse(), self.y.reverse()) }
    pub fn rz_rx_rz(self) -> Self { Self::new(self.z.reverse(), self.x.reverse(), self.z.reverse()) }
    pub fn rz_rx_0(self) -> Self { Self::new(self.z.reverse(), self.x.reverse(), T::ZERO) }
    pub fn rz_ry_x(self) -> Self { Self::new(self.z.reverse(), self.y.reverse(), self.x) }
    pub fn rz_ry_y(self) -> Self { Self::new(self.z.reverse(), self.y.reverse(), self.y) }
    pub fn rz_ry_z(self) -> Self { Self::new(self.z.reverse(), self.y.reverse(), self.z) }
    pub fn rz_ry_rx(self) -> Self { Self::new(self.z.reverse(), self.y.reverse(), self.x.reverse()) }
    pub fn rz_ry_ry(self) -> Self { Self::new(self.z.reverse(), self.y.reverse(), self.y.reverse()) }
    pub fn rz_ry_rz(self) -> Self { Self::new(self.z.reverse(), self.y.reverse(), self.z.reverse()) }
    pub fn rz_ry_0(self) -> Self { Self::new(self.z.reverse(), self.y.reverse(), T::ZERO) }
    pub fn rz_rz_x(self) -> Self { Self::new(self.z.reverse(), self.z.reverse(), self.x) }
    pub fn rz_rz_y(self) -> Self { Self::new(self.z.reverse(), self.z.reverse(), self.y) }
    pub fn rz_rz_z(self) -> Self { Self::new(self.z.reverse(), self.z.reverse(), self.z) }
    pub fn rz_rz_rx(self) -> Self { Self::new(self.z.reverse(), self.z.reverse(), self.x.reverse()) }
    pub fn rz_rz_ry(self) -> Self { Self::new(self.z.reverse(), self.z.reverse(), self.y.reverse()) }
    pub fn rz_rz_rz(self) -> Self { Self::new(self.z.reverse(), self.z.reverse(), self.z.reverse()) }
    pub fn rz_rz_0(self) -> Self { Self::new(self.z.reverse(), self.z.reverse(), T::ZERO) }
    pub fn rz_0_x(self) -> Self { Self::new(self.z.reverse(), T::ZERO, self.x) }
    pub fn rz_0_y(self) -> Self { Self::new(self.z.reverse(), T::ZERO, self.y) }
    pub fn rz_0_z(self) -> Self { Self::new(self.z.reverse(), T::ZERO, self.z) }
    pub fn rz_0_rx(self) -> Self { Self::new(self.z.reverse(), T::ZERO, self.x.reverse()) }
    pub fn rz_0_ry(self) -> Self { Self::new(self.z.reverse(), T::ZERO, self.y.reverse()) }
    pub fn rz_0_rz(self) -> Self { Self::new(self.z.reverse(), T::ZERO, self.z.reverse()) }
    pub fn rz_0_0(self) -> Self { Self::new(self.z.reverse(), T::ZERO, T::ZERO) }
    pub fn _0_x_rx(self) -> Self { Self::new(T::ZERO, self.x, self.x.reverse()) }
    pub fn _0_x_ry(self) -> Self { Self::new(T::ZERO, self.x, self.y.reverse()) }
    pub fn _0_x_rz(self) -> Self { Self::new(T::ZERO, self.x, self.z.reverse()) }
    pub fn _0_y_rx(self) -> Self { Self::new(T::ZERO, self.y, self.x.reverse()) }
    pub fn _0_y_ry(self) -> Self { Self::new(T::ZERO, self.y, self.y.reverse()) }
    pub fn _0_y_rz(self) -> Self { Self::new(T::ZERO, self.y, self.z.reverse()) }
    pub fn _0_z_rx(self) -> Self { Self::new(T::ZERO, self.z, self.x.reverse()) }
    pub fn _0_z_ry(self) -> Self { Self::new(T::ZERO, self.z, self.y.reverse()) }
    pub fn _0_z_rz(self) -> Self { Self::new(T::ZERO, self.z, self.z.reverse()) }
    pub fn _0_rx_x(self) -> Self { Self::new(T::ZERO, self.x.reverse(), self.x) }
    pub fn _0_rx_y(self) -> Self { Self::new(T::ZERO, self.x.reverse(), self.y) }
    pub fn _0_rx_z(self) -> Self { Self::new(T::ZERO, self.x.reverse(), self.z) }
    pub fn _0_rx_rx(self) -> Self { Self::new(T::ZERO, self.x.reverse(), self.x.reverse()) }
    pub fn _0_rx_ry(self) -> Self { Self::new(T::ZERO, self.x.reverse(), self.y.reverse()) }
    pub fn _0_rx_rz(self) -> Self { Self::new(T::ZERO, self.x.reverse(), self.z.reverse()) }
    pub fn _0_rx_0(self) -> Self { Self::new(T::ZERO, self.x.reverse(), T::ZERO) }
    pub fn _0_ry_x(self) -> Self { Self::new(T::ZERO, self.y.reverse(), self.x) }
    pub fn _0_ry_y(self) -> Self { Self::new(T::ZERO, self.y.reverse(), self.y) }
    pub fn _0_ry_z(self) -> Self { Self::new(T::ZERO, self.y.reverse(), self.z) }
    pub fn _0_ry_rx(self) -> Self { Self::new(T::ZERO, self.y.reverse(), self.x.reverse()) }
    pub fn _0_ry_ry(self) -> Self { Self::new(T::ZERO, self.y.reverse(), self.y.reverse()) }
    pub fn _0_ry_rz(self) -> Self { Self::new(T::ZERO, self.y.reverse(), self.z.reverse()) }
    pub fn _0_ry_0(self) -> Self { Self::new(T::ZERO, self.y.reverse(), T::ZERO) }
    pub fn _0_rz_x(self) -> Self { Self::new(T::ZERO, self.z.reverse(), self.x) }
    pub fn _0_rz_y(self) -> Self { Self::new(T::ZERO, self.z.reverse(), self.y) }
    pub fn _0_rz_z(self) -> Self { Self::new(T::ZERO, self.z.reverse(), self.z) }
    pub fn _0_rz_rx(self) -> Self { Self::new(T::ZERO, self.z.reverse(), self.x.reverse()) }
    pub fn _0_rz_ry(self) -> Self { Self::new(T::ZERO, self.z.reverse(), self.y.reverse()) }
    pub fn _0_rz_rz(self) -> Self { Self::new(T::ZERO, self.z.reverse(), self.z.reverse()) }
    pub fn _0_rz_0(self) -> Self { Self::new(T::ZERO, self.z.reverse(), T::ZERO) }
    pub fn _0_0_rx(self) -> Self { Self::new(T::ZERO, T::ZERO, self.x.reverse()) }
    pub fn _0_0_ry(self) -> Self { Self::new(T::ZERO, T::ZERO, self.y.reverse()) }
    pub fn _0_0_rz(self) -> Self { Self::new(T::ZERO, T::ZERO, self.z.reverse()) }
}

impl<T : HaveZero + Copy> C<T> 
{ 
    pub fn x_x_x(self) -> Self { Self::new(self.x, self.x, self.x) }
    pub fn x_x_y(self) -> Self { Self::new(self.x, self.x, self.y) }
    pub fn x_x_z(self) -> Self { Self::new(self.x, self.x, self.z) }
    pub fn x_x_0(self) -> Self { Self::new(self.x, self.x, T::ZERO) }
    pub fn x_y_x(self) -> Self { Self::new(self.x, self.y, self.x) }
    pub fn x_y_y(self) -> Self { Self::new(self.x, self.y, self.y) }
    pub fn x_y_z(self) -> Self { Self::new(self.x, self.y, self.z) }
    pub fn x_y_0(self) -> Self { Self::new(self.x, self.y, T::ZERO) }
    pub fn x_z_x(self) -> Self { Self::new(self.x, self.z, self.x) }
    pub fn x_z_y(self) -> Self { Self::new(self.x, self.z, self.y) }
    pub fn x_z_z(self) -> Self { Self::new(self.x, self.z, self.z) }
    pub fn x_z_0(self) -> Self { Self::new(self.x, self.z, T::ZERO) }
    pub fn x_0_x(self) -> Self { Self::new(self.x, T::ZERO, self.x) }
    pub fn x_0_y(self) -> Self { Self::new(self.x, T::ZERO, self.y) }
    pub fn x_0_z(self) -> Self { Self::new(self.x, T::ZERO, self.z) }
    pub fn x_0_0(self) -> Self { Self::new(self.x, T::ZERO, T::ZERO) }
    pub fn y_x_x(self) -> Self { Self::new(self.y, self.x, self.x) }
    pub fn y_x_y(self) -> Self { Self::new(self.y, self.x, self.y) }
    pub fn y_x_z(self) -> Self { Self::new(self.y, self.x, self.z) }
    pub fn y_x_0(self) -> Self { Self::new(self.y, self.x, T::ZERO) }
    pub fn y_y_x(self) -> Self { Self::new(self.y, self.y, self.x) }
    pub fn y_y_y(self) -> Self { Self::new(self.y, self.y, self.y) }
    pub fn y_y_z(self) -> Self { Self::new(self.y, self.y, self.z) }
    pub fn y_y_0(self) -> Self { Self::new(self.y, self.y, T::ZERO) }
    pub fn y_z_x(self) -> Self { Self::new(self.y, self.z, self.x) }
    pub fn y_z_y(self) -> Self { Self::new(self.y, self.z, self.y) }
    pub fn y_z_z(self) -> Self { Self::new(self.y, self.z, self.z) }
    pub fn y_z_0(self) -> Self { Self::new(self.y, self.z, T::ZERO) }
    pub fn y_0_x(self) -> Self { Self::new(self.y, T::ZERO, self.x) }
    pub fn y_0_y(self) -> Self { Self::new(self.y, T::ZERO, self.y) }
    pub fn y_0_z(self) -> Self { Self::new(self.y, T::ZERO, self.z) }
    pub fn y_0_0(self) -> Self { Self::new(self.y, T::ZERO, T::ZERO) }
    pub fn z_x_x(self) -> Self { Self::new(self.z, self.x, self.x) }
    pub fn z_x_y(self) -> Self { Self::new(self.z, self.x, self.y) }
    pub fn z_x_z(self) -> Self { Self::new(self.z, self.x, self.z) }
    pub fn z_x_0(self) -> Self { Self::new(self.z, self.x, T::ZERO) }
    pub fn z_y_x(self) -> Self { Self::new(self.z, self.y, self.x) }
    pub fn z_y_y(self) -> Self { Self::new(self.z, self.y, self.y) }
    pub fn z_y_z(self) -> Self { Self::new(self.z, self.y, self.z) }
    pub fn z_y_0(self) -> Self { Self::new(self.z, self.y, T::ZERO) }
    pub fn z_z_x(self) -> Self { Self::new(self.z, self.z, self.x) }
    pub fn z_z_y(self) -> Self { Self::new(self.z, self.z, self.y) }
    pub fn z_z_z(self) -> Self { Self::new(self.z, self.z, self.z) }
    pub fn z_z_0(self) -> Self { Self::new(self.z, self.z, T::ZERO) }
    pub fn z_0_x(self) -> Self { Self::new(self.z, T::ZERO, self.x) }
    pub fn z_0_y(self) -> Self { Self::new(self.z, T::ZERO, self.y) }
    pub fn z_0_z(self) -> Self { Self::new(self.z, T::ZERO, self.z) }
    pub fn z_0_0(self) -> Self { Self::new(self.z, T::ZERO, T::ZERO) }
    pub fn _0_x_x(self) -> Self { Self::new(T::ZERO, self.x, self.x) }
    pub fn _0_x_y(self) -> Self { Self::new(T::ZERO, self.x, self.y) }
    pub fn _0_x_z(self) -> Self { Self::new(T::ZERO, self.x, self.z) }
    pub fn _0_x_0(self) -> Self { Self::new(T::ZERO, self.x, T::ZERO) }
    pub fn _0_y_x(self) -> Self { Self::new(T::ZERO, self.y, self.x) }
    pub fn _0_y_y(self) -> Self { Self::new(T::ZERO, self.y, self.y) }
    pub fn _0_y_z(self) -> Self { Self::new(T::ZERO, self.y, self.z) }
    pub fn _0_y_0(self) -> Self { Self::new(T::ZERO, self.y, T::ZERO) }
    pub fn _0_z_x(self) -> Self { Self::new(T::ZERO, self.z, self.x) }
    pub fn _0_z_y(self) -> Self { Self::new(T::ZERO, self.z, self.y) }
    pub fn _0_z_z(self) -> Self { Self::new(T::ZERO, self.z, self.z) }
    pub fn _0_z_0(self) -> Self { Self::new(T::ZERO, self.z, T::ZERO) }
    pub fn _0_0_x(self) -> Self { Self::new(T::ZERO, T::ZERO, self.x) }
    pub fn _0_0_y(self) -> Self { Self::new(T::ZERO, T::ZERO, self.y) }
    pub fn _0_0_z(self) -> Self { Self::new(T::ZERO, T::ZERO, self.z) }
    pub fn _0_0_0(self) -> Self { Self::new(T::ZERO, T::ZERO, T::ZERO) }
}

impl<T : ScalarPositive + CoordinateRequirements> CoordinateScalarPositive for C<T>
{
    fn length_squared(self) -> T { self.x*self.x + self.y*self.y + self.z*self.z }
    fn area(self) -> T { (op_component!(self, *)).abs() }
    
    #[inline] fn min_element(self) -> T { self.x.min_partial(self.y.min_partial(self.z)) }
    #[inline] fn max_element(self) -> T { self.x.max_partial(self.y.max_partial(self.z)) }
}

impl<T : CoordinateRequirements> Coordinate for C<T>
{
    type Precision = T;
    type SELF<Component> = C<Component>;
    const DIMENSION : usize = 3;

    #[inline] fn splat(xyz : T) -> Self { Self::splat_const(xyz) }

    #[inline] fn x(self) -> T { self.x }
    #[inline] fn y(self) -> T { self.y }
    #[inline] fn z(self) -> T { self.z }
    #[inline] fn w(self) -> T { ___() }

    #[inline] fn set_x(&mut self, x : T) -> &mut Self { self.x = x; self }
    #[inline] fn set_y(&mut self, y : T) -> &mut Self { self.y = y; self }
    #[inline] fn set_z(&mut self, z : T) -> &mut Self { self.z = z; self }
    #[inline] fn set_w(&mut self, _w : T) -> &mut Self { self }

    #[inline] fn with_x(mut self, x : T) -> Self { self.x = x; self }
    #[inline] fn with_y(mut self, y : T) -> Self { self.y = y; self }
    #[inline] fn with_z(mut self, z : T) -> Self { self.z = z; self }
    #[inline] fn with_w(self, _w : T) -> Self { self }

    #[inline] fn any<P>(self, p : P) -> bool where P : Fn(T) -> bool { p(self.x) || p(self.y) || p(self.z) }
    #[inline] fn all<P>(self, p : P) -> bool where P : Fn(T) -> bool { p(self.x) && p(self.y) && p(self.z) }

    #[inline] fn cmp_any<P>(self, other : Self, p : P) -> bool where P : Fn(T, T) -> bool 
    { p(self.x, other.x) || p(self.y, other.y) || p(self.z, other.z) }
    #[inline] fn cmp_all<P>(self, other : Self, p : P) -> bool where P : Fn(T, T) -> bool
    { p(self.x, other.x) && p(self.y, other.y) && p(self.z, other.z) }
    #[inline] fn cmp_map<F>(self, other : Self, f : F) -> Self where F : Fn(T, T) -> T
    { Self::new(f(self.x, other.x), f(self.y, other.y), f(self.z, other.z)) }

    #[inline] fn new_2d(x : T, y : T) -> Self { Self::new(x, y, ___()) }
    #[inline] fn new_3d(x : T, y : T, z : T) -> Self { Self::new(x, y, z) }
    #[inline] fn new_4d(x : T, y : T, z : T, _w : T) -> Self { Self::new(x, y, z) }

    #[inline] fn new_with<F>(f : F) -> Self where F : Fn(usize) -> T
    { Self::new(f(0), f(1), f(2)) }

    #[inline] fn map_intern<F>(self, f : F) -> Self where F : Fn(Self::Precision) -> Self::Precision { self.map(f) }
    #[inline] fn map<Z, F>(self, f : F) -> Self::SELF<Z> where F : Fn(T) -> Z { C::<Z>::new( f(self.x), f(self.y), f(self.z)) }
}

impl<T : Scalar> Moveable<T> for C<T>
{
    fn move_by(&mut self, v : impl Into<C4<T>>) -> &mut Self {
        let c : C4<T> = v.into();
        *self += c.to_c3();
        self
    }
}

impl<T : UnitArithmeticPositive> CoordinateUnitPositive for C<T>
{
    fn sum_axis(self) -> T { op_component!(self, +) }
    fn length_manhattan(self) -> T { self.abs().sum_axis() }
    
    const X : Self = Self::new(T::ONE, T::ZERO, T::ZERO);
    const Y : Self = Self::new(T::ZERO, T::ONE, T::ZERO);
    const Z : Self = Self::new(T::ZERO, T::ZERO, T::ONE);
    const W : Self = Self::new(T::ZERO, T::ZERO, T::ZERO);
}


impl<T : UnitArithmetic> C<T>
{
    const ADJACENT_DELTA : [Self; 6] = 
    [
        Self::new(T::ONE, T::ZERO, T::ZERO), Self::new(T::MINUS_ONE, T::ZERO, T::ZERO), 
        Self::new(T::ZERO, T::ONE, T::ZERO), Self::new(T::ZERO, T::MINUS_ONE, T::ZERO),
        Self::new(T::ZERO, T::ZERO, T::ONE), Self::new(T::ZERO, T::ZERO, T::MINUS_ONE),
    ];
    const DIAGONAL_DELTA : [Self; 20] = 
    [
        // All permutation ?
        Self::new(T::ONE, T::ONE, T::ZERO),
        Self::new(T::ONE, T::MINUS_ONE, T::ZERO),
        Self::new(T::MINUS_ONE, T::MINUS_ONE, T::ZERO),
        Self::new(T::MINUS_ONE, T::ONE, T::ZERO),

        Self::new(T::ONE, T::ZERO, T::ONE),
        Self::new(T::ONE, T::ZERO, T::MINUS_ONE),
        Self::new(T::MINUS_ONE, T::ZERO, T::MINUS_ONE),
        Self::new(T::MINUS_ONE, T::ZERO, T::ONE),

        Self::new(T::ZERO, T::ONE, T::ONE),
        Self::new(T::ZERO, T::ONE, T::MINUS_ONE),
        Self::new(T::ZERO, T::MINUS_ONE, T::MINUS_ONE),
        Self::new(T::ZERO, T::MINUS_ONE, T::ONE),

        // + edge. Like a logic true table
        Self::new(T::ONE, T::ONE, T::ONE),
        Self::new(T::ONE, T::ONE, T::MINUS_ONE),
        Self::new(T::ONE, T::MINUS_ONE, T::ONE),
        Self::new(T::ONE, T::MINUS_ONE, T::MINUS_ONE),
        Self::new(T::MINUS_ONE, T::ONE, T::ONE),
        Self::new(T::MINUS_ONE, T::ONE, T::MINUS_ONE),
        Self::new(T::MINUS_ONE, T::MINUS_ONE, T::ONE),
        Self::new(T::MINUS_ONE, T::MINUS_ONE, T::MINUS_ONE),
    ];

    /// ADJACENT_DELTA + DIAGONAL_DELTA
    const ORTHO_DIAGONAL_DELTA : [Self; 6 + 20] = 
    [
        Self::new(T::ONE, T::ZERO, T::ZERO), Self::new(T::MINUS_ONE, T::ZERO, T::ZERO), 
        Self::new(T::ZERO, T::ONE, T::ZERO), Self::new(T::ZERO, T::MINUS_ONE, T::ZERO),
        Self::new(T::ZERO, T::ZERO, T::ONE), Self::new(T::ZERO, T::ZERO, T::MINUS_ONE),

        Self::new(T::ONE, T::ONE, T::ZERO),
        Self::new(T::ONE, T::MINUS_ONE, T::ZERO),
        Self::new(T::MINUS_ONE, T::MINUS_ONE, T::ZERO),
        Self::new(T::MINUS_ONE, T::ONE, T::ZERO),

        Self::new(T::ONE, T::ZERO, T::ONE),
        Self::new(T::ONE, T::ZERO, T::MINUS_ONE),
        Self::new(T::MINUS_ONE, T::ZERO, T::MINUS_ONE),
        Self::new(T::MINUS_ONE, T::ZERO, T::ONE),

        Self::new(T::ZERO, T::ONE, T::ONE),
        Self::new(T::ZERO, T::ONE, T::MINUS_ONE),
        Self::new(T::ZERO, T::MINUS_ONE, T::MINUS_ONE),
        Self::new(T::ZERO, T::MINUS_ONE, T::ONE),

        // + edge. Like a logic true table
        Self::new(T::ONE, T::ONE, T::ONE),
        Self::new(T::ONE, T::ONE, T::MINUS_ONE),
        Self::new(T::ONE, T::MINUS_ONE, T::ONE),
        Self::new(T::ONE, T::MINUS_ONE, T::MINUS_ONE),
        Self::new(T::MINUS_ONE, T::ONE, T::ONE),
        Self::new(T::MINUS_ONE, T::ONE, T::MINUS_ONE),
        Self::new(T::MINUS_ONE, T::MINUS_ONE, T::ONE),
        Self::new(T::MINUS_ONE, T::MINUS_ONE, T::MINUS_ONE),
    ];
}

impl<T : UnitArithmetic> CoordinateUnit for C<T>
{
    const LEFT  : Self = Self::new(T::MINUS_ONE, T::ZERO, T::ZERO);
    const DOWN  : Self = Self::new(T::ZERO, T::MINUS_ONE, T::ZERO);
    const BACKWARD  : Self = Self::new(T::ZERO, T::ZERO, T::MINUS_ONE);

    fn orthogonal_delta() -> Self::OrthogonalArray { Self::ADJACENT_DELTA }
    type OrthogonalArray = [Self; 6];
    
    fn diagonal_delta() -> Self::DiagonalArray { Self::DIAGONAL_DELTA }
    type DiagonalArray = [Self; 20];
    
    fn ortho_diagonal_delta() -> Self::OrthoDiagonalArray { Self::ORTHO_DIAGONAL_DELTA }
    type OrthoDiagonalArray = [Self; 6 + 20];
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