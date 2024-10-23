use super::*;

/// Vector of `bool` type with 2 dimensions
pub type Bool2 = C2<bool>;
pub const fn bool2(x: bool, y: bool) -> Bool2 { Bool2::new(x, y) }

/// Vector of `bool` type with 3 dimensions
pub type Bool3 = C3<bool>;
pub const fn bool3(x: bool, y: bool, z: bool) -> Bool3 { Bool3::new(x, y, z) }

/// Vector of `bool` type with 4 dimensions
pub type Bool4 = C4<bool>;
pub const fn bool4(x: bool, y: bool, z: bool, w: bool) -> Bool4 { Bool4::new(x, y, z, w) }



/// Vector of `float` type with 2 dimensions
pub type Vec2 = C2<float>;
pub const fn vec2(x: float, y: float) -> Vec2 { Vec2::new(x, y) }
pub type Vec2Coef = Vec2;



/// Vector of `float` type with 3 dimensions
pub type Vec3 = C3<float>;
pub const fn vec3(x: float, y: float, z: float) -> Vec3 { Vec3::new(x, y, z) }
pub type Vec3Coef = Vec3;



/// Vector of `float` type with 4 dimensions
pub type Vec4 = C4<float>;
pub const fn vec4(x: float, y: float, z: float, w:float) -> Vec4 { Vec4::new(x, y, z, w) }
pub type Vec4Coef = Vec4;


/// Unsigned 2D point
pub type UPoint2 = C2<uint>;
pub const fn upoint2(x: uint, y: uint) -> UPoint2 { UPoint2::new(x, y) }

/// Unsigned 2D point
pub type UPoint3 = C3<uint>;
pub const fn upoint3(x: uint, y: uint, z: uint) -> UPoint3 { UPoint3::new(x, y, z) }

/// Unsigned 2D point
pub type UPoint4 = C4<uint>;
pub const fn upoint4(x: uint, y: uint, z: uint, w: uint) -> UPoint4 { UPoint4::new(x, y, z, w) }


/// Signed 2D point. Allow for relative position, so it is preferable over `UPoint2` even if you got half the range
pub type Point2 = C2<int>;
pub const fn point2(x: int, y: int) -> Point2 { Point2::new(x, y) }

/// Signed 2D point. Allow for relative position, so it is preferable over `UPoint3` even if you got half the range
pub type Point3 = C3<int>;
pub const fn point3(x: int, y: int, z: int) -> Point3 { Point3::new(x, y, z) }

/// Signed 2D point. Allow for relative position, so it is preferable over `UPoint4` even if you got half the range
pub type Point4 = C4<int>;
pub const fn point4(x: int, y: int, z: int, w: int) -> Point4 { Point4::new(x, y, z, w) }



/// Unsigned 2D integer point.
pub type Rect2u = Rect<UPoint2>;
pub const fn rect2u(x: uint, y: uint, w: uint, h: uint) -> Rect2u { Rect2u::new(upoint2(x, y), upoint2(w, h)) }

/// Unsigned 3D integer point.
pub type Rect3u = Rect<UPoint3>;
/// Unsigned 4D integer point.
pub type Rect4u = Rect<UPoint4>;


/// Signed 2D integer point.
pub type Rect2i = Rect<Point2>;
pub const fn rect2i(x: int, y: int, w: int, h: int) -> Rect2i { Rect2i::new(point2(x, y), point2(w, h)) }

/// Signed 3D integer point. 
pub type Rect3i = Rect<Point3>;

/// Signed 4D integer point. 
pub type Rect4i = Rect<Point4>;


/// 2D float rectangle
pub type Rect2f = Rect<Vec2>;
pub const fn rect2f(x: float, y: float, w: float, h: float) -> Rect2f { Rect2f::new(vec2(x, y), vec2(w, h)) }

/// 3D float rectangle
pub type Rect3f = Rect<Vec3>;

/// 4D float rectangle
pub type Rect4f = Rect<Vec4>;

pub type Rect2Coef = Rect2f;
pub type Rect3Coef = Rect3f;
pub type Rect4Coef = Rect4f;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() 
    {
        assert_eq!(upoint2(2, 4).y_x(), upoint2(4, 2));
        assert_eq!(upoint2(2, 4).x_x(), upoint2(2, 2));
        assert_eq!(upoint2(2, 4).y_y(), upoint2(4, 4));
        assert_eq!(point2(2, 4).rx_0(), point2(-2, 0));

        assert_eq!(point2(2, 4).to_vec2().length(), vec2(2.,4.).length());
    }
}
