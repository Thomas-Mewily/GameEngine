use super::*;

pub trait Moveable<T : Scalar>
{
    fn move_by(&mut self, v : impl Into<C4<T>>) -> &mut Self;

    fn move_x(&mut self, v : T) -> &mut Self { self.move_by(C4::<T>::X * v) }
    fn move_y(&mut self, v : T) -> &mut Self { self.move_by(C4::<T>::Y * v) }
    fn move_z(&mut self, v : T) -> &mut Self { self.move_by(C4::<T>::Z * v) }
    fn move_w(&mut self, v : T) -> &mut Self { self.move_by(C4::<T>::W * v) }

    fn move_neg_x(&mut self, v : T) -> &mut Self where T : Neg<Output = T> { self.move_by(C4::<T>::X * -v) }
    fn move_neg_y(&mut self, v : T) -> &mut Self where T : Neg<Output = T> { self.move_by(C4::<T>::Y * -v) }
    fn move_neg_z(&mut self, v : T) -> &mut Self where T : Neg<Output = T> { self.move_by(C4::<T>::Z * -v) }
    fn move_neg_w(&mut self, v : T) -> &mut Self where T : Neg<Output = T> { self.move_by(C4::<T>::W * -v) }



    fn with_move_by(mut self, v : impl Into<C4<T>>) -> Self where Self : Copy { self.move_by(v); self }

    fn with_move_x(self, v : T) -> Self where Self : Copy { self.with_move_by(C4::<T>::X * v) }
    fn with_move_y(self, v : T) -> Self where Self : Copy { self.with_move_by(C4::<T>::Y * v) }
    fn with_move_z(self, v : T) -> Self where Self : Copy { self.with_move_by(C4::<T>::Z * v) }
    fn with_move_w(self, v : T) -> Self where Self : Copy { self.with_move_by(C4::<T>::W * v) }

    fn with_move_neg_x(self, v : T) -> Self where Self : Copy, T : Neg<Output = T> { self.with_move_by(C4::<T>::X * -v) }
    fn with_move_neg_y(self, v : T) -> Self where Self : Copy, T : Neg<Output = T> { self.with_move_by(C4::<T>::Y * -v) }
    fn with_move_neg_z(self, v : T) -> Self where Self : Copy, T : Neg<Output = T> { self.with_move_by(C4::<T>::Z * -v) }
    fn with_move_neg_w(self, v : T) -> Self where Self : Copy, T : Neg<Output = T> { self.with_move_by(C4::<T>::W * -v) }
}