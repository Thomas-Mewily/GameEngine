use super::*;

#[derive(Debug)]
pub struct Context
{
    pub assets  : AssetsManager,
    pub pen     : Pen,
    pub input   : InputManager,
    pub perf    : Perf,
}
impl Context
{
    pub fn new() -> Self 
    { 
        Self 
        {
            assets : AssetsManager::const_default(),
            pen  : Pen::___(),
            input: InputManager::___(),
            perf : Perf::const_default(),
        }
    }
}

impl Default for Context
{
    fn default() -> Self { Self::new() } 
}

impl Context
{
    pub fn init(&mut self) { self.begin_frame() }

    pub async fn next_frame(&mut self) 
    { 
        self.end_frame();
        macroquad::prelude::next_frame().await;
        self.begin_frame();
    }
    pub fn calculate_delta_time(&mut self) -> DeltaTime 
    { 
        let dt = DeltaTime::from_s(macroquad::prelude::get_frame_time() as float);
        debug_assert!(dt.is_positive());
        dt
    }

    pub fn debug<E : std::fmt::Debug>(&mut self, e : &E) { self.log(format!("{:?}", e)) }
    pub fn log(&mut self, txt : String) { self.pen.debug.push(txt); }
}

impl Context
{
    pub(crate) fn end_frame(&mut self)
    {
        self.pen.end_frame();
        self.input.end_frame();
    }

    pub(crate) fn begin_frame(&mut self)
    {
        let dt = self.calculate_delta_time();
        self.perf.begin_frame(dt);
        self.pen.begin_frame();
        self.pen.cam.begin_frame();
        self.input.begin_frame();
    }
}

/// Singleton for the context. I know singleton are bad, but here it is necessary.
/// 
/// Everythings in the context can't exist twice
pub fn ctx() -> &'static mut Context 
{
    // Declare a mutable static reference (unsafe)
    static mut INSTANCE: *mut Context = std::ptr::null_mut();
    static ONCE: std::sync::Once = std::sync::Once::new();

    // Initialize the singleton (only once)
    ONCE.call_once(|| {
        // Allocate the singleton instance
        let singleton = Box::new(Context::new());
        // Leak the Box to make it live forever (leaks the memory to static mut pointer)
        unsafe {
            INSTANCE = Box::into_raw(singleton);
        }
    });

    // Return a mutable reference to the singleton (unsafe)
    unsafe {
        &mut *INSTANCE
    }
}

pub fn pen() -> &'static mut Pen { &mut ctx().pen }
pub fn cam() -> &'static mut CameraManager { &mut ctx().pen.cam }
pub fn perf() -> &'static mut Perf { &mut ctx().perf }
pub fn input() -> &'static mut InputManager { &mut ctx().input }

/* 
impl ILoopManager for Context
{
    type Accumulator = FrontTime;
    fn init() -> FrontTime { ctx().init(); ___() }
    
    async fn wait_next_frame(acc : &mut Self::Accumulator) 
    { 
        ctx().next_frame().await;
        acc.update(ctx().calculate_delta_time());
    }

    /* 
    fn debug_key_pressed(&self) -> bool {
        self.input.key(KeyCode::Space).is_just_pressed()
    }*/
}*/

/* 
pub type NbFrame = u64;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct FrontTime
{
    pub total    : Time,
    pub dt       : DeltaTime,
    pub nb_frame : NbFrame,
}

impl FrontTime
{
    pub fn new(total : Time, dt : DeltaTime, nb_frame : Tick, ) -> Self { Self { total, dt, nb_frame }}

    pub fn with_total(mut self, total : Time) -> Self { self.total = total; self }
    pub fn with_delta_time(mut self, dt : DeltaTime) -> Self { self.dt = dt; self }
    pub fn with_nb_frame(mut self, nb_frame : NbFrame) -> Self { self.nb_frame = nb_frame; self }

    pub fn update(&mut self, dt : DeltaTime)
    {
        self.dt = dt;
        self.total += dt;
        self.nb_frame += 1;
    }
}
impl Display for FrontTime
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult {
        write!(f, "front time {} (# {}, dt {})", self.total, self.nb_frame, self.dt)
    }
}*/