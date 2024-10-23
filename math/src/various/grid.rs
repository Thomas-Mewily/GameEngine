
use std::{clone, fmt::{Debug, Display, Formatter, Result}, marker::PhantomData, ops::{Index, IndexMut, Range}};
use crate::*;

pub type Grid2<T> = Grid<Point2, T>;
pub type Grid3<T> = Grid<Point3, T>;
pub type Grid4<T> = Grid<Point4, T>;

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct Grid<C : CoordinateIndex, T>
{
    size : C,
    all  : Vec<T>,
}

impl<C : CoordinateIndex, T> Grid<C, T> 
{
    pub fn iter_without_pos(&self) -> impl Iterator<Item=&T> { self.all.iter() }
    pub fn iter_mut_without_pos(&mut self) -> impl Iterator<Item=&mut T> { self.all.iter_mut() }
    
    pub fn iter(&self) -> impl Iterator<Item=(C, &T)> { self.iter_without_pos().enumerate().map(|(idx, v)| (C::from_idx_unchecked(idx as int, self.size), v)) }
    pub fn iter_mut(&mut self) -> impl Iterator<Item=(C, &mut T)> { let s = self.size; self.iter_mut_without_pos().enumerate().map(move |(idx, v)| (C::from_idx_unchecked(idx as int, s), v)) }
    
    pub fn iter_idx(&self) -> impl Iterator<Item=C> { self.size.iter_area() }

    pub fn iter_idx_rect(&self, r : Rect<C>) -> impl Iterator<Item=C> { r.clamp(self.rect()).iter_idx() }
    
    pub fn iter_rect(&self, r : Rect<C>) -> impl Iterator<Item=(C, &T)> { r.clamp(self.rect()).iter_idx().map(|v| (v, &self[v])) }
    pub fn iter_rect_mut(&mut self, r : Rect<C>) -> impl Iterator<Item=(C, &mut T)> 
    { 
        let s = self.size; 
        r.clamp(self.rect()).iter_idx().map(move |pos|
            {
                let idx = C::to_idx_unchecked(pos, s).to_uint();
                // SAFETY : each position is unique, so the same tiles can't be returned twice or more,
                // so each element only have one mutable borrow at a time
                unsafe { (pos, (&mut self.all as *mut Vec<T>).as_mut().unwrap().get_mut(idx).unwrap()) }
            }
        )
    }

    pub fn iter_orthogonal_idx(&self, pos : C) -> impl Iterator<Item=C> { let s = self.size(); pos.iter_orthogonal().filter(move |pos| (*pos).is_inside(s)) }
    pub fn iter_diagonal_idx(&self, pos : C) -> impl Iterator<Item=C> { let s = self.size(); pos.iter_diagonal().filter(move |pos| (*pos).is_inside(s)) }
    pub fn iter_ortho_diagonal_idx(&self, pos : C) -> impl Iterator<Item=C> { let s = self.size(); pos.iter_ortho_diagonal().filter(move |pos| (*pos).is_inside(s)) }

    pub fn iter_orthogonal_idx_mut(&mut self, pos : C) -> impl Iterator<Item=(C, &mut T)> 
    { 
        let s = self.size();
        unsafe 
        {
            // SAFETY : each orthogonal position is unique, so the same tiles can't be returned twice or more,
            // so each element only have one mutable borrow at a time
            self.iter_orthogonal_idx(pos).map(
                move |v| (v, (&mut self.all as *mut Vec<T>).as_mut().unwrap().get_mut(C::to_idx_unchecked(v, s).to_uint()).unwrap()) 
            )
        }
    }
    pub fn iter_diagonal_idx_mut(&mut self, pos : C) -> impl Iterator<Item=(C, &mut T)> 
    { 
        let s = self.size();
        unsafe 
        {
            // SAFETY : each diagonal position is unique, so the same tiles can't be returned twice or more,
            // so each element only have one mutable borrow at a time
            self.iter_diagonal_idx(pos).map(
                move |v| (v, (&mut self.all as *mut Vec<T>).as_mut().unwrap().get_mut(C::to_idx_unchecked(v, s).to_uint()).unwrap()) 
            )
        }
    }
    pub fn iter_ortho_diagonal_idx_mut(&mut self, pos : C) -> impl Iterator<Item=(C, &mut T)> 
    { 
        let s = self.size();
        unsafe 
        {
            // SAFETY : each ortho_diagonal position is unique, so the same tiles can't be returned twice or more,
            // so each element only have one mutable borrow at a time
            self.iter_ortho_diagonal_idx(pos).map(
                move |v| (v, (&mut self.all as *mut Vec<T>).as_mut().unwrap().get_mut(C::to_idx_unchecked(v, s).to_uint()).unwrap()) 
            )
        }
    }
}

impl<C : CoordinateIndex, T> Grid<C, T>
{
    pub fn new_map<F>(size : C, f : F) -> Self where F : Fn(C) -> T 
    {
        assert_eq!(size, size.max(C::ZERO), "negative grid size : {size:?}");

        Self
        {
            size,
            all : 
            {
                let s = size.area();
                let mut v = Vec::with_capacity(s.to_uint());
                for idx in 0.. s
                {
                    v.push(f(C::from_idx(idx, size).unwrap()));
                }
                v
            }
        }
    }

    pub fn new(size : C) -> Self where T : Clone + Default { Self::new_with_tile(size, T::default())}
    pub fn new_with_tile(size : C, tile : T) -> Self where T : Clone { Self::new_map(size, |_idx| tile.clone()) }


    pub fn iter_x(&self) -> Range<C::Precision> { C::Precision::ZERO..self.size.x() }
    pub fn iter_y(&self) -> Range<C::Precision> { C::Precision::ZERO..self.size.y() }
    pub fn iter_z(&self) -> Range<C::Precision> { C::Precision::ZERO..self.size.z() }
    pub fn iter_w(&self) -> Range<C::Precision> { C::Precision::ZERO..self.size.w() }

    pub fn rect  (&self) -> Rect<C> { self.recti() }
    
    pub fn rectu (&self) -> Rect<C::SELF<uint>> { self.recti().to_uint() }
    pub fn recti (&self) -> Rect<C> { Rect::<C>::new(zero(), self.size()) }
    pub fn rectf (&self) -> Rect<C::SELF<float>> { self.recti().to_float() }

    pub fn size  (&self) -> C { self.size }

    #[inline] pub fn is_inside_x(&self, x : C::Precision) -> bool { x >= C::Precision::ZERO && x < self.size.x() }
    #[inline] pub fn is_inside_y(&self, y : C::Precision) -> bool { y >= C::Precision::ZERO && y < self.size.y() }
    #[inline] pub fn is_inside_z(&self, z : C::Precision) -> bool { z >= C::Precision::ZERO && z < self.size.z() }
    #[inline] pub fn is_inside_w(&self, w : C::Precision) -> bool { w >= C::Precision::ZERO && w < self.size.w() }

    #[inline] pub fn is_outside_x(&self, x : C::Precision) -> bool { !self.is_inside_x(x) }
    #[inline] pub fn is_outside_y(&self, y : C::Precision) -> bool { !self.is_inside_y(y) }
    #[inline] pub fn is_outside_z(&self, z : C::Precision) -> bool { !self.is_inside_z(z) }
    #[inline] pub fn is_outside_w(&self, w : C::Precision) -> bool { !self.is_inside_w(w) }

    #[inline] pub fn is_inside(&self, pos : C) -> bool { pos.is_inside(self.size) }
    #[inline] pub fn is_outside(&self, pos : C) -> bool { !self.is_inside(pos) }
    
    pub fn coordinate_to_idx_checked(&self, pos : C) -> Option<C::Precision> { C::to_idx(pos, self.size) }
    pub fn idx_to_coordinate_checked(&self, idx : C::Precision) -> Option<C> { C::from_idx(idx, self.size) }

    pub fn coordinate_to_idx_uncheck(&self, pos : C) -> C::Precision { C::to_idx_unchecked(pos, self.size) }
    pub fn idx_to_uncheck_uncheck(&self, idx : C::Precision) -> C { C::from_idx_unchecked(idx, self.size) }

    pub fn coordinate_to_idx(&self, pos : C) -> C::Precision { self.coordinate_to_idx_checked(pos).expect("coordinate out of grid") }
    pub fn idx_to_uncheck(&self, idx : C::Precision) -> C { self.idx_to_coordinate_checked(idx).expect("coordinate out of grid") }

    pub fn get(&self, pos : C) -> Option<&T> { self.coordinate_to_idx_checked(pos).map(|i| &self.all[i.to_uint()]) }
    pub fn get_mut(&mut self, pos : C) -> Option<&mut T>  { self.coordinate_to_idx_checked(pos).map(|i| &mut self.all[i.to_uint()]) }

    pub fn swap(&mut self, pos_a : C, pos_b : C) 
    { 
        let a = self.coordinate_to_idx(pos_a).to_uint();
        let b = self.coordinate_to_idx(pos_b).to_uint();
        self.all.swap(a, b)
    }

    pub fn set(&mut self, val : T, pos : C) { self[pos] = val; }
    pub fn try_set(&mut self, val : T, pos : C) -> Option<T> { self.get_mut(pos).map(|e| std::mem::replace(e, val)) }

    /// Clamp the rectangle and apply a function
    pub fn map_rect<A>(&mut self, r : Rect<C>, action : A) -> &mut Self
        where A : Fn(C, &mut T) { self.iter_rect_mut(r).for_each(|(idx, val)| action(idx, val)); self }
}
impl<C : CoordinateIndex, T> Length for Grid<C, T> { fn len(&self) -> usize { self.all.len() } }

impl<C : CoordinateIndex, T> Index<C> for Grid<C, T> { type Output=T; fn index(&self, index: C) -> &Self::Output { self.get(index.into()).expect("idx out of grid") }}
impl<C : CoordinateIndex, T> IndexMut<C> for Grid<C, T> { fn index_mut(&mut self, index: C) -> &mut Self::Output { self.get_mut(index.into()).expect("idx out of grid") }}