use super::*;

use macroquad::{prelude, prelude::Touch as MTouch};
pub type KeyCode = prelude::KeyCode;

#[derive(Clone, PartialEq)]
pub struct InputManager
{
    /// Also include the mouse
    touch : Vec<Touch>,
    mouse : Touch,
    keyboard : HashMap<KeyCode, InputBool>,
}
impl Default for InputManager
{
    fn default() -> Self 
    {
        let mut mouse = Touch::___();
        mouse.kind = TouchKind::Mouse;
        Self { touch: ___(), mouse, keyboard: ___() }
    }
}

impl Debug for InputManager
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "input {{ {} nb touch, {} just pressed, {} just released }}", self.touch.len(), self.iter_touch_just_pressed().count(), self.iter_touch_just_released().count())
    }
}

pub type MouseButton = prelude::MouseButton;

/* 
impl ContextInput
{
    pub fn is_mouse_button_pressed(&self, button : MouseButton) -> bool 
    { macroquad::prelude::is_mouse_button_pressed(button) }

    pub fn is_mouse_pressed(&self) -> bool 
    { self.is_mouse_button_pressed(prelude::MouseButton::Left) }

    pub fn mouse_pos_px(&self) -> Vec2 
    { 
        let (x, y) = macroquad::prelude::mouse_position();
        Vec2::new(x, y)
    }

    pub fn mouse_pos(&self, _pen : &Pen) -> Vec2 
    { 
        let (x, y) = macroquad::prelude::mouse_position();
        Vec2::new(x, y);
        todo!("convert it");
    }
}
*/

pub trait GetInputBoolExtension
{
    fn is_pressed(&self) -> bool;
    fn was_pressed(&self) -> bool;

    /// `false` to `true`, `0` to `1`
    fn is_pull_up(&self) -> bool { self.is_pressed() && (!self.was_pressed()) } 
    /// `true` to `false`, `1` to `0`
    fn is_pull_down(&self) -> bool { self.was_pressed() && (!self.is_pressed()) } 

    fn is_pull_changed(&self) -> bool { self.is_pressed() != self.was_pressed() }


    fn is_released(&self) -> bool { !self.is_pressed() } 
    fn was_released(&self) -> bool { !self.was_pressed() } 

    fn is_just_pressed(&self) -> bool { self.is_pull_up() } 
    fn is_just_released(&self) -> bool { self.is_pull_down() } 
}

impl GetInputBoolExtension for KeyCode
{
    fn is_pressed(&self) -> bool { input().key(*self).is_pressed() }
    fn was_pressed(&self) -> bool { input().key(*self).was_pressed() }
}

impl InputManager
{
    pub fn key(&self, key : KeyCode) -> InputBool { self.keyboard.get(&key).copied().unwrap_or_default() }
    
    pub fn iter_key_just_pressed(&self) -> impl Iterator<Item=&KeyCode> { self.keyboard.iter().filter_map(|(k, a)| if a.is_just_pressed() { Some(k) } else { None }) }
    pub fn iter_key_just_released(&self) -> impl Iterator<Item=&KeyCode> { self.keyboard.iter().filter_map(|(k, a)| if a.is_just_released() { Some(k) } else { None }) }
    pub fn iter_key_pressed(&self) -> impl Iterator<Item=&KeyCode> { self.keyboard.iter().filter_map(|(k, a)| if a.is_pressed() { Some(k) } else { None }) }

    pub fn mouse(&self) -> &Touch { &self.mouse }

    pub fn iter_touch(&self) -> impl Iterator<Item=&Touch> { self.touch.iter() }
    pub fn iter_touch_just_pressed(&self) -> impl Iterator<Item=&Touch> { self.touch.iter().filter(|t| t.press().is_just_pressed()) }
    pub fn iter_touch_just_released(&self) -> impl Iterator<Item=&Touch> { self.touch.iter().filter(|t| t.press().is_just_released()) }
    pub fn iter_touch_pressed(&self) -> impl Iterator<Item=&Touch> { self.touch.iter().filter(|t| t.press().is_pressed()) }
    pub fn iter_touch_released(&self) -> impl Iterator<Item=&Touch> { self.touch.iter().filter(|t| t.press().is_released()) }
    
    pub(crate) fn mq_mouse_pressed() -> bool { macroquad::prelude::is_mouse_button_down(macroquad::input::MouseButton::Left) }
    pub(crate) fn mq_mouse_pos(cam : &CameraManager) -> Vec2 { let (mx, my) = macroquad::prelude::mouse_position(); cam.lib_to_engine(vec2(mx as float, my as float)) }

    pub(crate) const MAX_TOUCH_SCREEN_TOUCH : usize = 8;
}


impl InputManager
{
    pub(crate) fn begin_frame(&mut self) 
    {
        self.update_mouse_and_touch();
        self.update_keyboard();
    }

    pub(crate) fn update_keyboard(&mut self) 
    {
        let mq_pressed = macroquad::prelude::get_keys_down();
        for pressed in mq_pressed.iter().copied()
        {
            match self.keyboard.get_mut(&pressed)
            {
                Some(v) => v.update(true, ()),
                None => { self.keyboard.insert(pressed, ___()); },
            }
        }
        
        for (k, a) in self.keyboard.iter_mut() 
        { if !mq_pressed.contains(k) { a.update(false, ()) }}
    }

    pub(crate) fn update_mouse_and_touch(&mut self) 
    {
        let cam = cam();
        self.touch.retain(|e| e.press().is_pressed() && e.kind != TouchKind::Mouse);

        let mut touch = prelude::touches();
        touch.truncate(Self::MAX_TOUCH_SCREEN_TOUCH);
        
        for t in touch.iter().cloned()
        {
            let pos = cam.lib_to_engine(t.position.to_engine());

            match self.touch.iter().position(|p| *p.id == t.id)
            {
                Some(old_touch) => 
                {
                    let old = &mut self.touch[old_touch];
                    old.update(pos, match t.phase
                        {
                            macroquad::input::TouchPhase::Started => true,
                            macroquad::input::TouchPhase::Stationary => true,
                            macroquad::input::TouchPhase::Moved => true,
                            macroquad::input::TouchPhase::Ended => false,
                            macroquad::input::TouchPhase::Cancelled => false,
                        });
                },
                _ => 
                {
                    // Should not happen because it should be added first, but I prefer to avoid panicking
                },
            }
        }

        for t in self.touch.iter_mut()
        {
            if touch.iter().position(|e| t.id.0 == e.id).is_none()
            {
                // will be removed
                t.press.set_pressed(false);
            }
        }

        self.mouse.update(Self::mq_mouse_pos(cam), Self::mq_mouse_pressed());
        self.touch.push(self.mouse);
    }



    pub(crate) fn end_frame(&mut self) 
    {
        cam().reset();
        let window_size = cam().window_size();

        let pen = pen();

        for t in self.iter_touch()
        {
            if t.press().is_pressed()
            {
                let pos = t.position().cur;
                let pos_pressed = t.pressed_position();
                
                let delta_len = t.delta_from_press().length();

                let coef = delta_len / window_size.length();

                let scale = 0.5 * window_size.min_element();

                let s = 1.0.lerp(40. as float, coef.powf(1./1.25)).percent();
                let s_origin = 4.0.lerp(0.25 as float, coef.powf(1./2.)).percent();

                let greyscale_effect = (coef*5.).min(1.);
                pen.circle(vec2(pos.x, pos.y), s * scale, Color::new_greyscale(1. - greyscale_effect));
                pen.circle(vec2(pos_pressed.x, pos_pressed.y), s_origin * scale, Color::new_greyscale(greyscale_effect));
                
            }
        }
    }
}

/* 
impl<Glob> ContextEvent<Glob> for ContextInput
{
    fn tick_begin(ctx : &mut DefaultContext<Glob>)
    {
        ContextInput::_tick_begin(ctx);
    }

    fn input_begin (ctx : &mut DefaultContext<Glob>) 
    {
        ContextInput::_input_begin(ctx);
    }

    fn draw_end(ctx : &mut DefaultContext<Glob>)
    {
        let win_len = ctx.pen.cam.window_size_px().length();

        for t in &ctx.input.touch
        {
            if t.press().is_pressed()
            {
                let delta_len = t.delta_from_press_px().length();

                let coef = delta_len / win_len;

                let s_coef = 1.0.lerp(30. as float, coef.powf(1./1.25)) / 100.;
                let s = s_coef * ctx.pen.cam.window_size_px().min_element();
                //let s_origin = coef.lerp(4. as float, 0.25 as float) / 100. * ctx.pen.cam.window_size().min_element();
                let s_origin = 4.0.lerp(0.25 as float, coef.powf(1./2.)) / 100. * ctx.pen.cam.window_size_px().min_element();

                let greyscale_effect = (coef*5.).min(1.);
                ctx.pen.circle((t.position_px.x, t.position_px.y), s/2., Color::new_greyscale(1. - greyscale_effect));
                ctx.pen.circle((t.pressed_position_px.x, t.pressed_position_px.y), s_origin/2., Color::new_greyscale(greyscale_effect));
            }
        }
    }
}
*/



#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TouchId(u64);
impl Deref for TouchId { type Target=u64; fn deref(&self) -> &Self::Target { &self.0 } }
impl DerefMut for TouchId { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }
impl TouchId
{
    pub const MOUSE_ID : TouchId = TouchId(u64::MAX-17*9);
}
impl Debug for TouchId { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Touch#{}", self.0) }}

#[derive(Default, Clone, Copy, PartialEq, Debug, Deref)]
pub struct Touch
{
    id : TouchId,
    
    position_px : InputVec2,

    press : InputBool,
    
    /// The position when the touch was just pressed. By default the same as the first position
    pressed_position_px : Vec2,
    /// The position when the touch was just released. By default the same as the first position
    released_position_px : Vec2,

    #[deref]
    kind  : TouchKind,
}

impl Touch
{
    pub fn with_kind(mut self, kind  : TouchKind) -> Self { self.kind = kind; self }

    pub fn id(&self) -> TouchId { self.id }

    /// The windows position in pixel 
    pub fn position_px(&self) -> InputVec2 { self.position_px }
    /// world position
    pub fn position(&self) -> InputVec2 { InputVec2 { cur: cam().lib_to_world(self.position_px.cur()), old: cam().lib_to_world(self.position_px.old()), time:() } }

    pub fn pressed_position_px(&self) -> Vec2 { self.pressed_position_px }
    pub fn pressed_position(&self) -> Vec2 { cam().lib_to_world(self.pressed_position_px) }

    pub fn released_position_px(&self) -> Vec2 { self.released_position_px }
    pub fn released_position(&self) -> Vec2 { cam().lib_to_world(self.released_position_px) }

    pub fn delta_from_press_px(&self) -> Vec2 { self.position_px().cur() - self.pressed_position_px }
    /// world position
    pub fn delta_from_press(&self) -> Vec2 { self.position().cur() - self.pressed_position() }

    pub fn press(&self) -> InputBool { self.press }
    pub fn kind(&self) -> TouchKind { self.kind }

    pub fn update(&mut self, position_px : Vec2, is_press : bool)
    {
        self.position_px.update(position_px, ());
        self.press.update(is_press, ());

        if self.press.is_just_pressed() { self.pressed_position_px = position_px; }
        if self.press.is_just_released() { self.released_position_px = position_px; }
    }
}


#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TouchKind
{
    Mobile,
    Mouse,
    #[default]
    Unknown,
}

impl TouchKind
{
    pub fn is_mouse (&self) -> bool { matches!(self, TouchKind::Mouse) }
    pub fn is_mobile(&self) -> bool { matches!(self, TouchKind::Mobile) }
}

