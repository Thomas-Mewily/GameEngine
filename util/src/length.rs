use super::*;

pub trait Length
{
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn is_not_empty(&self) -> bool { !self.is_empty() }
}

macro_rules! impl_length {
    ($($t:ident),*) => {
        $(
            impl<T> Length for $t<T> { fn len(&self) -> usize { Self::len(&self) }}
        )*
    };
}
impl_length!(Vec, VecDeque, HashSet);
impl<K, V, S> Length for HashMap<K, V, S>
{
    fn len(&self) -> usize { self.len() }
}
impl<T> Length for [T]
{
    fn len(&self) -> usize { self.len() }
}

impl<T, const L : usize> Length for [T; L]
{
    fn len(&self) -> usize { L }
}




pub trait IteratorExtension<T>
{
    fn collect_to_vec(self) -> Vec<T>;
}

impl<It : Iterator<Item=T>, T> IteratorExtension<T> for It
{
    fn collect_to_vec(self) -> Vec<T> {
        let vec : Vec<T> = self.collect();
        vec
    }
}