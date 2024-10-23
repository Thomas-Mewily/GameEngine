use super::*;

#[derive(Debug)]
pub struct Perf 
{
    time : Time,

    last_whole_fps : usize,
    fps_count : usize,
}

impl Default for Perf
{
    fn default() -> Self {
        Self::const_default()
    }
}

impl Perf
{
    pub(crate) const fn const_default() -> Self
    {
        Self { time: Time::ZERO, last_whole_fps: 0, fps_count: 0 }
    }
}

impl Perf
{
    pub fn fps(&self) -> usize { self.last_whole_fps }

    pub fn begin_frame(&mut self, dt : Time)
    {
        self.fps_count += 1;
        self.time += dt;

        let one_s = 1.s();

        if self.time >= one_s
        {
            self.last_whole_fps = self.fps_count;
            self.fps_count = 0;
            self.time -= one_s;

            //info!("fps : {}", self.fps());
        }
    }
}