use super::*;

#[derive(Clone, Deref, DerefMut)]
pub struct GameGraphique
{
    #[deref] #[deref_mut]
    game : Game,
}

impl GameGraphique
{
    pub fn new() -> Self { Self { game: Game::new() }}

    pub fn test_level() -> Self
    {
        let mut s = Self::new();

        //g.camera = rect2f(-4., -5., 8., 10.);
        s.add(Entity::new(Kind::Banana, Physic::___()));
        //g.add(Entity::new(Kind::Banana, Physic::___().with_pos(vec2(3., 5.))));
        s
    }

    pub fn draw(&mut self)
    {
        let pen = pen();
        let cam = cam();

        let r = Rect2f::new(-1.0.splat2(), 3.0.splat2());
        
        for viewport in cam.viewport().split_x(3)
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
        }
    }
}