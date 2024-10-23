use crate::*;

/// A `N` dimension rectangle
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Rect<C>
{
    pub pos  : C,
    pub size : C,
}

impl<C : HaveZero> HaveZero for Rect<C>  { const ZERO : Self = Self { pos : C::ZERO, size:C::ZERO }; }
impl<C : HaveZero + HaveOne> HaveOne for Rect<C>  { const ONE : Self = Self { pos : C::ZERO, size:C::ONE }; }
impl<C : HaveZero + HaveMinusOne> HaveMinusOne for Rect<C>  { const MINUS_ONE : Self = Self { pos : C::ZERO, size:C::MINUS_ONE }; }

impl<C : CoordinateScalarPositive> Rounding for Rect<C>
{
    fn floor(self) -> Self { self.map(|v| v.floor()) }
    fn round(self) -> Self { self.map(|v| v.round()) }
    fn ceil (self) -> Self { self.map(|v| v.ceil ()) }
    
    fn trunc(self) -> Self { self.map(|v| v.trunc()) }
}

impl<C>  Rect<C> 
{
    pub const fn new(pos : C, size : C) -> Self { Self { pos, size } }
    pub const fn new_sized(size : C) -> Self where C: HaveZero { Self { pos : C::ZERO, size } }
}

//impl<C : CoordinateScalar> Rectangle<C>
impl<C : CoordinateScalarPositive>  Rect<C> 
{
    pub fn new_from_pos_to_pos(start_pos : C, end_pos : C) -> Self { Self { pos: start_pos, size : end_pos - start_pos } }

    /// Put a area of a given size inside the current rectangle, and center it
    /// 
    /// This handle the scaling and the offset of the sub rectangle
    pub fn put_inside(&self, size_inside : C, center : C) -> Rect<C>
    {
        let area = size_inside * C::splat((self.size/size_inside).min_element());
        let unused_area = self.size - area;
        Rect::new(self.pos+unused_area * center, area)
    }

    pub fn new_centered(pos_middle : C, size : C) -> Self { Self::new(pos_middle-size/ (C::ONE + C::ONE), size) }
    pub fn new_center(bottom_left : C, size : C, center : C) -> Self { Self::new(bottom_left - center * size, size) }

    pub fn split_axis(self, nb : usize, axis : C) -> Vec<Self> where C : CoordinateScalar, Self : Moveable<C::Precision>
    {
        /* 
        let mut v = Vec::with_capacity(nb);
        let k = C::Precision::from_uint(nb);
        let axis_move = (self.size * axis).to_c4() / k;
        let mut s = self.with_size(self.size() * (C::ONE - axis + (axis / C::splat(k))));
        for _ in 0..nb
        {
            v.push(s);
            s.move_by(axis_move);
        }
        v
        */

        let mut v = Vec::with_capacity(nb);
        let mut old_pos = self.pos;
        let unchanged_hitbox = self.size() * (C::ONE - axis);
        
        for c in nb.iter_coef_skip_zero_finish_at_one()
        {
            let new_pos = self.pos + self.size * (axis * C::splat(C::Precision::from_float(c)));
            v.push(Rect::new_from_pos_to_pos(old_pos, new_pos + unchanged_hitbox));
            old_pos = new_pos;
        }
        v
    }
    pub fn split_x(self, nb : usize) -> Vec<Self> where C : CoordinateScalar, Self : Moveable<C::Precision> { self.split_axis(nb, C::X) }
    pub fn split_y(self, nb : usize) -> Vec<Self> where C : CoordinateScalar, Self : Moveable<C::Precision> { self.split_axis(nb, C::Y) }
    pub fn split_z(self, nb : usize) -> Vec<Self> where C : CoordinateScalar, Self : Moveable<C::Precision> { self.split_axis(nb, C::Z) }
    pub fn split_w(self, nb : usize) -> Vec<Self> where C : CoordinateScalar, Self : Moveable<C::Precision> { self.split_axis(nb, C::W) }

    pub fn bottom_left (&self) -> C { self.pos }
    pub fn bottom_right(&self) -> C { self.pos + self.size * C::X }
    pub fn top_right   (&self) -> C { self.pos + self.size }
    pub fn top_left    (&self) -> C { self.pos + self.size * C::Y }

    pub fn middle(&self) -> C { self.pos + self.size / (C::ONE + C::ONE) }
    pub fn middle_top   (&self) -> C { self.pos + self.size.with_x(self.size.x() / (C::Precision::ONE + C::Precision::ONE)) }
    pub fn middle_bot   (&self) -> C { self.pos + self.size.with_x(self.size.x() / (C::Precision::ONE + C::Precision::ONE)).with_y(zero()) }
    pub fn middle_right (&self) -> C { self.pos + self.size.with_y(self.size.y() / (C::Precision::ONE + C::Precision::ONE)) }
    pub fn middle_left  (&self) -> C { self.pos + self.size.with_y(self.size.y() / (C::Precision::ONE + C::Precision::ONE)).with_x(zero()) }

    pub fn area(&self) -> C::Precision { self.size.area() }
    //pub fn perimeter(&self) -> C::Precision { self.size.perimeter() }

    pub fn min(&self) -> C { self.pos }
    pub fn max(&self) -> C { self.pos + self.size }
    //pub fn max_excluded(&self) -> C { self.max() -  C::ONE }

    pub fn left_value  (&self) -> C::Precision { self.pos.x() }
    pub fn right_value (&self) -> C::Precision { self.pos.x() + self.size.x() }
    pub fn bot_value   (&self) -> C::Precision { self.pos.y() }
    pub fn top_value   (&self) -> C::Precision { self.pos.y() + self.size.y() }

    pub fn width(&self)  -> C::Precision { self.size.x() }
    pub fn height(&self) -> C::Precision { self.size.y() }

    pub fn pos(&self) -> C { self.pos }
    pub fn set_pos(&mut self, pos : C) -> &mut Self { self.pos = pos; self }
    pub fn set_pos_x(&mut self, x : C::Precision) -> &mut Self { self.pos.set_x(x); self }
    pub fn set_pos_y(&mut self, y : C::Precision) -> &mut Self { self.pos.set_y(y); self }
    pub fn set_pos_z(&mut self, z : C::Precision) -> &mut Self { self.pos.set_y(z); self }
    pub fn set_pos_w(&mut self, w : C::Precision) -> &mut Self { self.pos.set_w(w); self }

    pub fn with_pos(mut self, pos : C) -> Self { self.pos = pos; self }
    pub fn with_pos_x(mut self, x : C::Precision) -> Self { self.pos.set_x(x); self }
    pub fn with_pos_y(mut self, y : C::Precision) -> Self { self.pos.set_y(y); self }
    pub fn with_pos_z(mut self, z : C::Precision) -> Self { self.pos.set_z(z); self }
    pub fn with_pos_w(mut self, w : C::Precision) -> Self { self.pos.set_w(w); self }

    pub fn size(&self) -> C { self.size }
    pub fn set_size(&mut self, size : C) -> &mut Self { self.size = size; self }
    pub fn set_size_x(&mut self, x : C::Precision) -> &mut Self { self.size.set_x(x); self }
    pub fn set_size_y(&mut self, y : C::Precision) -> &mut Self { self.size.set_y(y); self }
    pub fn set_size_z(&mut self, z : C::Precision) -> &mut Self { self.size.set_y(z); self }
    pub fn set_size_w(&mut self, w : C::Precision) -> &mut Self { self.size.set_w(w); self }

    pub fn set_width(&mut self, x : C::Precision) -> &mut Self { self.set_size_x(x) }
    pub fn set_height(&mut self, y : C::Precision) -> &mut Self { self.set_size_y(y) }

    pub fn with_size(mut self, size : C) -> Self { self.size = size; self }
    pub fn with_size_x(mut self, x : C::Precision) -> Self { self.size.set_x(x); self }
    pub fn with_size_y(mut self, y : C::Precision) -> Self { self.size.set_y(y); self }
    pub fn with_size_z(mut self, z : C::Precision) -> Self { self.size.set_z(z); self }
    pub fn with_size_w(mut self, w : C::Precision) -> Self { self.size.set_w(w); self }

    pub fn is_inside(&self, coordinate : C) -> bool where C : Number
    {
        self.pos.cmp_all(coordinate, |a, c| c >= a)
        && 
        (self.pos + self.size).cmp_all(coordinate, |a, c| c <= a)
    }

    pub fn to_unsigned(self) -> Rect<C::SELF<<C::Precision as ToUnsigned>::UnsignedEquivalent>> { self.map(|c| c.to_unsigned()) }
    pub fn to_signed  (self) -> Rect<C::SELF<<C::Precision as ToSigned>::SignedEquivalent>> { self.map(|c| c.to_signed()) }

    pub fn to_uint(self) -> Rect<C::SELF::<uint>> { self.map(|c| c.to_uint()) }
    pub fn to_int (self) -> Rect<C::SELF::<int>>  { self.map(|c| c.to_int()) }
    pub fn to_float(self) -> Rect<C::SELF::<float>> { self.map(|c| c.to_float()) }
}

impl<C : CoordinateInteger> AreaIter for Rect<C>
{
    type Index = C;

    fn iter_area(self) -> impl Iterator<Item = Self::Index> { self.size.iter_area().map(move |v| v + self.pos) }
    fn iter_idx(self) -> impl Iterator<Item = Self::Index> { self.size.iter_idx().map(move |v| v + self.pos) }
}

impl<C : CoordinateScalarPositive> Clamp for Rect<C>
{
    fn clamp(mut self, max_included : Self) -> Self 
    {
        self.pos = self.pos.max(max_included.pos);
        self.size = self.pos - self.max().clamp(max_included.max());
        self
    }
}

impl<T> Rect<T>
{
    pub fn map<Z, F>(self, f : F) -> Rect<Z> where F : Fn(T) -> Z { Rect::<Z>::new(f(self.pos), f(self.size)) }
    pub fn cmp_map<Z, F>(self, other : Self, f : F) -> Rect<Z> where F : Fn(T, T) -> Z { Rect::<Z>::new(f(self.pos, other.pos), f(self.size, other.size)) }
}


//impl<T : Display> Display for Rectangle<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "rect({}, {})", self.pos, self.size) }}
impl<T : Debug> Debug for Rect<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "rect({:?}, {:?})", self.pos, self.size) }}

//impl<T> Into<(C2<T>, C2<T>)> for Rectangle2<C2<T>> { fn into(self) -> (C2<T>, C2<T>) { (self.pos, self.size)}}
impl<C : Coordinate> Into<(C, C)> for Rect<C> { fn into(self) -> (C, C) { (self.pos, self.size)}}
impl<C : Coordinate> From<(C, C)> for Rect<C> { fn from(rect: (C, C)) -> Self { Self::new(rect.0, rect.1) }}

impl<C : Coordinate> Into<[C; 2]> for Rect<C> { fn into(self) -> [C; 2] { [self.pos, self.size]}}
impl<C : Coordinate> From<[C; 2]> for Rect<C> { fn from(rect: [C; 2]) -> Self { Self::new(rect[0], rect[1]) }}

impl<C : Add<C, Output = C> + Copy> Add<C> for Rect<C> { type Output=Self; fn add(self, rhs: C) -> Self::Output { Self::new(self.pos.add(rhs), self.size.add(rhs)) }}
impl<C : AddAssign<C> + Copy> AddAssign<C> for Rect<C> { fn add_assign(&mut self, rhs: C) { self.pos.add_assign(rhs); self.size.add_assign(rhs); }}

impl<C : Sub<C, Output = C> + Copy> Sub<C> for Rect<C> { type Output=Self; fn sub(self, rhs: C) -> Self::Output { Self::new(self.pos.sub(rhs), self.size.sub(rhs)) }}
impl<C : SubAssign<C> + Copy> SubAssign<C> for Rect<C> { fn sub_assign(&mut self, rhs: C) { self.pos.sub_assign(rhs); self.size.sub_assign(rhs); }}

impl<C : Mul<C, Output=C> + Copy> Mul<C> for Rect<C> { type Output=Self; fn mul(self, rhs: C) -> Self::Output { Self::new(self.pos.mul(rhs), self.size.mul(rhs)) }}
impl<C : MulAssign<C> + Copy> MulAssign<C> for Rect<C> { fn mul_assign(&mut self, rhs: C) { self.pos.mul_assign(rhs); self.size.mul_assign(rhs); }}

impl<C : Div<C, Output=C> + Copy> Div<C> for Rect<C> { type Output=Self; fn div(self, rhs: C) -> Self::Output { Self::new(self.pos.div(rhs), self.size.div(rhs)) }}
impl<C : DivAssign<C> + Copy> DivAssign<C> for Rect<C> { fn div_assign(&mut self, rhs: C) { self.pos.div_assign(rhs); self.size.div_assign(rhs); }}

impl<C : Rem<C, Output=C> + Copy> Rem<C> for Rect<C> { type Output=Self; fn rem(self, rhs: C) -> Self::Output { Self::new(self.pos.rem(rhs), self.size.rem(rhs)) }}
impl<C : RemAssign<C> + Copy> RemAssign<C> for Rect<C> { fn rem_assign(&mut self, rhs: C) { self.pos.rem_assign(rhs); self.size.rem_assign(rhs); }}


impl<C : CoordinateScalar + Moveable<C::Precision>> Moveable<C::Precision> for Rect<C>
{
    fn move_by(&mut self, v : impl Into<C4<C::Precision>>) -> &mut Self {
        self.pos.move_by(v); self
    }
}

/* 
impl<C : CoordinateScalar> Rectangle<C>
    where
        C : Moveable<C::Precision>, 
        C::Precision : Neg<Output = C::Precision>,
{
    /* 
    pub fn move_by(&mut self, delta : C) -> &mut Self 
    { 
        self.pos += delta;
        self
    }*/

    pub fn add_margin_top(&mut self, margin_top : C::Precision) -> &mut Self
    { 
        self.size.move_neg_y(margin_top);
        self
    }

    
    pub fn add_margin_bot(&mut self, margin_bot : C::Precision) -> &mut Self
    {
        self.add_margin_top(margin_bot);
        self.pos.move_y(margin_bot);
        self
    }

    pub fn add_margin_right(&mut self, margin_right : C::Precision) -> &mut Self
    {
        self.size.move_neg_x(margin_right);
        self
    }

    pub fn add_margin_left(&mut self, margin_left : C::Precision) -> &mut Self 
    {
        self.add_margin_right(margin_left);
        self.pos.move_x(margin_left);
        self
    }

    pub fn add_margin_left_and_right(&mut self, margin_right_and_also_left : C::Precision) -> &mut Self
    {
        self.add_margin_left(margin_right_and_also_left).add_margin_right(margin_right_and_also_left)
    }

    pub fn add_margin_top_and_bot(&mut self, margin_top_and_also_bot : C::Precision) -> &mut Self
    {
        self.add_margin_top(margin_top_and_also_bot).add_margin_bot(margin_top_and_also_bot)
    }

    pub fn add_margin(&mut self, margin_each_side : C) -> &mut Self
    {
        self.add_margin_left_and_right(margin_each_side.x()).add_margin_top_and_bot(margin_each_side.y())
    }


    pub fn glue_bot(&mut self, size_bot : C::Precision) -> &mut Self
    { self.add_margin_top(self.size.y() - size_bot); self }

    pub fn glue_top(&mut self, size_top : C::Precision) -> &mut Self
    { self.add_margin_bot(self.size.y() - size_top); self }

    pub fn glue_right(&mut self, size_right : C::Precision) -> &mut Self
    { self.add_margin_left(self.size.x() - size_right); self }

    pub fn glue_left(&mut self, size_left : C::Precision) -> &mut Self
    { self.add_margin_right(self.size.x() - size_left); self }
}*/