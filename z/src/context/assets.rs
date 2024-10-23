use super::*;

#[derive(Debug)]
pub struct AssetsManager 
{
}

impl Default for AssetsManager
{
    fn default() -> Self { Self::const_default() }
}

impl AssetsManager
{
    pub const FONT_DEFAULT_SIZE : FontSize = 96;

    pub(crate) const fn const_default() -> Self { Self {  }}
}