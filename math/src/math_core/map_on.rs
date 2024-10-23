pub use crate::zero_sized_number::*;


/// `i8`, `i16`, `i32`, `i64`, `isize`
#[macro_export]
macro_rules! map_on_signed {
    ($mac:ident) => {
        $mac!(i8);
        $mac!(i16);
        $mac!(i32);
        $mac!(i64);
        $mac!(isize);
    };
}

/// `u8`, `u16`, `u32`, `u64`, `usize`
#[macro_export]
macro_rules! map_on_unsigned {
    ($mac:ident) => {
        $mac!(u8);
        $mac!(u16);
        $mac!(u32);
        $mac!(u64);
        $mac!(usize);
    };
}

/// (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`)
#[macro_export]
macro_rules! map_on_integer {
    ($mac:ident) => {
        $crate::map_on_unsigned!($mac);
        $crate::map_on_signed!($mac);
    };
}

/// `f32`, `f64`
#[macro_export]
macro_rules! map_on_floating {
    ($mac:ident) => {
        $mac!(f32);
        $mac!(f64);
    };
}

/// (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`) + (`f32`, `f64`)
#[macro_export]
macro_rules! map_on_scalar {
    ($mac:ident) => 
    {
        $crate::map_on_integer!($mac);
        $crate::map_on_floating!($mac);
    };
}