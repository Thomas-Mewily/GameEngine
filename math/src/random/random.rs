use crate::*;

macro_rules! rand_method {
    ($fn_name : ident, $type_name : ident) => {
        fn $fn_name (&mut self) -> $type_name { $type_name ::rand(self) }
    };

    ($type_name : ident) => {
        fn $type_name (&mut self) -> $type_name { $type_name ::rand(self) }
    };
}

/// 64 bits Random
pub trait IRandom : Default
{
    fn new_unseed() -> Self { ___() }
    fn new(seed : u64) -> Self;


    // The only one to overload
    fn u64(&mut self) -> u64;
    rand_method!(u32);
    rand_method!(u16);
    rand_method!(u8);

    /// Between `[0.0 .. 1.0]`
    fn coef(&mut self) -> Coef { Coef::rand(self) }

    /// Between `[0.0 .. 1.0[`
    fn coef_excluded(&mut self) -> Coef 
    { 
        /*
            let mut a = u64::MAX;
            let b = u64::MAX;
            loop
            {
                a -= 1;
                let a_f = a as f64;
                let b_f = b as f64;
                if a_f / b_f < 1.0 { break; }
            }
            println!("{} {} : {}", a, b, b - a);
        */

        /* 
            let mut d = 0.;
            let a = u64::MAX;
            let b = u64::MAX;
            loop
            {
                d += 1.0;
                let a_f = (a as f64) - d;
                let b_f = b as f64;
                if a_f / b_f < 1.0 { break; }
            }
            println!("{} {} : {}", a as f64, b as f64, d);
        */

        /* 
            let mut d = 0.;
            let a = u64::MAX;
            let b = u64::MAX;
            loop
            {
                d += 1.0;
                let a_f = a as f64;
                let b_f = b as f64 + d;
                if a_f / b_f < 1.0 { break; }
            }
            println!("{} {} : {}", a as f64, b as f64 + d, d);
            // d need to be at least 2049
        */
        //let c_f64 = (self.u64() as f64) / ((u64::MAX as f64) + 2049.0);
        //let c=  c_f64 as float;

        let mut c;
        loop
        {
            c = self.u64().to_float() / u64::MAX.to_float();
            if c < 1.0 { break; }
        }
        //let c=  (self.u32() as f32) / ((u32::MAX as f32) + 1.);
        debug_assert!(c >= 0., "random coef_excluded() to low : {c}" );
        debug_assert!(c <  1., "random coef_excluded() to high : {c}");
        c
    }

    /// Between `[-1.0 .. 1.0]`
    fn signed_coef(&mut self) -> float { self.coef() * 2. - 1. }


    rand_method!(vec2, Vec2);

    rand_method!(usize);
    rand_method!(isize);

    fn uint (&mut self) -> uint { self.usize() }
    fn int  (&mut self) -> int  { self.isize() }

    rand_method!(i64);
    rand_method!(i32);
    rand_method!(i16);
    rand_method!(i8);

    rand_method!(bool);

    rand_method!(angle, Angle);

    /// -1 or 1
    fn sign<T : HaveOne + HaveMinusOne>(&mut self) -> T  
    {  if self.bool() { T::ONE } else { T::MINUS_ONE } }

    /// Unsigned Sign
    /// 
    /// 0 or 1
    fn usign<T : HaveOne + HaveZero>(&mut self) -> T  
    {  if self.bool() { T::ONE } else { T::ZERO } }

    fn generate<T : FromRand<Self>>(&mut self) -> T { T::rand(self) }


    fn range<T : Scalar + SmallestIncrement, Range : RangeBounds<T>>(&mut self, range: Range) -> T { range.rand_inside(self) }

    fn inside<R : RandInside<Self>>(&mut self, r: &R) -> R::RandInside where <R as RandInside<Self>>::RandInside: Sized { r.rand_inside(self) }

    /// return a normalized vec2 (length = 1) with a random direction
    fn vec2_direction(&mut self) -> Vec2 { self.angle().to_vec2_normalized() }
    /// return a vec2 where each axis is in `[0. .. 1.]`
    fn vec2_coef(&mut self) -> Vec2 { Vec2::new(self.coef(), self.coef()) }
    /// return a vec2 where each axis is in `[-1. .. 1.]`
    fn vec2_signed_coef(&mut self) -> Vec2 { Vec2::new(self.signed_coef(), self.signed_coef()) }
    
}
 
pub trait FromRand<R : IRandom> { fn rand(r : &mut R) -> Self; }

impl<R : IRandom> FromRand<R> for bool   { fn rand(r : &mut R) -> Self { r.u64() % 2 == 0 }}

impl<R : IRandom> FromRand<R> for u64    { fn rand(r : &mut R) -> Self { r.u64() }}
impl<R : IRandom> FromRand<R> for u32    { fn rand(r : &mut R) -> Self { r.u64() as u32 }}
impl<R : IRandom> FromRand<R> for u16    { fn rand(r : &mut R) -> Self { r.u64() as u16 }}
impl<R : IRandom> FromRand<R> for u8     { fn rand(r : &mut R) -> Self { r.u64() as u8 }}

impl<R : IRandom> FromRand<R> for i64    { fn rand(r : &mut R) -> Self { i64::from_ne_bytes(r.u64().to_ne_bytes()) }}
impl<R : IRandom> FromRand<R> for i32    { fn rand(r : &mut R) -> Self { r.i64() as i32 }}
impl<R : IRandom> FromRand<R> for i16    { fn rand(r : &mut R) -> Self { r.i64() as i16 }}
impl<R : IRandom> FromRand<R> for i8     { fn rand(r : &mut R) -> Self { r.i64() as i8 }}

impl<R : IRandom> FromRand<R> for usize  { fn rand(r : &mut R) -> Self { r.u64() as usize }}
impl<R : IRandom> FromRand<R> for isize  { fn rand(r : &mut R) -> Self { isize::from_ne_bytes(r.usize().to_ne_bytes()) }}

impl<R : IRandom> FromRand<R> for Coef   { fn rand(r : &mut R) -> Self { r.u64().to_float() / u64::MAX.to_float() }}
impl<R : IRandom> FromRand<R> for Angle  { fn rand(r : &mut R) -> Self { Angle::from_turn(r.coef()) }}

impl<R : IRandom, T : FromRand<R>> FromRand<R> for C2<T> { fn rand(r : &mut R) -> Self { Self::new(T::rand(r), T::rand(r)) }}
impl<R : IRandom, T : FromRand<R>> FromRand<R> for C3<T> { fn rand(r : &mut R) -> Self { Self::new(T::rand(r), T::rand(r), T::rand(r)) }}
impl<R : IRandom, T : FromRand<R>> FromRand<R> for C4<T> { fn rand(r : &mut R) -> Self { Self::new(T::rand(r), T::rand(r), T::rand(r), T::rand(r)) }}

impl<R : IRandom, T : FromRand<R>> FromRand<R> for Rect<T> { fn rand(r : &mut R) -> Self { Self::new(T::rand(r), T::rand(r)) }}

pub trait RandInside<R : IRandom, Extra=()> { type RandInside : ?Sized; fn rand_inside(&self, r : &mut R) -> Self::RandInside; }

impl<R : IRandom, Ra, T> RandInside<R, T> for Ra
where 
    Ra : RangeBounds<T>,
    T : ScalarPositive + SmallestIncrement
{ 
    type RandInside = T;

    fn rand_inside(&self, r : &mut R) -> T 
    { 
        let start_included = match self.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + T::SMALL_INC,
            Bound::Unbounded => T::MIN_VAL,
        };
        let end_inclued = match self.end_bound() {
            Bound::Included(&e) => e,
            Bound::Excluded(&e) => e - T::SMALL_INC,
            Bound::Unbounded => T::MAX_VAL,
        };
    
        assert!(start_included <= end_inclued, "Random : invalid range : {start_included:?} was not <= {end_inclued:?}");
    
        let range_size = end_inclued - start_included;
        let c = r.coef();
        let nb = start_included + T::from_float(c * range_size.to_float());

        assert!(nb >= start_included, "random generated number was too low : {nb:?} for [{start_included:?}, {end_inclued:?}] with coef {c}");
        assert!(nb <= end_inclued, "random generated number was too hight : {nb:?} for [{start_included:?}, {end_inclued:?}] with coef {c}");
        nb
    }
}


impl<R : IRandom, T> RandInside<R, ()> for Rect<C2<T>> 
    where
    T : Scalar + SmallestIncrement,
{
    type RandInside=C2<T>;
    fn rand_inside(&self, r : &mut R) -> Self::RandInside 
    { 
        self.pos() + C2::new(
            (T::ZERO.. self.size.x).rand_inside(r), 
            (T::ZERO.. self.size.y).rand_inside(r),
        )
    }
}

impl<R : IRandom, T> RandInside<R, ()> for Rect<C3<T>> 
    where
    T : Scalar + SmallestIncrement,
{
    type RandInside=C3<T>;
    fn rand_inside(&self, r : &mut R) -> Self::RandInside 
    { 
        self.pos() + C3::new(
            (T::ZERO.. self.size.x).rand_inside(r), 
            (T::ZERO.. self.size.y).rand_inside(r),
            (T::ZERO.. self.size.z).rand_inside(r),
        )
    }
}

impl<R : IRandom, T> RandInside<R, ()> for Rect<C4<T>> 
    where
    T : Scalar + SmallestIncrement,
{
    type RandInside=C4<T>;
    fn rand_inside(&self, r : &mut R) -> Self::RandInside 
    { 
        self.pos() + C4::new(
            (T::ZERO.. self.size.x).rand_inside(r), 
            (T::ZERO.. self.size.y).rand_inside(r),
            (T::ZERO.. self.size.z).rand_inside(r),
            (T::ZERO.. self.size.w).rand_inside(r),
        )
    }
}

/* 
impl<R : IRandom, C : CoordinateNumber + FromRand<R>> RandInside<R> for Rectangle<C> where <C as CoordinateUnit>::Precision : Number 
{
    type RandInside=C;
    fn rand_inside(&self, r : &mut R) -> Self::RandInside 
    { 
        self.size() + self.pos()
    }
}
    */


impl<R : IRandom, T : Copy> RandInside<R> for [T]
{
    type RandInside=T;
    fn rand_inside(&self, r : &mut R) -> Self::RandInside { self[(0.. self.len()).rand_inside(r)] }
}