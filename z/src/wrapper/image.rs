use macroquad::telemetry::sample_gpu_queries;

use crate::*;

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Image
{
    pub(crate) img : LibImage,
}

impl From<LibImage> for Image
{
    fn from(value: LibImage) -> Self {
        Self { img: value }
    }
}

impl Image
{
    pub fn from_texture(texture : &Texture2D) -> Self { Self { img: texture.val.get_texture_data() } }

    pub fn new_transparent(size : Point2) -> Self { Self::new(size, ColorBits::TRANSPARENT) }
    pub fn new(size : Point2, color : ColorBits) -> Self
    {
        let val : [u8; 4] = color.into();
        let area = size.area();
        let mut bytes = Vec::with_capacity(area as usize * 4);

        for _ in 0..area
        {
            for b in val
            {
                bytes.push(b);
            }
        }

        Self
        {
            img : LibImage
            {
                bytes,
                width: size.x as u16,
                height: size.y as u16,
            },
        }
    }

    pub fn size  (&self) -> Point2 { point2(self.img.width as int, self.img.height as int)}
    pub fn width (&self) -> int  { self.size().x }
    pub fn height(&self) -> int  { self.size().y }

    pub fn sub_image(&self, rect : Rect2i) -> Self
    {
        Self { img : self.img.sub_image(rect.to_float().to_lib()) }
    }

    pub fn get_color(&self, pos : Point2) -> Color
    {
        self.img.get_pixel(pos.x as u32, pos.y as u32).to_engine()
    }

    pub fn get(&self, pos : Point2) -> ColorBits
    {
        self.img.get_image_data()[(pos.y * self.width() + pos.x) as usize].into()
    }

    pub fn set_color(&mut self, pos : Point2, color : Color) -> &mut Self
    {
        self.img.set_pixel(pos.x as u32, pos.y as u32, color.to_lib());
        self
    }

    pub fn set(&mut self, pos : Point2, color : ColorBits) -> &mut Self { self.set_color(pos, color.into()) }

    pub fn save_as_png(&self, path : &str) { self.img.export_png(path); }

    pub fn to_texture2d(self) -> Texture2D { Texture2D::new(LibTexture2D::from_image(&self.img)) }
}

impl ImportFromRaw for Image
{
    type ImportRawError=macroquad::Error;

    fn from_raw(raw : &[u8]) -> Result<Self, Self::ImportRawError> 
    {
        macroquad::texture::Image::from_file_with_format(raw, None).map(|v| Image { img: v })
    }
}