use crate::*;

pub(crate) use macroquad::prelude::glam::f32 as LibGlam; 

pub(crate) type LibVec2 = macroquad::prelude::Vec2;
pub(crate) type LibVec3 = macroquad::prelude::Vec3;
pub(crate) type LibVec4 = macroquad::prelude::Vec4;

pub(crate) type LibMat2 = macroquad::prelude::Mat2;
pub(crate) type LibMat3 = macroquad::prelude::Mat3;
pub(crate) type LibMat4 = macroquad::prelude::Mat4;

pub(crate) use macroquad::prelude::Camera as LibCamera;
pub(crate) type LibRenderPass = macroquad::prelude::RenderPass;
pub(crate) type LibViewport = (LibViewportComposite, LibViewportComposite, LibViewportComposite, LibViewportComposite);
pub(crate) type LibViewportComposite = i32;

pub(crate) type LibCam2 = macroquad::prelude::Camera2D;
pub(crate) type LibCam3 = macroquad::prelude::Camera3D;

pub(crate) type LibRect = macroquad::prelude::Rect;
pub(crate) type LibColor = macroquad::prelude::Color;

pub(crate) type LibTexture2D = macroquad::texture::Texture2D;

pub(crate) type LibImage = macroquad::texture::Image;
pub type ImageFormat = macroquad::prelude::ImageFormat;

pub(crate) type LibFont = macroquad::text::Font;

pub(crate) type LibConfig = macroquad::window::Conf;
pub type Config = LibConfig;

pub(crate) type LibIcon = macroquad::miniquad::conf::Icon;
pub type Icon = LibIcon;


pub mod basic_wrapper;
pub use basic_wrapper::*;

pub mod texture2d;
pub use texture2d::*;

pub mod texture2d_draw;
pub use texture2d_draw::*;

pub mod image;
pub use image::*;

pub mod font_wrapper;
pub use font_wrapper::*;

pub mod icon;
pub use icon::*;

pub mod animation;
pub use animation::*;