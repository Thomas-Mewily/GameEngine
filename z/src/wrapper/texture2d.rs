use crate::*;

pub type Texture2DFilter = macroquad::texture::FilterMode;

#[derive(Clone, Debug, PartialEq)]
pub struct Texture2D<Meta:Clone=()> where Meta : Clone
{
    pub(crate) val : LibTexture2D,
    pub meta : Meta,
}

impl<Meta : Clone> Texture2D<Meta>
{
    pub(crate) fn new(val : LibTexture2D, meta : Meta) -> Self { Self { val, meta }}

    pub fn size(&self) -> Vec2 { self.val.size().to_engine() }
    pub fn width(&self) -> float { self.size().x }
    pub fn height(&self) -> float { self.size().y }

    pub fn set_filter(&mut self, filter_mode : Texture2DFilter) -> &mut Self { self.val.set_filter(filter_mode); self }
    pub fn to_image(&self) -> Image<Meta> { Image::<Meta>::from_texture(self) }
}
