use crate::*;


pub trait LibToEngine { type Associate; fn to_engine(self) -> Self::Associate; }
pub trait EngineToLib { type Associate; fn to_lib(self) -> Self::Associate; }


impl LibToEngine for LibVec2 { type Associate = Vec2; #[inline] fn to_engine(self) -> Self::Associate { Self::Associate::new(self.x as float, self.y as float) }  }
impl EngineToLib for Vec2 { type Associate = LibVec2; #[inline] fn to_lib(self) -> Self::Associate { Self::Associate::new(self.x as f32, self.y as f32) }  }

impl LibToEngine for LibVec3 { type Associate = Vec3; #[inline] fn to_engine(self) -> Self::Associate { Self::Associate::new(self.x as float, self.y as float, self.z as float) }  }
impl EngineToLib for Vec3 { type Associate = LibVec3; #[inline] fn to_lib(self) -> Self::Associate { Self::Associate::new(self.x as f32, self.y as f32, self.z as f32) }  }

impl LibToEngine for LibVec4 { type Associate = Vec4; #[inline] fn to_engine(self) -> Self::Associate { Self::Associate::new(self.x as float, self.y as float, self.z as float, self.w as float) } }
impl EngineToLib for Vec4 { type Associate = LibVec4; #[inline] fn to_lib(self) -> Self::Associate { Self::Associate::new(self.x as f32, self.y as f32, self.z as f32, self.w as f32) } }

impl LibToEngine for LibRect { type Associate = Rect2f; #[inline] fn to_engine(self) -> Self::Associate { Self::Associate::new(vec2(self.x as float, self.y as float), vec2(self.w as float, self.h as float).into()) } }
impl EngineToLib for Rect2f { type Associate = LibRect; #[inline] fn to_lib(self) -> Self::Associate { Self::Associate::new(self.pos.x as f32 , self.pos.y as f32, self.size.x as f32, self.size.y as f32) } }

impl LibToEngine for LibColor { type Associate = Color; #[inline] fn to_engine(self) -> Self::Associate { Self::Associate::new(self.r as float, self.g as float, self.b as float, self.a as float) } }
impl EngineToLib for Color { type Associate = LibColor; #[inline] fn to_lib(self) -> Self::Associate { Self::Associate::new(self.r as f32, self.g as f32, self.b as f32, self.a as f32) } }

impl LibToEngine for LibImage { type Associate = Image; #[inline] fn to_engine(self) -> Self::Associate { Image::from(self) } }
impl EngineToLib for Image { type Associate = LibImage; #[inline] fn to_lib(self) -> Self::Associate { self.img } }

/// Glam is used by macroquad for the render matrix
pub(crate) trait ToGlam
{
    type Associate;
    fn to_glam(self) -> Self::Associate;
}
impl ToGlam for Vec2
{
    type Associate = LibGlam::Vec2;
    fn to_glam(self) -> Self::Associate {
        LibGlam::vec2(self.x as f32, self.y as f32)
    }
}
impl ToGlam for Vec3
{
    type Associate = LibGlam::Vec3;
    fn to_glam(self) -> Self::Associate {
        LibGlam::vec3(self.x as f32, self.y as f32, self.z as f32)
    }
}
impl ToGlam for Vec4
{
    type Associate = LibGlam::Vec4;
    fn to_glam(self) -> Self::Associate {
        LibGlam::vec4(self.x as f32, self.y as f32, self.z as f32, self.w as f32)
    }
}