use super::*;

pub trait Updateable
{
    fn update(&mut self, dt : DeltaTime);
}

pub trait Drawable
{
    fn draw(&mut self, dt : DeltaTime);
}

pub trait Resetable
{
    fn reset(&mut self);
}


#[allow(async_fn_in_trait)]
pub trait Runnable : Updateable + Drawable + Resetable
{
    async fn run_at_60_ups(&mut self) { self.run_at_fixed_frequency((1. / 60.).s()).await }
    async fn run_at_fixed_frequency(&mut self, dt_update : Time)
    {
        let mut old_time = Time::ZERO;
        let mut time = old_time;

        loop
        {
            let dt_draw = ctx().calculate_delta_time();
            time += dt_draw;
    
            while time - old_time >= dt_update
            {
                old_time += dt_update;
                self.update(dt_update);
            }
            self.draw(dt_draw);
            
            ctx().next_frame().await;
        }
    }
}
impl<T> Runnable for T where T : Updateable + Drawable + Resetable {}