use crate::*;

/* 
pub struct AnimationSequence
{
    pub frames : Vec<AnimationFrame<F>>,
}*/

#[derive(Default, Debug, Deref, DerefMut, Clone, Indexable, IndexableMut)]
pub struct Animation<F=Texture2D>
{
    #[index] #[index_mut]
    pub frames : Vec<AnimationFrame<F>>,
    #[deref] #[deref_mut]
    pub params : AnimationParams,
}

#[derive(Debug, Clone, Copy)]
pub struct AnimationParams
{
    current_frame : usize,
    elapsed_time  : Time,
    time_scale    : float,
    repeat        : bool,
    paused        : bool,
}

impl Default for AnimationParams
{
    fn default() -> Self {
        Self { current_frame: 0, elapsed_time: zero(), time_scale: 0., repeat: true, paused: false }
    }
}

impl TextureParam for Animation
{
    fn set_aa(&mut self, aa : bool) -> &mut Self {
        for t in self.frames.iter_mut()
        {
            t.set_aa(aa);
        }
        self
    }
}

impl<F> Animation<F>
{
    pub fn new(frames: Vec<AnimationFrame<F>>, params : AnimationParams) -> Self {
        Self { frames, params }
    }

    pub fn add_frame(&mut self, frame : AnimationFrame<F>) -> &mut Self { self.frames.push(frame); self }

    pub fn frame(&self) -> &AnimationFrame<F> { &self[self.current_frame] }
    pub fn frame_mut(&mut self) -> &mut AnimationFrame<F> { let idx = self.current_frame; &mut self[idx] }

    fn is_last_frame(&self) -> bool { self.current_frame + 1 >= self.frames.len() }

    fn advance_frame(&mut self) 
    {
        self.elapsed_time = zero();
        if !self.is_last_frame()
        {
            self.current_frame += 1;
            return;
        }
        if self.repeat
        {
            self.current_frame = 0;
        }
    }
}

impl<F> Updateable for Animation<F>
{
    fn update(&mut self, dt : DeltaTime) {
        if !self.paused
        {
            self.elapsed_time += dt;
            if self.elapsed_time > self.frame().duration 
            {
                let remaning = self.elapsed_time - self.frame().duration;
                self.advance_frame();
                self.elapsed_time = remaning;
            }
        }
    }
}

impl<F> AnimationFrame<F>
{
    pub fn new(frame : F, duration : Time) -> Self { Self { frame, duration } }
}


impl ImportFromRaw for Animation<Texture2D>
{
    type ImportRawError=();

    fn from_raw(raw : &[u8]) -> Result<Self, Self::ImportRawError> 
    {
        // mainly based on https://crates.io/crates/quad-gif
        //use macroquad::prelude::*;
        use rgb::ComponentBytes;

        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::Indexed);
        let mut decoder = options.read_info(raw).map_err(|_e| ())?;
        let mut screen = gif_dispose::Screen::new_decoder(&decoder);
        
        let mut frames: Vec<AnimationFrame> = Vec::new();
        while let Some(frame) = decoder.read_next_frame().unwrap() 
        {
            screen.blit_frame(&frame).map_err(|_e| ())?;
            let (pixels, frame_width, frame_height) = screen.pixels.as_contiguous_buf();

            let frame = AnimationFrame::new(Texture2D{ val: LibTexture2D::from_rgba8(
                frame_width as u16,
                frame_height as u16,
                pixels.as_bytes(),
            ) }, (frame.delay as float / 100.).s());

            frames.push(frame);
        }
        Ok(Self::new(frames, ___()))
    }
}

#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct AnimationFrame<F=Texture2D>
{
    #[deref] #[deref_mut]
    frame     : F,
    duration  : Time,
}