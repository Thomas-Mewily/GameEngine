// I did not use cargo because the code is not long, and I try to avoid dependency
// Code by https://github.com/dpc/rust-default

use std::collections::VecDeque;


/// A shorthand for `Default::default()`
///
/// See `https://internals.rust-lang.org/t/could-we-have-std-default/8756` for
/// discussion
#[allow(non_snake_case, unused_qualifications)]
pub fn ___<T:Default>()-> T {
    std::default::Default::default() // gosh, that's a lot of default, isn't it?
}

pub trait DefaultExtension
{
    #[allow(non_snake_case)]
    fn ___() -> Self;
}
impl<T:Default> DefaultExtension for T
{
    #[allow(non_snake_case)]
    fn ___() -> Self {

        Self::default()
    }
}

// Thanks to user@ctrl-alt-delor on `https://stackoverflow.com/questions/67872308/how-to-check-if-for-loop-is-on-the-last-element-of-an-iterator`
#[allow(clippy::wrong_self_convention)]
pub trait IterEndPeek
{ 
    fn is_last(&mut self) -> bool;
    fn is_not_last(&mut self) -> bool { !self.is_last() }
}
impl<I: Iterator> IterEndPeek for  std::iter::Peekable<I> 
{
    fn is_last(&mut self) -> bool { self.peek().is_none() }
}
/* */
#[macro_export]
macro_rules! vec_into  
{ 
    // Base case for empty input
    () => {
        Vec::new()
    };

    // Recursive case for non-empty input
    ($val:expr $(, $tokens:expr)*) => {{
        let mut vec = Vec::with_capacity(0 $(+ {let _ = stringify!($tokens); 1})*);
        vec.push($val.into());
        $( vec.push($tokens.into()); )*
        vec
    }};

    /* 
    // With extra comma
    ($val:expr $(, $tokens:expr)* ,) => {{
        let mut vec = Vec::with_capacity(0 $(+ {let _ = stringify!($tokens); 1})*);
        vec.push($val.into());
        $( vec.push($tokens.into()); )*
        vec
    }};
    */
}


pub trait ResultExpectedOrDisplay<T>
{
    fn expect_or_display(self) -> T;
}
impl<T,E: std::fmt::Display> ResultExpectedOrDisplay<T> for Result<T, E>
{
    fn expect_or_display(self) -> T {
        match self
        {
            Ok(v) => v,
            Err(err) => panic!("{}", err),
        }
    }
}


