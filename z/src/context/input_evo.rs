use super::*;

pub type InputFloat = InputEvo<float>;
pub type InputVec2 = InputEvo<Vec2>;
pub type InputBool = InputEvo<bool>;

/// Track the input evolution over time
#[derive(Clone, Copy, PartialEq, Deref)]
pub struct InputEvo<I : Copy + PartialEq, T : Copy + PartialEq=()>
{
    #[deref]
    pub cur  : I,
    pub old  : I,
    pub time : T,
}

impl<I : Copy + PartialEq + Debug, T : Copy + PartialEq + Debug>
    Debug for InputEvo<I, T>  { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("InputOf").field("val", &self.cur).field("old", &self.old).field("time", &self.time).finish() }}
impl<I : Copy + PartialEq + Default, T : Copy + PartialEq + Default>
    Default for InputEvo<I, T>  { fn default() -> Self { Self { cur: ___(), old: ___(), time : ___() } }}

impl<I : Copy + PartialEq + Default, T : Copy + PartialEq + Default>
    From<I> for InputEvo<I, T> { fn from(value: I) -> Self { Self { cur: value, .. ___() } }}

impl<I : Copy + PartialEq + Default, T : Copy + PartialEq + Default> InputEvo<I, T> 
{
    pub fn have_change(&self) -> bool { !self.is_same() } 
    pub fn is_same(&self) -> bool { self.cur == self.old } 
}
impl<I : Copy + PartialEq + Sub<I>,  T : Copy + PartialEq> InputEvo<I, T> { pub fn delta(&self) -> <I as Sub<I>>::Output { self.cur - self.old }}

impl<T : Copy + PartialEq> InputEvo<bool, T> 
{
    pub(crate) fn set_pressed(&mut self, pressed : bool) { self.cur = pressed; } 
}

impl<T : Copy + PartialEq> GetInputBoolExtension for InputEvo<bool, T> 
{
    fn is_pressed(&self) -> bool { self.cur }
    fn was_pressed(&self) -> bool { self.old }
}

impl<I : Copy + PartialEq + Default, T : Copy + PartialEq>  InputEvo<I, T> 
{
    pub fn new(cur : I, time : T) -> Self
    {
        Self::new_full(cur, ___(), time)
    }
}
impl<I : Copy + PartialEq, T : Copy + PartialEq>  InputEvo<I, T> 
{
    pub fn new_full(cur : I, old : I, time : T) -> Self
    {
        Self { cur, old, time }
    }

    pub fn update(&mut self, new : I, time : T) 
    {
        if self.old != self.cur
        {
            self.time = time;
            self.old = self.cur;
        }
        self.cur = new;
    }

    pub fn cur(&self) -> I { self.cur }
    pub fn old(&self) -> I { self.old }

    pub fn last_time_changed(&self) -> T { self.time }
}