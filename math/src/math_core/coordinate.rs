use std::ops::*;
use crate::*;

pub trait CoordinateRequirements : Sized + Copy + Debug + Default {}
impl<T> CoordinateRequirements for T
    where T : Sized + Copy + Debug + Default {}

pub trait Coordinate : CoordinateRequirements
{
    type Precision : Copy;
    type SELF<Component>;

    const DIMENSION : usize;

    fn splat(val : Self::Precision) -> Self;
    
    fn new_with<F>(f : F) -> Self where F : Fn(usize) -> Self::Precision;

    fn new_2d(x : Self::Precision, y : Self::Precision) -> Self;
    fn new_3d(x : Self::Precision, y : Self::Precision, z : Self::Precision) -> Self;
    fn new_4d(x : Self::Precision, y : Self::Precision, z : Self::Precision, w : Self::Precision) -> Self;

    /// Zero if the component don't exist
    fn x(self) -> Self::Precision;
    /// Zero if the component don't exist
    fn y(self) -> Self::Precision;
    /// Zero if the component don't exist
    fn z(self) -> Self::Precision;
    /// Zero if the component don't exist
    fn w(self) -> Self::Precision;

    /// Do nothings if the component don't exist
    fn set_x(&mut self, x : Self::Precision) -> &mut Self;
    /// Do nothings if the component don't exist
    fn set_y(&mut self, y : Self::Precision) -> &mut Self;
    /// Do nothings if the component don't exist
    fn set_z(&mut self, z : Self::Precision) -> &mut Self; 
    /// Do nothings if the component don't exist
    fn set_w(&mut self, w : Self::Precision) -> &mut Self;

    /// Do nothings if the component don't exist
    fn with_x(self, x : Self::Precision) -> Self;
    /// Do nothings if the component don't exist
    fn with_y(self, y : Self::Precision) -> Self;
    /// Do nothings if the component don't exist
    fn with_z(self, z : Self::Precision) -> Self;
    /// Do nothings if the component don't exist
    fn with_w(self, w : Self::Precision) -> Self;


    fn x_or(self, val : Self::Precision) -> Self::Precision { if self.have_x() { self.x() } else { val }}
    fn y_or(self, val : Self::Precision) -> Self::Precision { if self.have_y() { self.y() } else { val }}
    fn z_or(self, val : Self::Precision) -> Self::Precision { if self.have_z() { self.z() } else { val }}
    fn w_or(self, val : Self::Precision) -> Self::Precision { if self.have_w() { self.w() } else { val }}

    fn x_or_one(self) -> Self::Precision where Self::Precision : HaveOne { self.x_or(Self::Precision::ONE) }
    fn y_or_one(self) -> Self::Precision where Self::Precision : HaveOne { self.y_or(Self::Precision::ONE) }
    fn z_or_one(self) -> Self::Precision where Self::Precision : HaveOne { self.z_or(Self::Precision::ONE) }
    fn w_or_one(self) -> Self::Precision where Self::Precision : HaveOne { self.w_or(Self::Precision::ONE) }

    fn have_x(&self) -> bool { Self::DIMENSION >= 1 }
    fn have_y(&self) -> bool { Self::DIMENSION >= 2 }
    fn have_z(&self) -> bool { Self::DIMENSION >= 3 }
    fn have_w(&self) -> bool { Self::DIMENSION >= 4 }

    /// At least one
    /// 
    /// True if the predicate is true for at least one component
    fn any<P>(self, p : P) -> bool where P : Fn(Self::Precision) -> bool;

    /// True for all.
    ///
    /// True if the predicate is true for all component
    fn all<P>(self, p : P) -> bool where P : Fn(Self::Precision) -> bool { !self.any(|a| !p(a))}
    
    fn map_intern<F>(self, f : F) -> Self where F : Fn(Self::Precision) -> Self::Precision;
    fn map<Z, F>(self, f : F) -> Self::SELF<Z> where F : Fn(Self::Precision) -> Z;
    /// Create a new Self from 2 Self
    fn cmp_map<F>(self, other : Self, f : F) -> Self where F : Fn(Self::Precision, Self::Precision) -> Self::Precision;

    /// At least one
    /// 
    /// True if the predicate is true for at least one component
    fn cmp_any<P>(self, other : Self, p : P) -> bool where P : Fn(Self::Precision, Self::Precision) -> bool;
    /// True for all.
    ///
    /// True if the predicate is true for all component
    fn cmp_all<P>(self, other : Self, p : P) -> bool where P : Fn(Self::Precision, Self::Precision) -> bool { !self.cmp_any(other, |a, b| !p(a, b))}

    // generic parameters may not be used in const operations
    // type parameters may not be used in const expressions :
    // fn to_array(&self) -> [T; Self::DIMENSION];
    // fn from_array...

    //fn iter<'a>(&self) -> impl Iterator<Item = &T> + 'a;
    //fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + '_;

    /// Fill non existing component with Zero
    fn to_c2(self) -> C2<Self::Precision> { C2::<Self::Precision>::new(self.x(), self.y()) }
    /// Fill non existing component with Zero
    fn to_c3(self) -> C3<Self::Precision> { C3::<Self::Precision>::new(self.x(), self.y(), self.z()) }
    /// Fill non existing component with Zero
    fn to_c4(self) -> C4<Self::Precision> { C4::<Self::Precision>::new(self.x(), self.y(), self.y(), self.z()) }

    /// Fill non existing component with the given value
    fn to_c2_with(self, val : Self::Precision) -> C2<Self::Precision> where Self::Precision : HaveOne { C2::<Self::Precision>::new(self.x_or(val), self.y_or(val)) }
    /// Fill non existing component with the given value
    fn to_c3_with(self, val : Self::Precision) -> C3<Self::Precision> where Self::Precision : HaveOne { C3::<Self::Precision>::new(self.x_or(val), self.y_or(val), self.z_or(val)) }
    /// Fill non existing component with the given value
    fn to_c4_with(self, val : Self::Precision) -> C4<Self::Precision> where Self::Precision : HaveOne { C4::<Self::Precision>::new(self.x_or(val), self.y_or(val), self.y_or(val), self.z_or(val)) }

    /// Fill non existing component with One
    fn to_c2_with_one(self) -> C2<Self::Precision> where Self::Precision : HaveOne { self.to_c2_with(Self::Precision::ONE)}
    /// Fill non existing component with One
    fn to_c3_with_one(self) -> C3<Self::Precision> where Self::Precision : HaveOne { self.to_c3_with(Self::Precision::ONE)}
    /// Fill non existing component with One
    fn to_c4_with_one(self) -> C4<Self::Precision> where Self::Precision : HaveOne { self.to_c4_with(Self::Precision::ONE)}

    fn have_length(self) -> bool where Self::Precision : HaveZero + PartialEq { self.any(|c| c.is_non_zero()) }
















}


pub trait CoordinateUnitPositive : Coordinate<Precision : UnitArithmeticPositive> + UnitArithmeticPositive
    //where 
    //Self::Precision : UnitArithmetic + PartialEq
{
    /// `x + y + z ...`
    fn sum_axis(self) -> Self::Precision;

    fn length_manhattan(self) -> Self::Precision;

    const X : Self;
    const Y : Self;
    const Z : Self;
    const W : Self;

    /// + X
    const RIGHT : Self = Self::X;
    /// + Y
    const UP    : Self = Self::Y;
    /// + Z
    const FORWARD   : Self = Self::Z;

    fn right(self) -> Self { self + Self::X }
    fn left(self) -> Self { self - Self::X }

    fn up(self) -> Self { self + Self::Y }
    fn down(self) -> Self { self - Self::Y }

    fn forward(self) -> Self { self + Self::Z }
    fn backward(self) -> Self { self - Self::Z }
}


pub trait CoordinateUnit : CoordinateUnitPositive<Precision : UnitArithmetic> + UnitArithmetic
{
    /// - X
    const LEFT : Self;
    /// - Y
    const DOWN : Self;
    /// - Z
    const BACKWARD : Self;

    /// Define the adjacent tile delta of a point, 
    /// where the Manhattan distance is one
    /// 
    /// ```md
    ///    | X | 
    /// ---+---+---
    ///  X | . | X
    /// ---+---+---
    ///    | X |
    /// ```
    fn   orthogonal_delta() -> Self::OrthogonalArray;
    type OrthogonalArray : Index<usize, Output = Self> + Length + IntoIterator<Item = Self>;
    /// Define the adjacent tile delta of a point, 
    /// where the Manhattan distance is one
    /// 
    /// ```md
    ///    | X | 
    /// ---+---+---
    ///  X | . | X
    /// ---+---+---
    ///    | X |
    /// ```
    fn iter_orthogonal(self) -> impl Iterator<Item = Self> { Self::orthogonal_delta().into_iter().map(move |delta| self + delta) }
    

    /// Define the diagonal tile delta of a point,
    /// where the tile have exactly one point in common
    /// 
    /// ```md
    ///  X |   | X
    /// ---+---+---
    ///    | . |  
    /// ---+---+---
    ///  X |   | X
    /// ```
    fn   diagonal_delta() -> Self::DiagonalArray;
    type DiagonalArray : Index<usize, Output = Self> + Length + IntoIterator<Item = Self>;
    /// Define the diagonal tile delta of a point,
    /// where the tile have exactly one point in common
    /// 
    /// ```md
    ///  X |   | X
    /// ---+---+---
    ///    | . |  
    /// ---+---+---
    ///  X |   | X
    /// ```
    fn iter_diagonal(self) -> impl Iterator<Item = Self> { Self::diagonal_delta().into_iter().map(move |delta| self + delta) }

    

    /// Combine orthogonal and diagonal delta
    /// where the Chebyshev/Moore neighborhood/Conway neighborhood distance is one
    /// 
    /// ```md
    ///  X | X | X
    /// ---+---+---
    ///  X | . | X
    /// ---+---+---
    ///  X | X | X
    /// ```
    fn   ortho_diagonal_delta() -> Self::OrthoDiagonalArray;
    type OrthoDiagonalArray : Index<usize, Output = Self> + Length + IntoIterator<Item = Self>;
    /// Combine orthogonal and diagonal delta
    /// where the Chebyshev/Moore neighborhood/Conway neighborhood distance is one
    /// 
    /// ```md
    ///  X | X | X
    /// ---+---+---
    ///  X | . | X
    /// ---+---+---
    ///  X | X | X
    /// ```
    fn iter_ortho_diagonal(self) -> impl Iterator<Item = Self> { Self::ortho_diagonal_delta().into_iter().map(move |delta| self + delta) }
}

// Coordinate of floating/float
pub trait CoordinateFloating : CoordinateScalar<Precision : FloatingNumber>
    //where 
    //Self::Precision : FloatingNumber
{
    fn length(self) -> Self::Precision { self.length_squared().sqrt() }
    fn normalized(self) -> Self 
    { 
        if self.have_length() 
        { 
            self / Self::splat(self.length())
        } 
        else 
        { 
            Self::ZERO
        }
    }

    /// set the angle of the `(x, y)` axis
    fn set_angle(&mut self, angle : Angle) -> &mut Self;
    /// Using the `(x, y)` axis
    fn angle(self) -> Angle;
    
    fn normalize(&mut self) { *self = self.normalized(); }
    fn set_length(&mut self, length : Self::Precision) { *self = self.normalized() * Self::splat(length);  }
    fn with_length(mut self, length : Self::Precision) -> Self { self.set_length(length); self }
}


pub trait CoordinateIndex : CoordinateInteger<Precision = int> + CoordinateScalar<Precision : Integer>
{
    fn is_inside(self, size : Self) -> bool { self.cmp_all(size, |a, m| a >= Self::Precision::ZERO && a < m) }
    fn is_outside(self, size : Self) -> bool { !self.is_inside(size) }

    fn to_idx(self, size : Self) -> Option<Self::Precision> { self.is_inside(size).then(|| self.to_idx_unchecked(size)) }
    fn to_idx_unchecked(self, size : Self) -> Self::Precision 
    {
        self.x() 
        + self.y() * size.x() 
        + self.z() * (size.x() * size.y()) 
        + self.w() * (size.x() * size.y() * size.z())
    }

    fn from_idx(idx : Self::Precision, size : Self) -> Option<Self> { (idx < size.area()).then(|| Self::from_idx_unchecked(idx, size)) }
    fn from_idx_unchecked(idx : Self::Precision, size : Self) -> Self
    {
        Self::new_4d(
            if size.have_x() { idx % size.x() } else { Self::Precision::ZERO }, 
            if size.have_y() { idx / size.x() % size.y() } else { Self::Precision::ZERO }, 
            if size.have_z() { idx / (size.x() * size.y()) % size.z() } else { Self::Precision::ZERO }, 
            if size.have_w() { idx / (size.x() * size.y() * size.z()) % size.w() } else { Self::Precision::ZERO }
        )
    }
}
impl<C : CoordinateInteger<Precision = int> + CoordinateScalar<Precision : Integer>> CoordinateIndex for C{}


/// for int and uint
pub trait CoordinateInteger : Sized + CoordinateScalarPositive<Precision : Integer> + AreaIter<Index=Self>{}
impl<C : Sized + CoordinateScalarPositive<Precision : Integer> + AreaIter<Index=C>> CoordinateInteger for C{}

pub trait AreaIter
{
    type Index;

    /// The last value is included.
    /// 
    /// Do not use it for index, use iter_idx() instead
    fn iter_area(self) -> impl Iterator<Item = Self::Index>;
    /// Last value is excluded because it is an index
    fn iter_idx(self) -> impl Iterator<Item = Self::Index>;
}

/// For unsigned + signed + floating...
pub trait CoordinateScalarPositive : CoordinateUnitPositive<Precision : ScalarPositive> + NumberArithmeticPositive + Rounding + Clamp
{
    fn length_squared(self) -> Self::Precision;
    fn area(self) -> Self::Precision;
    //fn perimeter(self) -> Self::Precision;

    fn min_element(self) -> Self::Precision;
    fn max_element(self) -> Self::Precision;

    fn min(self, other : Self) -> Self { self.cmp_map(other, |a, b| a.min_partial(b)) }
    fn max(self, other : Self) -> Self { self.cmp_map(other, |a, b| a.max_partial(b)) }

    fn to_unsigned(self) -> Self::SELF<<Self::Precision as ToUnsigned>::UnsignedEquivalent> { self.map(|c| c.to_unsigned()) }
    fn to_signed (self) -> Self::SELF<<Self::Precision as ToSigned>::SignedEquivalent> { self.map(|c| c.to_signed()) }

    fn to_uint (self) -> Self::SELF<uint>  { self.map(|c| c.to_uint()) }
    fn to_int  (self) -> Self::SELF<int>   { self.map(|c| c.to_int()) }
    fn to_float(self) -> Self::SELF<float> { self.map(|c| c.to_float()) }


    fn to_upoint2(self) -> C2<uint> { C2::<uint>::new(self.x().to_uint(), self.y().to_uint())}
    fn to_upoint3(self) -> C3<uint> { C3::<uint>::new(self.x().to_uint(), self.y().to_uint(), self.z().to_uint())}
    fn to_upoint4(self) -> C4<uint> { C4::<uint>::new(self.x().to_uint(), self.y().to_uint(), self.z().to_uint() , self.w().to_uint())}

    fn to_point2(self) -> C2<int> { C2::<int>::new(self.x().to_int(), self.y().to_int())}
    fn to_point3(self) -> C3<int> { C3::<int>::new(self.x().to_int(), self.y().to_int(), self.z().to_int())}
    fn to_point4(self) -> C4<int> { C4::<int>::new(self.x().to_int(), self.y().to_int(), self.z().to_int() , self.w().to_int())}

    fn to_vec2(self) -> C2<float> { C2::<float>::new(self.x().to_float(), self.y().to_float())}
    fn to_vec3(self) -> C3<float> { C3::<float>::new(self.x().to_float(), self.y().to_float(), self.z().to_float())}
    fn to_vec4(self) -> C4<float> { C4::<float>::new(self.x().to_float(), self.y().to_float(), self.z().to_float() , self.w().to_float())}
}

pub trait CoordinateScalar : CoordinateUnit<Precision : Scalar> + CoordinateScalarPositive{}
impl<T> CoordinateScalar for T
    where T : CoordinateUnit<Precision : Scalar> + CoordinateScalarPositive
{}

impl<C : Coordinate<Precision : CoordinateRequirements + Rounding>> Rounding for C
{
    fn floor(self) -> Self { self.map_intern(|v| v.floor()) }
    fn round(self) -> Self { self.map_intern(|v| v.round()) }
    fn ceil (self) -> Self { self.map_intern(|v| v.ceil()) }

    fn trunc(self) -> Self { self.map_intern(|v| v.trunc()) }
}
impl<C : Coordinate<Precision : CoordinateRequirements + PartialOrd + HaveZero>> Clamp for C
{
    fn clamp(self, max_inclued : Self) -> Self { self.cmp_map(max_inclued, |a, m| a.clamp_partial(C::Precision::ZERO, m)) }
}





