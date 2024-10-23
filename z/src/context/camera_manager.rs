use super::*;

/// The camera by default is centered on the middle on the screen (0, 0)
/// A square starting from (-1, -1) to (1, 1) is always visible on screen, and not stretched
#[derive(Debug)]
pub struct CameraManager
{
    camera : LastStack<Camera>,
    window : Vec2,



    // Smallest axis is 1. ?
    //window_normalized :
}

impl Default for CameraManager
{
    fn default() -> Self 
    {
        let camera = LastStack::new(Camera::new_2d(Rect2f::new(one(), zero())));
        //camera.push();
        Self 
        {
            window: Vec2::ONE,
            camera,
        }
    }
}

impl CameraManager
{
    pub fn window(&self) -> Rect2f { Rect2f::new(self.window_pos(), self.window_size()) }
    pub fn window_pos(&self) -> Vec2 { zero() }
    pub fn window_size(&self) -> Vec2 { self.window }
}

impl CameraManager
{
    pub(crate) fn end_frame(&mut self) -> &mut Self
    {
        assert_eq!(self.camera.len(), 1, "Forget to pop camera");
        //self.reset_current_cam();
        //self.camera.keep_only_first();
        self
    }
    
    pub(crate) fn begin_frame(&mut self) -> &mut Self
    {
        assert_eq!(self.camera.len(), 1, "Forget to pop camera");

        self.window = vec2(macroquad::prelude::screen_width() as float, macroquad::prelude::screen_height() as float);
        //self.move_by(self.window * 0.5).zoom_without_apply(Vec2::ONE.x_ry());
        self.camera.keep_only_first();
        self.reset()
    }

    pub(crate) fn lib_to_engine(&self, lib_pos : Vec2) -> Vec2 { self.window_size() * 0.5 - lib_pos }
    pub(crate) fn lib_to_world(&self, pos : Vec2) -> Vec2  { self.px_to_world(self.lib_to_engine(pos)) }
    pub(crate) fn px_to_world(&self, pos : Vec2) -> Vec2  { self.last().screen_to_world(pos, self.window_size()) }
}

impl Moveable<float> for CameraManager
{
    fn move_by(&mut self, v : impl Into<C4<float>>) -> &mut Self { self.move_by_without_apply(v).apply() }
}

impl CameraManager
{
    pub fn last(&self) -> &Camera { self.camera.last() }
    pub fn last_mut(&mut self) -> &mut Camera { self.camera.last_mut() }

    /* 
    pub fn set_default_camera(&mut self, cam : Camera) -> &mut Self 
    {
        self.camera[0] = cam;
        self
    }*/

    /// Add a checkpoint to the current view matrix.
    /// When pen.pop() is call, return to the last checkpoint
    pub fn push(&mut self) -> &mut Self { self.push_without_apply().apply() }
    pub fn push_without_apply(&mut self) -> &mut Self { self.camera.push(); self }

    /// Apply the current matrix
    pub fn apply(&mut self) -> &mut Self { self.last_mut().apply(); self }
    fn multiply_mat_without_apply(&mut self, mat : LibMat4) -> &mut Self { self.last_mut().matrix *= mat; self }

    pub fn move_x_without_apply(&mut self, v : float) -> &mut Self { self.move_by_without_apply(Vec4::X * v) }
    pub fn move_y_without_apply(&mut self, v : float) -> &mut Self { self.move_by_without_apply(Vec4::Y * v) }
    pub fn move_z_without_apply(&mut self, v : float) -> &mut Self { self.move_by_without_apply(Vec4::Z * v) }
    pub fn move_w_without_apply(&mut self, v : float) -> &mut Self { self.move_by_without_apply(Vec4::W * v) }
    pub fn move_by_without_apply(&mut self, v : impl Into<C4<float>>) -> &mut Self 
    { 
        let v : Vec4 = v.into();
        self.multiply_mat_without_apply(LibMat4::from_translation(v.to_c3().to_glam()))
    }

    pub fn zoom(&mut self, v : impl CoordinateScalar) -> &mut Self { self.zoom_without_apply(v).apply() }
    pub fn zoom_without_apply(&mut self, v : impl CoordinateScalar) -> &mut Self 
    {
        let v : Vec3 = v.to_c3_with_one().to_vec3();
        self.multiply_mat_without_apply(LibMat4::from_scale(v.to_glam()))
    }

    pub fn viewport(&self) -> Rect2f { self.camera.viewport }
    pub fn viewport_size(&self) -> Vec2 { self.camera.viewport.size }
    pub fn viewport_pos(&self) -> Vec2 { self.camera.viewport.pos }

    pub fn set_viewport(&mut self, viewport : Rect2f) -> &mut Self { self.set_viewport_without_apply(viewport).apply() }
    pub fn set_viewport_without_apply(&mut self, viewport : Rect2f) -> &mut Self
    {
        self.last_mut().viewport = viewport;
        self
    }

    pub fn rot_x(&mut self, angle : Angle) -> &mut Self { self.rot_x_without_apply(angle).apply() }
    pub fn rot_x_without_apply(&mut self, angle : Angle) -> &mut Self { self.multiply_mat_without_apply(LibMat4::from_rotation_x(angle.radian() as f32)) }
    
    pub fn rot_y(&mut self, angle : Angle) -> &mut Self { self.rot_y_without_apply(angle).apply() }
    pub fn rot_y_without_apply(&mut self, angle : Angle) -> &mut Self { self.multiply_mat_without_apply(LibMat4::from_rotation_y(angle.radian() as f32)) }
    
    pub fn rot_z(&mut self, angle : Angle) -> &mut Self { self.rot_z_without_apply(angle).apply() }
    pub fn rot_z_without_apply(&mut self, angle : Angle) -> &mut Self { self.multiply_mat_without_apply(LibMat4::from_rotation_z(angle.radian() as f32)) }
    
    pub fn pop(&mut self) -> &mut Self { self.pop_without_apply().apply() }
    pub fn pop_without_apply(&mut self) -> &mut Self { self.camera.pop(); self }


    /// Reset the current camera to target the whole window
    pub fn reset(&mut self) -> &mut Self { self.reset_without_apply().apply() }

    /// Reset the current camera to target the whole window
    pub fn reset_without_apply(&mut self) -> &mut Self
    {
        self.reset_matrix_without_apply();
        self.camera.viewport = Rect2f::new_sized(self.window_size());
        self
    }

    pub fn reset_matrix(&mut self) -> &mut Self { self.reset_matrix_without_apply().apply() }
    pub fn reset_matrix_for_rect(&mut self, rect : Rect2f) -> &mut Self { self.reset_matrix_for_rect_without_apply(rect).apply() }

    /// Reset the current camera to target the whole window
    pub fn reset_matrix_without_apply(&mut self) -> &mut Self { self.reset_matrix_for_rect(self.window())}
    pub fn reset_matrix_for_rect_without_apply(&mut self, rect : Rect2f) -> &mut Self
    {
        self.camera.matrix = LibMat4::IDENTITY;
        
        // focus a centered square
        // max to avoid dividing by 0 and having NaN value
        // self.zoom_without_apply(z.y_x() / z.max_element().max(1.0) * 2.0);

        self.zoom_without_apply(1.0.splat2() / rect.size * 2.0);
        self.move_by_without_apply(rect.size * -0.5)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FocusResult
{
    /// wanted is always inside obtained
    pub wanted   : Rect2f,
    pub obtained : Rect2f,
}
impl FocusResult
{
    pub fn new(wanted : Rect2f, obtained : Rect2f) -> Self { Self { wanted, obtained }}

    pub fn have_bonus_space(&self) -> bool { self.wanted != self.obtained }

    pub fn have_bonus_space_x(&self) -> bool { self.obtained.size.x > self.wanted.size.x }
    pub fn bonus_left(&self) -> Rect2f { self.obtained.with_size_x(self.wanted.pos.x - self.obtained.pos.x) }

    pub fn bonus_right(&self) -> Rect2f
    {
        self.obtained.with_size_x(self.obtained.right_value() - self.wanted.right_value()).with_pos(self.wanted.bottom_right())
    }

    
    pub fn have_bonus_space_y(&self) -> bool { self.obtained.size.y > self.wanted.size.y }
    pub fn bonus_bot(&self) -> Rect2f { self.obtained.with_size_y(self.wanted.pos.y - self.obtained.pos.y) }

    pub fn bonus_top(&self) -> Rect2f
    {
        self.obtained.with_size_y(self.obtained.top_value() - self.wanted.top_value()).with_pos_y(self.wanted.top_value())
    }
}

impl CameraManager
{
    pub fn focus(&mut self, wanted : Rect2f) -> FocusResult { let r = self.focus_ex_without_apply(wanted, half()); self.apply(); r }
    pub fn focus_ex_without_apply(&mut self, wanted : Rect2f, center : Vec2) -> FocusResult 
    { 
        let (size, pos) = (wanted.size, wanted.pos);
        self.reset_matrix();
        
        let viewport = self.viewport();

        let zoom_dif = viewport.size / size;
        let zoom = zoom_dif.min_element().splat2();

        let obtained_size = viewport.size / zoom;
        let size_dif  = obtained_size - wanted.size;
        let size_offset = -size_dif * center;
        let obtained_pos  = wanted.pos + size_offset;

        self.zoom_without_apply(zoom * (self.window_size() / viewport.size)).move_by_without_apply(-(pos + size_offset));
        FocusResult::new(wanted, Rect2f::new(obtained_pos, obtained_size))
    }
}