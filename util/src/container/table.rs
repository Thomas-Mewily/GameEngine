use crate::*;

#[derive(Deref, DerefMut)]
pub struct TableIdx<K, V> 
{ 
    #[deref] #[deref_mut]
    val : usize, 
    _ph : PhantomData<(K, V)>,
}
impl<K, V> TableIdx<K, V>
{
    pub fn new(val : usize) -> Self { Self { val, _ph: ___() }}
}

impl<K,V> Copy for TableIdx<K,V> {}
impl<K,V> Clone for TableIdx<K,V> { fn clone(&self) -> Self { Self { val: self.val.clone(), _ph: self._ph.clone() } }}
impl<K,V> PartialEq for TableIdx<K,V> { fn eq(&self, other: &Self) -> bool { self.val == other.val } }
impl<K,V> Eq for TableIdx<K,V> {}

impl<K,V> Debug for TableIdx<K,V> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "#{}", self.val) }}

/// A vector where each entry can be named as many time as you want
pub struct Table<K : Hash + Eq + Debug, V : Debug>
{
    val : Vec<V>,
    named : HashMap<K, TableIdx<K,V>>
}

impl<K : Hash + Eq + Debug, V : Debug> Table<K, V>
{
    pub fn new() -> Self { ___() }


    pub fn push(&mut self, val : V) -> TableIdx<K,V>
    { 
        self.val.push(val);
        TableIdx::new(self.val.len()-1)
    }

    /// replace the name if it already exist
    pub fn add_name(&mut self, name : K, id : TableIdx<K,V>) 
    {
        self.named.insert(name, id);
    }

    /// replace the name if it already exist
    pub fn add_non_existing_name(&mut self, name : K, id : TableIdx<K,V>) -> Result<(), &str>
    {
        if !self.contains(&name) { self.add_name(name, id); Ok(()) } else { Err("name already defined")}
    }

    pub fn contains(&self, k : &K) -> bool { self.get(k).is_some() }

    pub fn len(&self) -> usize { self.val.len() }

    pub fn iter(&self) -> impl Iterator<Item = &V> { self.val.iter() }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut V> { self.val.iter_mut() }

    pub fn get_id<Q: ?Sized>(&self, k: &Q) -> Option<TableIdx<K,V>>
        where
        K: std::borrow::Borrow<Q>,
        Q: Hash + Eq,
    {
        self.named.get(&k).copied()
    }
    
    // Lot of constraint because that way Table<String, ...> can be indexed by `String`, or `&str`
    pub fn get<Q: ?Sized>(&self, k : &Q) -> Option<&V> 
    where
        K: std::borrow::Borrow<Q>,
        Q: Hash + Eq,
    { self.get_id(&k).map(|e| &self[e]) }

    pub fn get_mut<Q: ?Sized>(&mut self, k : &Q) -> Option<&mut V>
    where
    K: std::borrow::Borrow<Q>,
    Q: Hash + Eq,
    { self.get_id(k).map(|e| &mut self[e]) }
}



impl<K : Hash + Eq + Debug, V : Debug> Index<TableIdx<K,V>> for Table<K,V>
{
    type Output=V;
    fn index(&self, index: TableIdx<K,V>) -> &Self::Output {
        &self.val[index.val]
    }
}
impl<K : Hash + Eq + Debug, V : Debug> IndexMut<TableIdx<K,V>> for Table<K,V>
{
    fn index_mut(&mut self, index: TableIdx<K,V>) -> &mut Self::Output {
        &mut self.val[index.val]
    }
}

impl<K : Hash + Eq + Debug, V : Debug> Index<&K> for Table<K,V>
{
    type Output=V;
    fn index(&self, index: &K) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<K : Hash + Eq + Debug, V : Debug> IndexMut<&K> for Table<K,V>
{
    fn index_mut(&mut self, index: &K) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<K : Hash + Eq + Debug, V : Debug> Default for Table<K,V>
{
    fn default() -> Self {
        Self { val: ___(), named: ___() }
    }
}

impl<K : Hash + Eq + Debug, V : Debug> Debug for Table<K, V> 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Table").field("val", &self.val).field("named", &self.named).finish()
    }
}

impl<K : Hash + Eq + Debug + Clone, V : Debug + Clone> Clone for Table<K, V> 
{
    fn clone(&self) -> Self {
        Self { val: self.val.clone(), named: self.named.clone() }
    }
}