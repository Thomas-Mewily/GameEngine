use super::*;

use macroquad::prelude;


#[derive(Debug)]
pub struct PenStat
{
    pub nb_clear      : usize,

    pub nb_rectangle  : usize,
    pub nb_triangle   : usize,

    pub nb_texture    : usize,

    pub text_len      : usize,
}
impl Default for PenStat { fn default() -> Self { Self::const_default() }}
impl PenStat
{
    pub const fn const_default() -> Self 
    {
        Self { nb_clear: 0, nb_rectangle: 0, nb_triangle: 0, nb_texture: 0, text_len: 0 }
    }
}

#[derive(Debug)]
pub struct Pen 
{
    stat : PenStat,

    background_color : Color,

    pub(crate) debug : Vec<String>,

    fonts : Vec<Font>,

    //#[deref] #[deref_mut]
    pub(crate) cam : CameraManager,
}
impl Default for Pen 
{ 
    fn default() -> Self 
    { 
        Self 
        { 
            stat: PenStat::const_default(), 
            background_color: Colored::WHITE,
            debug: Vec::new(), 
            fonts: Vec::new(),
            cam: ___(),  
        }
    }
}


impl Pen
{
    pub fn push_font(&mut self, f : &Font) { self.fonts.push(f.clone()); }
    pub fn pop_font(&mut self) { self.fonts.pop().expect("Can't pop the empty Font stack"); }
    pub fn font(&self) -> Option<&Font> { self.fonts.last() }
}


impl Pen
{
    pub fn stat(&self) -> &PenStat { &self.stat }

    pub fn reset_stat(&mut self) -> &mut Self { self.stat = ___(); self }
    

    pub(crate) fn end_frame(&mut self) 
    {
        self.cam.end_frame();
    }
    pub(crate) fn begin_frame(&mut self) 
    { 
        let bg_color = self.background_color();
        self.reset_stat();
        self.clear_background(bg_color);
    }

    pub fn clear_background(&mut self, c : Color) 
    {
        self.stat.nb_clear += 1;
        macroquad::prelude::clear_background(c.to_lib());
    }

    pub fn background_color(&self) -> Color { self.background_color }
    pub fn set_window_background_color(&mut self, window_background_color : Color) { self.background_color = window_background_color; }
}


/* 
struct DefaultMatrix;
impl macroquad::camera::Camera for DefaultMatrix
{
    fn matrix(&self) -> prelude::Mat4 { LibMat4::IDENTITY }
    fn depth_enabled(&self) -> bool { false }
    fn render_pass(&self) -> Option<prelude::RenderPass> { None }
    fn viewport(&self) -> Option<(i32, i32, i32, i32)> { None }
}

impl Pen
{
    pub fn default_matrix(&mut self) -> &mut Self { macroquad::camera::set_camera(&DefaultMatrix); self }
    pub fn push_default_matrix(&mut self) -> &mut Self { self.push(); self.reset_pen() macroquad::camera::set_camera(&DefaultMatrix); self }
}*/

/* 
pub trait Penable
{
    fn draw(&self) { self.draw_with_pen(pen()); }
    fn draw_with_pen(&self, pen : &mut Pen);
}
*/

impl Pen
{
    pub fn rect( &mut self, rect : Rect2f, color : Color) { self.rectangle(rect.pos, rect.size, color); }

    pub fn rectangle
    (
        &mut self, 
        pos : Vec2,
        size : Vec2,
        color : Color
    )
    {
        self.rectangle_ex(pos, size, zero(), zero(), color);
    }

    pub fn rectangle_ex
    (
        &mut self, 
        pos : Vec2,
        size : Vec2,
        center_coef: Vec2,
        angle : Angle,
        color : Color
    )
    {
        self.stat.nb_rectangle += 1;
        prelude::draw_rectangle_ex(pos.x as f32, pos.y as f32, size.x as f32, size.y as f32, macroquad::shapes::DrawRectangleParams{ offset: center_coef.to_lib(), rotation: angle.radian() as f32, color: color.to_lib() })
    }

    pub fn line(&mut self, src : impl Into<Vec2>, dest : impl Into<Vec2>, tickness : float, color : Color)
    {
        let src  : Vec2 = src.into();  
        let dest : Vec2 = dest.into();

        let direction = dest-src;
        self.rectangle_ex(src, vec2(tickness, direction.length()), vec2(0.5, 0.), direction.angle() - Angle::RIGHT, color);
    } 
}

impl Pen
{
    pub fn triangle(&mut self, p1 : Vec2, p2 : Vec2, p3 : Vec2, color : Color)
    { 
        self.stat.nb_triangle += 1;
        prelude::draw_triangle(Into::<Vec2>::into(p1).to_lib(), Into::<Vec2>::into(p2).to_lib(), Into::<Vec2>::into(p3).to_lib(), color.to_lib())
    }

    pub fn circle(&mut self, pos : Vec2, radius : float, color : Color) { self.ellipse(pos, Vec2::splat(radius), ANGLE_ZERO, color) }

    pub fn ellipse(&mut self, pos : Vec2, radius : Vec2, angle: Angle, color: Color) 
    { self.ellipse_with(pos, radius, angle, color, 48)}
    pub fn ellipse_with(&mut self, pos : Vec2, radius : Vec2, angle: Angle, color: Color, mut nb_triangle : usize) 
    {
        nb_triangle = nb_triangle.max(3);

        let mut rotation = angle;
        let rotation_inc = ANGLE_FULL / nb_triangle;

        let mut rotation_point = pos + rotation.to_vec2_normalized() * radius;

        for _ in 0..nb_triangle
        {
            rotation += rotation_inc;
            let old_angle_point = rotation_point;
            rotation_point = pos + rotation.to_vec2_normalized() * radius;
            self.triangle(pos, old_angle_point, rotation_point, color);
        }
    }
}

impl Pen
{
    pub fn texture 
    (
        &mut self, 
        texture : &Texture2D,
        pos : Vec2,
        size : Vec2,
        center_coef : Vec2,
        params : DrawTexture2D
    )
    {
        self.stat.nb_texture += 1;
        self._texture(texture, pos, size, center_coef, params);
    }

    fn _texture 
    (
        &self, 
        texture : &Texture2D,
        mut pos : Vec2,
        size : Vec2,
        mut center_coef : Vec2,
        params : DrawTexture2D
    )
    {
        //if params.flip.x { center_coef.x = 1. - center_coef.x; }
        //if params.flip.y { center_coef.y = 1. - center_coef.y; }

        pos -= center_coef * size;

        prelude::draw_texture_ex(&texture.val, pos.x as f32, pos.y as f32, params.color.to_lib(),macroquad::texture::DrawTextureParams 
        { 
            dest_size: Some(size.to_lib()),
            source: params.source.map(|r| r.to_lib()),
            rotation: params.angle.radian() as f32, 
            flip_x:  params.flip.x,
            flip_y: !params.flip.y,
            pivot: params.pivot.map(|p| p.to_lib()),
        })
    }
}


/// Copied from macroquad 
#[derive(Debug, Clone, Default)]
pub struct DrawFont<'a>
{
    pub font : Option<&'a Font>,
    pub angle: Angle,
}

impl<'a> DrawFont<'a>
{
    pub fn with_font(self, font : &'a Font) -> Self { self.with_optionnal_font(Some(font)) }
    pub fn with_optionnal_font(mut self, font : Option<&'a Font>) -> Self { self.font = font; self }
    pub fn with_angle(mut self, angle : Angle) -> Self { self.angle = angle; self }
}

pub trait PenTextDrawable { fn get_str(&self) -> &str; }
impl PenTextDrawable for &str { fn get_str(&self) -> &str { self }}
impl PenTextDrawable for String { fn get_str(&self) -> &str { self }}

impl Pen
{
    pub fn mesure_text(&self, text : &str, font : Option<&Font>, font_scale : float) -> Vec2 
    { self.mesure_text_with_fontsize(text, font, AssetsManager::FONT_DEFAULT_SIZE, font_scale / AssetsManager::FONT_DEFAULT_SIZE.to_float()) }
    pub fn mesure_text_with_fontsize(&self, text : &str, font : Option<&Font>, font_size : FontSize, font_scale : float) -> Vec2
    {
        let dim = prelude::measure_text(&text, font.map(|e| &e.font), font_size as u16, font_scale as f32);
        let (width, mut height) = (dim.width as float, dim.height as float);
        if !height.is_finite() 
        {
            height = font_size as float * font_scale;
        }
        vec2(width, height)
    }

    // Todo : do a pen_text!(fmt) macro to avoid managing the allocation (format!() will allocated at each frame...)
    pub fn text_ex<'a>(&mut self, text : impl PenTextDrawable, pos : Vec2, font_scale : float, center : Vec2, color: Color, params : DrawFont)
    {
        self._text_ex(text.get_str(), pos, font_scale, center, color, params)
    }

    fn _text_ex(&mut self, text : &str, mut pos : Vec2, mut font_scale : float, center : Vec2, color: Color, params : DrawFont)
    {
        self.stat.text_len += text.len();

        let font_size  = AssetsManager::FONT_DEFAULT_SIZE;
        font_scale /= font_size as float;

        let f = if params.font.is_none() { self.fonts.last() } else { params.font };

        let dim = self.mesure_text_with_fontsize(text, f, font_size, font_scale);

        pos -= dim * center;
        
        prelude::draw_text_ex(&text, pos.x as f32, pos.y as f32, 
            macroquad::text::TextParams
            {
                font_scale: -font_scale as f32,
                font_size : font_size,
                font_scale_aspect : -1.,
                color : color.to_lib(),
                font : f.map(|e| &e.font),
                rotation : params.angle.radian() as f32
            }
        );
    }
}