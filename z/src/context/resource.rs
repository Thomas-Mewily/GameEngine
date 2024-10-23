use super::*;

/// stored on the disk
pub trait IPersistant : Clone + PartialEq
{}
impl IPersistant for () {}

pub trait IRes
{
    type Persistant : IPersistant;
}
impl IRes for () { type Persistant = (); }