use crate::*;

/// Copied from macroquad 
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DrawTexture2D 
{
    pub color : Color,

    /// Part of texture to draw. If None - draw the whole texture.
    /// Good use example: drawing an image from texture atlas.
    /// Is None by default
    pub source: Option<Rect2f>,

    /// Rotation in radians
    pub angle: Angle,

    /// Mirror on the `X / Y` axis
    pub flip : Bool2,

    /// Rotate around this point.
    /// When `None`, rotate around the texture's center.
    /// When `Some`, the coordinates are in screen-space.
    /// E.g. pivot (0,0) rotates around the top left corner of the screen, not of the
    /// texture.
    pub pivot: Option<Vec2>,
}
impl DrawTexture2D
{
    pub const fn with_source(mut self, source : Option<Rect2f>) -> Self { self.source = source; self }

    pub const fn with_flip(mut self, flip : Bool2) -> Self { self.flip = flip; self }
    pub const fn with_flip_x(mut self, flip_x : bool) -> Self { self.flip.x = flip_x; self }
    pub const fn with_flip_y(mut self, flip_y : bool) -> Self { self.flip.y = flip_y; self }

    pub const fn with_angle(mut self, angle : Angle) -> Self { self.angle = angle; self }

    pub const fn with_pivot(mut self, pivot : Option<Vec2>) -> Self { self.pivot = pivot; self }
    pub const fn with_color(mut self, color : Color) -> Self { self.color = color; self }
}