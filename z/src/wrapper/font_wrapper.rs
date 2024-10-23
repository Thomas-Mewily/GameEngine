use crate::*;

/// A TTF font on the GPU
#[derive(Clone)]
pub struct Font
{
    pub(crate) font : LibFont,
}
impl Debug for Font
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult { write!(f, "Font") }
}
impl PartialEq for Font { fn eq(&self, _other: &Self) -> bool { true }}

pub type FontSize = u16;
pub type FontFilter = macroquad::texture::FilterMode;

impl Font
{
    pub fn populate_font_cache(&mut self, characters: &[char], font_size: FontSize) 
    { self.font.populate_font_cache(characters, font_size as u16); }

    pub fn set_filter(&mut self, filter_mode: FontFilter) 
    { self.font.set_filter(filter_mode); }
}
