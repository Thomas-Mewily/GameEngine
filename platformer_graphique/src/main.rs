#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use platformer::*;

//use math::*;
use derive_more::{Deref,DerefMut};
//use util::*;
use engine::*;

pub mod game_graphique;
pub use game_graphique::*;

fn window_conf() -> Config 
{
    Config 
    {
        window_title: "platformer_graphique 1.0".to_owned(),
        fullscreen: false,
        high_dpi: true,
        window_resizable : true,
        icon : Some(generate_icon!("../assets/icon/icon")),
        ..___()
    }
}

#[engine::main(window_conf)]
async fn main() 
{
    /* 
    println!("hello");
    debug!("hello2");

    for i in 3.iter_coef_start_at_zero()
    {
        info!("{}", i);
    }
    infoln!();
    for i in 3.iter_coef_finish_at_one()
    {
        info!("{}", i);
    }
    infoln!();
    for i in 3.iter_coef()
    {
        info!("{}", i);
    }
    infoln!();*/




    pen().set_window_background_color(Color::GREEN);

    let mut g = GameGraphique::test_level();

    let mut old_time = Time::ZERO;
    let mut time = old_time;
    
    let fixed_step = (1.0 / 60.).s();

    ctx().init();

    loop
    {
        time += ctx().calculate_delta_time();

        while time - old_time >= fixed_step
        {
            old_time += fixed_step;
            g.update();
        }
        g.draw();
        
        ctx().next_frame().await;
    }
}
