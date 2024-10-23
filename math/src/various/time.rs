use std::{default, fmt::{Debug, Display, Formatter, Result}, ops::*};
use crate::*;

pub trait TimeExtension : ToFloat + Sized
{
    fn ms(self) -> Time  { Time::from_ms(self.to_float()) }
    fn s(self) -> Time { Time::from_s(self.to_float()) }
    fn mins(self) -> Time { Time::from_min(self.to_float()) }
    fn hours(self) -> Time { Time::from_hours(self.to_float()) }
    fn days(self) -> Time { Time::from_days(self.to_float()) }
}
impl<T:ToFloat> TimeExtension for T {}

/* 
pub trait TimeTickExtension : Sized
{
    fn ticks(self) -> Tick;
}
impl TimeTickExtension for int 
{
    fn ticks(self) -> Tick { Tick::_from_ticks(self) }
}
//pub type Tick = Time<int>;

pub type Turn = int;

pub type Tick = Time<Turn>;*/
pub type DeltaTime = Time;

#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
pub struct Time<T : UnitPositive = float> { _tick : T }

impl<T : FloatingNumber> Time<T>
{
    /// don't display the value if zero
    fn display_non_zero_unit(f: &mut Formatter<'_>, val : int, unit : &str) -> Result 
    { if val != 0 {  Self::display_unit(f, val, unit)?; write!(f, " ") } else { Ok(())} }

    fn display_unit(f: &mut Formatter<'_>, val : int, unit : &str) -> Result 
    { write!(f, "{}{}", val, unit) }
}

impl<T : FloatingNumber> Display for Time<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result 
    {
        if self.is_zero() { return write!(f, "0s"); }

        if self.is_strictly_negative() { write!(f, "-")?; }

        Self::display_non_zero_unit(f, self.timer_days(), "d")?;
        Self::display_non_zero_unit(f, self.timer_hours(), "h")?;
        Self::display_non_zero_unit(f, self.timer_mins(), "m")?;
        Self::display_non_zero_unit(f, self.timer_s(), "s")?;
        Self::display_non_zero_unit(f, self.timer_ms(), "ms")?;
        Ok(())
    }
}

impl<T : UnitPositive> Time<T>
{
    pub const fn _from_ticks(ticks : T) -> Self { Self { _tick: ticks } }
    pub const fn _ticks(self) -> T { self._tick }
}

impl<T : UnitPositive>    HaveZero     for Time<T> { const ZERO : Self = Self::_from_ticks(T::ZERO); }
impl<T : UnitPositive>    HaveOne      for Time<T> { const ONE  : Self = Self::_from_ticks(T::ONE ); }
impl<T : Unit> HaveMinusOne for Time<T> { const MINUS_ONE  : Self = Self::_from_ticks(T::MINUS_ONE); }

impl<T : UnitPositive> Absolute for Time<T> { fn abs(self) -> Self { Self::_from_ticks(self._tick.abs()) }}
impl<T : UnitPositive> SmallestIncrement for Time<T>  { const SMALL_INC  : Self = Self::_from_ticks(T::SMALL_INC); }


impl<T : FloatingNumber> Time<T>
{
    /// milliseconds
    /// ```
    /// use math::*;
    /// debug_assert_eq!(1000.ms(), 1.s());
    /// ```
    pub fn from_ms (ms : T) -> Self { Self { _tick: ms } }
    /// milliseconds
    /// ```
    /// use math::*;
    /// debug_assert_eq!(1.s().ms(), 1000.);
    /// ```
    pub fn ms(self) -> T { self._tick }
    /// whole milliseconds
    /// ```
    /// use math::*;
    /// debug_assert_eq!(0.5.ms().whole_ms(), 0);
    /// debug_assert_eq!(1.0.ms().whole_ms(), 1);
    /// debug_assert_eq!(1.5.ms().whole_ms(), 1);
    /// debug_assert_eq!(1.9.ms().whole_ms(), 1);
    /// debug_assert_eq!(2.0.ms().whole_ms(), 2);
    /// 
    /// debug_assert_eq!(-0.5.ms().whole_ms(),  0);
    /// debug_assert_eq!(-2.0.ms().whole_ms(), -2);
    /// debug_assert_eq!(-1.9.ms().whole_ms(), -1);
    /// ```
    pub fn whole_ms(self) -> int { self.ms().round_toward_from_zero().to_int() }
    /// Can be used to display milliseconds in a timer
    /// ```
    /// use math::*;
    /// debug_assert_eq!(10.5.ms().timer_ms(), 10);
    /// 
    /// debug_assert_eq!(999.ms().timer_ms(), 999);
    /// debug_assert_eq!(1000.ms().timer_ms(), 0);
    /// debug_assert_eq!(1001.ms().timer_ms(), 1);
    /// debug_assert_eq!(2005.ms().timer_ms(), 5);
    /// ```
    pub fn timer_ms(self) -> int { (self.ms().abs().floor() % T::from_usize(1000)).to_int() }

    /// seconds
    /// ```
    /// use math::*;
    /// debug_assert_eq!(60.s(), 1.mins());
    /// ```
    pub fn from_s  (s  : T) -> Self { Self { _tick: s * T::from_usize(1000) } }
    /// total seconds
    /// ```
    /// use math::*;
    /// debug_assert_eq!(1.mins().s(), 60.);
    /// ```
    pub fn s(self) -> T { self._tick / T::from_usize(1000) }
    /// whole seconds
    /// ```
    /// use math::*;
    /// debug_assert_eq!(0.5.s().whole_s(), 0);
    /// debug_assert_eq!(1.0.s().whole_s(), 1);
    /// debug_assert_eq!(1.5.s().whole_s(), 1);
    /// debug_assert_eq!(1.9.s().whole_s(), 1);
    /// debug_assert_eq!(2.0.s().whole_s(), 2);
    /// ```
    pub fn whole_s(self) -> int { self.s().round_toward_from_zero().to_int() }
    /// Can be used to display seconds in a timer
    /// 
    /// ```
    /// use math::*;
    /// debug_assert_eq!(10.5.s().timer_s(), 10);
    /// 
    /// debug_assert_eq!(59.s().timer_s(), 59);
    /// debug_assert_eq!(60.s().timer_s(), 0);
    /// debug_assert_eq!(61.s().timer_s(), 1);
    /// debug_assert_eq!(125.s().timer_s(), 5);
    /// ```
    pub fn timer_s(self) -> int { (self.s().abs().floor() % T::from_usize(60)).to_int() }

    /// minutes
    /// ```
    /// use math::*;
    /// debug_assert_eq!(60.mins(), 1.hours());
    /// ```
    pub fn from_min(min : T) -> Self { Self { _tick: min * T::from_usize(1000 * 60) } }
    /// minutes
    /// ```
    /// use math::*;
    /// debug_assert_eq!(1.hours().mins(), 60.);
    /// ```
    pub fn mins(self) -> T { self._tick / T::from_usize(1000 * 60) }
    /// whole minutes
    /// ```
    /// use math::*;
    /// debug_assert_eq!(0.5.mins().whole_mins(), 0);
    /// debug_assert_eq!(1.0.mins().whole_mins(), 1);
    /// debug_assert_eq!(1.5.mins().whole_mins(), 1);
    /// debug_assert_eq!(1.9.mins().whole_mins(), 1);
    /// debug_assert_eq!(2.0.mins().whole_mins(), 2);
    /// 
    /// debug_assert_eq!(-0.5.mins().whole_mins(),  0);
    /// debug_assert_eq!(-2.0.mins().whole_mins(), -2);
    /// debug_assert_eq!(-1.9.mins().whole_mins(), -1);
    /// ```
    pub fn whole_mins(self) -> int { self.mins().round_toward_from_zero().to_int() }
    /// Can be used to display mins in a timer
    /// ```
    /// use math::*;
    /// debug_assert_eq!(10.5.mins().timer_mins(), 10);
    /// 
    /// debug_assert_eq!(59.mins().timer_mins(), 59);
    /// debug_assert_eq!(60.mins().timer_mins(), 0);
    /// debug_assert_eq!(61.mins().timer_mins(), 1);
    /// debug_assert_eq!(125.mins().timer_mins(), 5);
    /// ```
    pub fn timer_mins(self) -> int { (self.mins().abs().floor() % T::from_usize(60)).to_int() }

    /// hours
    /// ```
    /// use math::*;
    /// debug_assert_eq!(24.hours(), 1.days());
    /// ```
    pub fn from_hours(hours : T) -> Self { Self { _tick: hours * T::from_usize(1000 * 60 * 60) } }
    /// hours
    /// ```
    /// use math::*;
    /// debug_assert_eq!(1.days().hours(), 24.);
    /// ```
    pub fn hours(self) -> T { self._tick / T::from_usize(1000 * 60 * 60) }
    /// whole hours
    /// ```
    /// use math::*;
    /// debug_assert_eq!(0.5.hours().whole_hours(), 0);
    /// debug_assert_eq!(1.0.hours().whole_hours(), 1);
    /// debug_assert_eq!(1.5.hours().whole_hours(), 1);
    /// debug_assert_eq!(1.9.hours().whole_hours(), 1);
    /// debug_assert_eq!(2.0.hours().whole_hours(), 2);
    /// 
    /// debug_assert_eq!(-0.5.hours().whole_hours(),  0);
    /// debug_assert_eq!(-2.0.hours().whole_hours(), -2);
    /// debug_assert_eq!(-1.9.hours().whole_hours(), -1);
    /// ```
    pub fn whole_hours(self) -> int { self.hours().round_toward_from_zero().to_int() }
    /// Can be used to display hours in a timer
    /// ```
    /// use math::*;
    /// debug_assert_eq!(10.5.hours().timer_hours(), 10);
    /// 
    /// debug_assert_eq!(23.hours().timer_hours(), 23);
    /// debug_assert_eq!(24.hours().timer_hours(), 0);
    /// debug_assert_eq!(25.hours().timer_hours(), 1);
    /// debug_assert_eq!((48+5).hours().timer_hours(), 5);
    /// ```
    pub fn timer_hours(self) -> int { (self.hours().abs().floor() % T::from_usize(24)).to_int() }

    /// days
    /// ```
    /// use math::*;
    /// debug_assert_eq!(1.days(), (3600*24).s());
    /// ```
    pub fn from_days(hours : T) -> Self { Self { _tick: hours * T::from_usize(1000 * 60 * 60 * 24) } }
    /// days
    /// ```
    /// use math::*;
    /// debug_assert_eq!(24.hours().days(), 1.);
    /// ```
    pub fn days(self) -> T { self._tick / T::from_usize(1000 * 60 * 60 * 24) }
    /// whole days
    /// ```
    /// use math::*;
    /// debug_assert_eq!(0.5.days().whole_days(), 0);
    /// debug_assert_eq!(1.0.days().whole_days(), 1);
    /// debug_assert_eq!(1.5.days().whole_days(), 1);
    /// debug_assert_eq!(1.9.days().whole_days(), 1);
    /// debug_assert_eq!(2.0.days().whole_days(), 2);
    /// 
    /// debug_assert_eq!(-0.5.days().whole_days(),  0);
    /// debug_assert_eq!(-2.0.days().whole_days(), -2);
    /// debug_assert_eq!(-1.9.days().whole_days(), -1);
    /// ```
    pub fn whole_days(self) -> int { self.days().round_toward_from_zero().to_int() }
    /// Can be used to display days in a timer
    /// ```
    /// use math::*;
    /// debug_assert_eq!(10.5.days().timer_days(), 10);
    /// 
    /// debug_assert_eq!(364.days().timer_days(), 364);
    /// debug_assert_eq!(365.days().timer_days(), 365);
    /// debug_assert_eq!(366.days().timer_days(), 366);
    /// debug_assert_eq!(900.days().timer_days(), 900);
    /// ```
    pub fn timer_days(self) -> int { self.days().abs().floor().to_int() }
}

impl<T : Integer> Time<T>
{
    /// for turn/tick based time
    pub fn from_turn(turn : T) -> Self { Self::_from_ticks(turn) }
    /// for turn/tick based time
    pub fn turn(self) -> T { self._tick }
}


impl<T : UnitPositive> Add<Time<T>> for Time<T> { type Output=Self; fn add(self, rhs: Self) -> Self::Output { Self::_from_ticks(self._tick + rhs._tick) }}
impl<T : UnitPositive> AddAssign<Time<T>> for Time<T> { fn add_assign(&mut self, rhs: Time<T>) { self._tick.add_assign(rhs._tick) }}

impl<T : UnitPositive> Sub<Time<T>> for Time<T> { type Output=Self; fn sub(self, rhs: Self) -> Self::Output { Self::_from_ticks(self._tick - rhs._tick) }}
impl<T : UnitPositive> SubAssign<Time<T>> for Time<T> { fn sub_assign(&mut self, rhs: Self) { self._tick.sub_assign(rhs._tick) }}

impl<T : NumberPositive> Div<Time<T>> for Time<T> { type Output=T; fn div(self, rhs: Self) -> Self::Output { self._tick / rhs._tick } }
impl<T : NumberPositive> DivAssign<T> for Time<T> { fn div_assign(&mut self, rhs: T) { self._tick.div_assign(rhs) }}

impl<T : NumberPositive> Mul<T> for Time<T> { type Output=Self; fn mul(self, rhs: T) -> Self::Output { Self::_from_ticks(self._tick.mul(rhs)) }}
impl<T : NumberPositive> MulAssign<T> for Time<T> { fn mul_assign(&mut self, rhs: T) { self._tick.mul_assign(rhs) }}

impl<T : NumberPositive> Div<T> for Time<T> { type Output=Time<T>; fn div(self, rhs: T) -> Self::Output { Self::_from_ticks(self._tick / rhs) } }
impl<T : NumberPositive> Rem<T> for Time<T> { type Output=Time<T>; fn rem(self, rhs: T) -> Self::Output { Self::_from_ticks(self._tick % rhs) } }


impl<T : Reversable + UnitPositive> Reversable for Time<T> { fn reverse(self) -> Self { Self::_from_ticks(self._tick.reverse()) } }





/* 
/// Define a definition for a smallest increment for a type. `1` for integer, 0.0 for float.
pub trait DefaultDeltaTime
{ 
    const DEFAULT_DELTA_INV  : Self; 
    fn default_delta_time() -> Self;
}
macro_rules! impl_default_delta_int {
    ($primitive_name: ty) => 
    { 
        impl DefaultDeltaTime for $primitive_name 
        { 
            const DEFAULT_DELTA_INV : Self = Self::ONE;
            fn default_delta_time() -> Self { Self::ONE }
        }
    };
}
map_on_integer!(impl_default_delta_int);
macro_rules! impl_default_delta_float {
    ($primitive_name: ty) => 
    { 
        impl DefaultDeltaTime for $primitive_name 
        { 
            const DEFAULT_DELTA_INV : Self = 60.;
            fn default_delta_time() -> Self { 1000. / Self::DEFAULT_DELTA_INV }
        }
    };
}
map_on_non_zero_floating!(impl_default_delta_float);
impl DefaultDeltaTime for f0 { const DEFAULT_DELTA_INV  : Self = Self; fn default_delta_time() -> Self { Self } }

impl<T : UnitPositive> DefaultDeltaTime for Time<T> 
{ 
    const DEFAULT_DELTA_INV  : Self = Self::_from_ticks(T::DEFAULT_DELTA_INV); 
    fn default_delta_time() -> Self { Self::_from_ticks(T::default_delta_time()) }
}
*/




/* 
impl<S : ToFloat> MulAssign<S> for TimeClock { fn mul_assign(&mut self, rhs: S) { self._tick.mul_assign(rhs.to_float()) }}
impl<S : ToFloat> Mul<S> for TimeClock
{
    type Output=TimeClock;
    fn mul(self, rhs: S) -> Self::Output {
        Self::from_ms(self._tick * rhs.to_float())
    }
}

impl<S : ToFloat> Div<S> for TimeClock
{
    type Output=TimeClock;
    fn div(self, rhs: S) -> Self::Output {
        Self::from_ms(self._tick / rhs.to_float())
    }
}



impl Debug for TimeClock
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{:.3?} ms", self.ms()) }
}

impl TimeClock
{
    pub fn fmt_total(self, precision : Option<PrecisionSnap>, postfix : &'static str) -> DisplayGameTime
    { DisplayGameTime { time:self, precision, postfix }}
    
    pub fn fmt_ms(self) -> DisplayGameTime { self.fmt_total( None, " ms") }
}
impl Display for TimeClock { fn fmt(&self, f: &mut Formatter<'_>) -> Result { self.fmt_ms().fmt(f) }}

#[derive(Clone)]
pub struct PrecisionSnap
{
    div : float,
    mul : float,
}

#[derive(Clone)]
pub struct DisplayGameTime{ time : TimeClock, precision : Option<PrecisionSnap>, postfix : &'static str }

impl Display for DisplayGameTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result 
    {
        write!(f, "{}{}",
            match &self.precision {
                Some(p) => (self.time.ms() / p.div).round()*p.mul,
                None => self.time.ms(),
            }, 
        self.postfix)
    }
}*/