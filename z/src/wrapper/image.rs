use macroquad::telemetry::sample_gpu_queries;

use crate::*;

#[derive(Clone, Debug)]
pub struct Image<Meta:Clone=()>
{
    pub(crate) img : LibImage,
    pub meta : Meta,
}

impl From<LibImage> for Image
{
    fn from(value: LibImage) -> Self {
        Self { img: value, meta : () }
    }
}

impl<Meta : Clone> Image<Meta>
{
    pub fn from_texture(texture : &Texture2D<Meta>) -> Self { Self { img: texture.val.get_texture_data(), meta : texture.meta.clone() } }

    pub fn new_transparent(size : Point2, meta : Meta) -> Self { Self::new(size, ColorBits::TRANSPARENT, meta) }
    pub fn new(size : Point2, color : ColorBits, meta : Meta) -> Self
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
            meta,
        }
    }

    pub fn size  (&self) -> Point2 { point2(self.img.width as int, self.img.height as int)}
    pub fn width (&self) -> isize  { self.size().x }
    pub fn height(&self) -> isize  { self.size().y }

    pub fn sub_image(&self, rect : Rect2i) -> Self
    {
        Self { img : self.img.sub_image(rect.to_float().to_lib()), meta : self.meta.clone() }
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

    pub fn to_texture2d(self) -> Texture2D<Meta> { Texture2D::new(LibTexture2D::from_image(&self.img), self.meta) }
}