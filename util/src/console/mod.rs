use std::io::BufRead;

use crate::*;

pub mod ansi_color;
pub use ansi_color::*;

pub fn read_ascii_lowercase_char() -> char
{
    use std::io::{Read, stdin};  
    let mut stdin_handle = stdin().lock();  
    let mut str = String::with_capacity(16);
    stdin_handle.read_line(&mut str).unwrap();
    (str.chars().next().unwrap() as char).to_ascii_lowercase()
}