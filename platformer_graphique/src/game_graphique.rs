use super::*;

#[derive(Clone)]
pub struct Assets
{
    gif : Animation,
    sprite : Texture2D,
}
impl Assets
{
    pub async fn load() -> Self
    {
        Self
        { 
            gif: Animation::from_file(r"./platformer_graphique\assets\test\player_walk.gif").await.unwrap(), 
            sprite: Texture2D::from_file(r"./platformer_graphique\assets\test\voyageur.png").await.unwrap(), 
        }
    }
}

#[derive(Clone, Deref, DerefMut)]
pub struct GameGraphique
{
    #[deref] #[deref_mut]
    game : Game,
    assets : Assets,
}

impl GameGraphique
{
    pub async fn new() -> Self 
    { 
        let assets = Assets::load().await;
        Self 
        { 
            game: Game::new(), 
            assets, 
        }
    }

    pub async fn test_level() -> Self
    {
        let mut s = Self::new().await;

        //g.camera = rect2f(-4., -5., 8., 10.);
        s.add(Entity::new(Kind::Banana, Physic::___()));
        //g.add(Entity::new(Kind::Banana, Physic::___().with_pos(vec2(3., 5.))));
        s
    }

    pub fn on_draw(&mut self, dt : DeltaTime)
    {
        let pen = pen();
        let cam = cam();

        let r = Rect2f::new(-1.0.splat2(), 3.0.splat2());
        
        self.assets.gif.update(dt);

        for viewport in cam.viewport().split_x(2)
        {
            cam.set_viewport(viewport);
            for viewport in cam.viewport().split_y(2)
            {
                //cam.rot_z(15.degree());
                cam.set_viewport(viewport);

                let focus = cam.focus(r);
                cam.rot_z(15.degree());
                
                pen.rect(r, Color::YELLOW);
                pen.rectangle(zero(), one(), Color::WHITE);

                pen.rect(focus.bonus_left(), Color::RED);
                pen.rect(focus.bonus_right(), Color::BLUE);
                pen.rect(focus.bonus_top(),  Color::GREEN.lerp(Color::BLACK, 0.5));
                pen.rect(focus.bonus_bot(),  Color::PINK);

                pen.texture(self.assets.gif.frame(), zero(), one(), zero(), 
                        DrawTexture2D::___().with_flip_x(false)
                    );
            }
        }
    }
}

impl Resetable for GameGraphique{
    fn reset(&mut self) {
        
    }
}

impl Updateable for GameGraphique { fn update(&mut self, dt : DeltaTime) { self.game.update(dt); } }
impl Drawable for GameGraphique   { fn draw(&mut self, dt : DeltaTime) { self.on_draw(dt); } }
