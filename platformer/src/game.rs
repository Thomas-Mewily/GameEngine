use super::*;

pub type EntityID = usize; //GenVecID<Entity>;


#[derive(Default, Debug, Clone)]
pub struct Game
{
    tick : Tick,
    entities : Vec<Entity>,
    //update_stack : Vec<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind
{
    Ground,
    Banana,
    Player,
    Camera,
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Entity
{
    kind   : Kind,
    #[deref] #[deref_mut]
    physic : Physic,
    last_tick_update : Tick,
}

impl Entity
{
    pub fn new(kind : Kind, physic : Physic) -> Self { Self { physic, kind, last_tick_update: 0 }}
}

#[derive(Clone, Debug)]
pub struct Physic
{
    //speed  : Vec2,
    pos    : Vec2,
    hitbox : Vec2,
    is_static : bool,
}

impl Default for Physic
{
    fn default() -> Self {
        Self { pos: ___(), hitbox: one(), is_static: true }
    }
}

impl Physic
{
    pub fn with_pos(mut self, pos : Vec2) -> Self { self.pos = pos; self }
    pub fn with_hitbox(mut self, hitbox : Vec2) -> Self { self.hitbox = hitbox; self }
    pub fn with_static(mut self, is_static : bool) -> Self { self.is_static = is_static; self }
}

pub type Tick = u64;

impl Game
{
    pub fn new() -> Game { ___() }

    pub fn add(&mut self, e : Entity) -> EntityID { self.entities.push(e); self.entities.len() - 1 }
    
    pub fn iter_entity_id(&self) -> impl Iterator<Item = EntityID> { 0..self.entities.len() }
    pub fn update(&mut self)
    {
        for e in self.iter_entity_id()
        {
            self.update_idx(e);
        }
    }


    pub fn update_idx(&mut self, idx : usize)
    {
        if self[idx].last_tick_update == self.tick { return; }
        self[idx].last_tick_update = self.tick;

        
    }

    
    //pub fn get(&self, id : EntityID) -> Option<&Entity> { self.entities.get(id) }
    //pub fn get_mut(&mut self, id : EntityID) -> Option<&mut Entity> { self.entities.get_mut(id) }
}
impl Index<EntityID> for Game
{
    type Output=Entity;
    fn index(&self, index: EntityID) -> &Self::Output { &self.entities[index] }
}
impl IndexMut<EntityID> for Game
{
    fn index_mut(&mut self, index: EntityID) -> &mut Self::Output { &mut self.entities[index] }
}
/* 
impl Deref for Game
{
    type Target=Entity;
    fn deref(&self) -> &Self::Target { self.entities.get_idx(self.update_stack.last().copied().unwrap()).unwrap() }
}

impl DerefMut for Game
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.entities.get_idx_mut(self.update_stack.last().copied().unwrap()).unwrap() }
}*/