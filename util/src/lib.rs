#![allow(unused_imports)]

pub use std::fmt::{Display, Debug, Write, Formatter, Result as DResult};
pub use std::ops::
{
    Add, AddAssign, 
    Sub, SubAssign,
    Div, DivAssign,
    Rem, RemAssign,
    Mul, MulAssign,
    Neg, 

    BitAnd, BitAndAssign, 
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,

    Shl, ShlAssign,
    Shr, ShrAssign,
    Not,

    Bound, RangeBounds, Range,

    Index, IndexMut,
};

use std::rc::{Rc, Weak};

pub use std::num::
{
    Wrapping, Saturating, 
    NonZero,
};


use std::sync::Mutex;
pub use std::{collections::{HashMap, HashSet, VecDeque, BinaryHeap}, default, marker::PhantomData};
pub use std::hash::Hash;
pub use std::mem;

#[macro_use(Deref, DerefMut)]
pub extern crate derive_more;

pub use derive_more::{Deref, DerefMut};

pub mod langage_extension;
pub use langage_extension::*;

pub mod container;
pub use container::*;

pub mod console;
pub use console::*;

pub mod length;
pub use length::*;