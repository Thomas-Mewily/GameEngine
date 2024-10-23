use super::*;

#[derive(Debug, Clone)]
pub struct Camera
{
    pub(crate) matrix     : LibMat4,
    pub(crate) inv_matrix : LibMat4,

    pub(crate) have_depth : bool,

    pub(crate) viewport   : Rect2f,
    //pub(crate) viewport   : Option<Rect2i>,
}

impl Camera 
{
    pub(crate) const fn new(have_depth : bool, viewport : Rect2f) -> Self
    {
        Self { matrix : LibMat4::IDENTITY, inv_matrix : LibMat4::IDENTITY, have_depth, viewport : viewport }
    }

    pub const fn new_2d(viewport : Rect2f) -> Self { Self::new(false, viewport) }
    pub const fn new_3d(viewport : Rect2f) -> Self { Self::new(true, viewport) }

    pub fn screen_to_world<T : From<Vec3> + Into<Vec3>>(&self, pos : T, screen_size : Vec2) -> T
    {
        let c : Vec3 = pos.into();
        let point = vec2(c.x / screen_size.x * 2. - 1., 1. - c.y / screen_size.y * 2.,);
        //let point = c;
        let transform = self.inv_matrix.transform_point3(vec3(point.x, point.y, 0.).to_glam()).to_engine();
        transform.into()
    }

    pub(crate) fn apply(&mut self)
    {
        self.inv_matrix = self.matrix.inverse();
        macroquad::prelude::set_camera(self);
    }

}
/* 
impl Default for Camera
{
    fn default() -> Self { Self::new_2d() }
}*/

impl LibCamera for Camera
{
    fn matrix(&self) -> LibMat4 { self.matrix }

    fn depth_enabled(&self) -> bool { self.have_depth }

    fn render_pass(&self) -> Option<LibRenderPass> { None }
    fn viewport(&self) -> Option<LibViewport> 
    { 
        let begin = self.viewport.pos.to_int();
        let end = self.viewport.top_right().to_int();

        Some
        (
            (
                begin.x as LibViewportComposite, 
                begin.y as LibViewportComposite, 
                (end.x - begin.x) as LibViewportComposite, 
                (end.y - begin.y) as LibViewportComposite, 
            )
        )
        /* 
        Some
        (
            (
                self.viewport.left_value() as LibViewportComposite, 
                self.viewport.bot_value() as LibViewportComposite, 
                self.viewport.width() as LibViewportComposite, 
                self.viewport.height() as LibViewportComposite
            )
        )
        */
    }
}