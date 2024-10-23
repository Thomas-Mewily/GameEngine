use crate::*;

pub(crate) type Texture2DFilter = macroquad::texture::FilterMode;

#[derive(Clone, Debug, PartialEq)]
pub struct Texture2D
{
    pub(crate) val : LibTexture2D,
}

impl Texture2D
{
    pub(crate) fn new(val : LibTexture2D) -> Self { Self { val }}

    pub fn size(&self) -> Vec2 { self.val.size().to_engine() }
    pub fn width(&self) -> float { self.size().x }
    pub fn height(&self) -> float { self.size().y }

    //pub fn set_filter(&mut self, filter_mode : Texture2DFilter) -> &mut Self { self.val.set_filter(filter_mode); self }


    pub fn to_image(&self) -> Image { Image::from_texture(self) }
}

pub trait TextureParam
{
    /// set the anti aliasing
    fn set_aa(&mut self, aa : bool) -> &mut Self;
}

impl TextureParam for Texture2D
{
    fn set_aa(&mut self, aa : bool) -> &mut Self { self.val.set_filter(if aa { Texture2DFilter::Linear } else { Texture2DFilter::Nearest }); self }
}

impl ImportFromRaw for Texture2D
{
    type ImportRawError=();

    fn from_raw(raw : &[u8]) -> Result<Self, Self::ImportRawError> 
    {
        // Todo : handle error in encoding here
        let v = macroquad::texture::Texture2D::from_file_with_format(raw, None);
        Ok(Texture2D::new(v))
    }
}
