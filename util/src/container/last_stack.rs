use super::*;

/// A stack that ALWAY have one element.
#[derive(Default, Clone, Debug, PartialEq, Eq, Deref)]
pub struct LastStack<T>
{
    /// will be readed frequently
    #[deref]
    last : T,
    stack : Vec<T>
}
impl<T> DerefMut for LastStack<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.last }}

/// A stack that always have at least one element,
/// where the last element can be frequently accessed
#[allow(clippy::len_without_is_empty)]
impl<T> LastStack<T>
{
    pub const fn new(value : T) -> Self { Self { last: value, stack: Vec::new() }}

    pub const fn last(&self) -> &T { &self.last }
    //pub const fn peek(&self) -> &T { self.last() }

    pub fn last_mut(&mut self) -> &mut T { &mut self.last }
    //pub fn peek_mut(&mut self) -> &mut T { self.last_mut() }

    pub fn keep_only_current(&mut self) { self.stack.clear() }
    pub fn keep_only_first(&mut self) 
    {
        if self.stack.is_empty() { return; }
        std::mem::swap(&mut self.last, &mut self.stack[0]);
        self.stack.clear();
    }

    pub fn set_last(&mut self, value : T) { self.last = value; } 
    pub fn push_val(&mut self, mut value : T) { std::mem::swap(&mut self.last, &mut value); self.stack.push(value); }
    /// Add a copy of the last value
    pub fn push(&mut self) where T : Clone { self.stack.push(self.last.clone()) } 
    pub fn pop(&mut self) { self.last = self.stack.pop().unwrap(); } 

    pub fn len(&self) -> usize  { self.stack.len() + 1 } 

    pub fn clone_last(&self) -> T where T : Clone { self.last.clone() }

    /* 
    pub fn remove_all(&mut self, val : &T) where T : PartialEq
    {
        self.stack.retain(|e| e != val);
        if &self.last == val
        {
            self.pop();
        }
    }
    */
}

impl<T> Index<usize> for LastStack<T>
{
    type Output=T;

    fn index(&self, index: usize) -> &Self::Output {
        if index + 1 == self.len()
        {
            &self.last
        }else 
        { 
            &self.stack[index]
        }
    }
}
impl<T> IndexMut<usize> for LastStack<T>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index + 1 == self.len()
        {
            &mut self.last
        }else 
        { 
            &mut self.stack[index]
        }
    }
}