#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use platformer::*;

//use math::*;
use derive_more::{Deref,DerefMut,Index,IndexMut};
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
    ctx().init();
    pen().set_window_background_color(Color::GREEN);
    
    let mut g = GameGraphique::test_level().await;

    g.run_at_60_ups().await;
}
