use super::*;

pub type GenerationIdx = u32;

/* 
pub struct SmartPhantomData<T>
{
    phantom : PhantomData<T>
}
impl<T> SmartPhantomData<T>
{
    const VALUE : Self = Self { phantom : PhantomData };
    pub const fn value() -> Self { Self::VALUE }
}
impl<T> Default for SmartPhantomData<T> { fn default() -> Self { Self::VALUE } }
impl<T> Clone for SmartPhantomData  <T> { fn clone(&self) -> Self { Self::VALUE } }
impl<T> Copy for SmartPhantomData   <T> {}
impl<T> PartialEq for SmartPhantomData <T> { fn eq(&self, other: &Self) -> bool { true } }
impl<T> Eq for SmartPhantomData <T> {}
impl<T> Ord for SmartPhantomData <T> { fn cmp(&self, other: &Self) -> std::cmp::Ordering { std::cmp::Ordering::Equal  }}
impl<T> PartialOrd for SmartPhantomData <T> { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(std::cmp::Ordering::Equal) } }
impl<T> Debug for SmartPhantomData <T> { fn fmt(&self, f: &mut Formatter<'_>) -> DResult { Ok(()) } }
*/

pub struct GenVecID<T>
{
    phantom : PhantomData<T>,
    idx : usize,
    gen : GenerationIdx,
}
impl<T> Debug for GenVecID<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult {
        f.debug_struct("GenVecId").field("idx", &self.idx).field("gen", &self.gen).finish()
    }
}
impl<T> Clone for GenVecID<T>{
    fn clone(&self) -> Self {
        Self { phantom: PhantomData, idx: self.idx.clone(), gen: self.gen.clone() }
    }
} 
impl<T> Copy for GenVecID<T>{}
impl<T> PartialEq for GenVecID<T>{
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx && self.gen == other.gen
    }
}
impl<T> Eq for GenVecID<T>{}



impl<T> GenVecID<T>
{
    pub fn new(idx : usize, gen : GenerationIdx) -> Self { Self { phantom: PhantomData, idx, gen }}

    pub fn get(self, gen_vec : &GenVec<T>) -> Option<&T> { gen_vec.get(self) }
    pub fn get_mut(self, gen_vec : &mut GenVec<T>) -> Option<&mut T> { gen_vec.get_mut(self) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenVecEntry<T>
{
    value      : Option<T>,
    generation : GenerationIdx,
}

/// A vector that can be indexed by usize, or temporary key GenVecID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenVec<T>
{
    all : Vec<GenVecEntry<T>>,
    free_slot : Vec<usize>,
    // How many some
    len : usize,
}

impl<T> Default for GenVec<T>
{
    fn default() -> Self {
        Self { all: ___(), free_slot: ___(), len: 0 }
    }
}



impl<T> GenVec<T>
{
    pub fn new() -> Self { ___() }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ { self.all.iter().filter_map(|v| v.value.as_ref())}
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ { self.all.iter_mut().filter_map(|v| v.value.as_mut())}

    pub fn len_id(&self) -> usize { self.len }
    pub fn iter_id(&self) -> impl Iterator<Item = GenVecID<T>> + '_
    {
        self.all.iter().enumerate().filter_map(
            |(idx, _v)| 
                if self.all[idx].value.is_some() 
                { 
                    Some(GenVecID::new(idx, self.all[idx].generation))
                } else { None} 
            )
    }

    pub fn len_idx(&self) -> usize { self.all.len() }
    pub fn iter_idx(&self) -> impl Iterator<Item = usize> { 0..self.len_idx() }

    pub fn get_id(&self, idx : usize) -> GenVecID<T> { GenVecID::new(idx, self.all[idx].generation) }
    pub fn id_is_valid(&self, id : GenVecID<T>) -> bool 
    { 
        let e = &self.all[id.idx];
        e.generation == id.gen && e.value.is_some() 
    }

    pub fn remove(&mut self, id : GenVecID<T>) -> bool
    {
        if !self.id_is_valid(id) { return false; }
        self.all[id.idx].value = None;
        self.len -= 1;
        if self.all[id.idx].generation != GenerationIdx::MAX 
        {
            self.free_slot.push(id.idx);
        }
        return true;
    }

    pub fn push(&mut self, val : T) -> GenVecID<T>
    {
        self.len += 1;
        match self.free_slot.pop()
        {
            Some(idx) => { self.all[idx].value = Some(val); self.all[idx].generation += 1; self.get_id(idx) }
            None => 
            {
                self.all.push(GenVecEntry { value: Some(val), generation: 0 });
                self.get_id(self.all.len()-1)
            }
        }
    }

    pub fn get(&self, id : GenVecID<T>) -> Option<&T> { if self.id_is_valid(id) { self.get_idx(id.idx) } else { None } }
    pub fn get_mut(&mut self, id : GenVecID<T>) -> Option<&mut T> { if self.id_is_valid(id) { self.get_idx_mut(id.idx) } else { None } }

    pub fn get_idx(&self, idx : usize) -> Option<&T> { self.all[idx].value.as_ref() }
    pub fn get_idx_mut(&mut self, idx : usize) -> Option<&mut T> { self.all[idx].value.as_mut() }
}