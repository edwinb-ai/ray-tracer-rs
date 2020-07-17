use crate::tuple::utils::float_eq;
use crate::tuple::{Tuple, Vector};
use std::ops::{Add, Sub, Neg, Mul, Div};

pub struct Point {
    _x: f64,
    _y: f64,
    _z: f64,
    _w: f64,
}

// * Trait implementations
impl Tuple for Point {
    /// Create a `Point` centered at `(0, 0, 0)`
    fn origin() -> Self {
        Point {
            _x: 0.0,
            _y: 0.0,
            _z: 0.0,
            _w: 0.0,
        }
    }

    /// Create a new `Point` with position `(x, y, z)`
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point {
            _x: x,
            _y: y,
            _z: z,
            _w: 0.0,
        }
    }

    fn get_x(&self) -> f64 {
        self._x
    }

    fn get_y(&self) -> f64 {
        self._y
    }

    fn get_z(&self) -> f64 {
        self._z
    }

    fn get_w(&self) -> f64 {
        self._w
    }
}

impl PartialEq for Point {
    /// Allow for the comparison between a `Point` and
    /// another `Point`
    fn eq(&self, other: &Self) -> bool {
        let xtrue = float_eq(self._x, other.get_x());
        let ytrue = float_eq(self._y, other.get_y());
        let ztrue = float_eq(self._z, other.get_z());
        let wtrue = float_eq(self._w, other.get_w());

        if xtrue && ytrue && ztrue && wtrue {
            true
        } else {
            false
        }
    }
}

/// Add a `Point` and a `Vector`
/// 
/// When you add a `Point` and a `Vector` the result
/// should always be a `Point`.
/// 
/// # Examples
/// ```
/// use ray_tracer::tuple::*;
/// 
/// let p1 = Point::new(3.0, -2.0, 5.0);
/// let v1 = Vector::new(-2.0, 3.0, 1.0);
/// let res: Point = p1 + v1; // Should be of type `Point`
/// ```
impl Add<Vector> for Point {
    type Output = Point;
    
    fn add(self, other: Vector) -> Point {
        Point::new(
            self._x + other.get_x(),
            self._y + other.get_y(),
            self._z + other.get_z(),
        )
    }
}


/// Subtract two `Point`s.
impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Vector {
        Vector::new(
            self._x - other.get_x(),
            self._y - other.get_y(),
            self._z - other.get_z(),
        )
    }
}

/// Subtract a `Point` and a `Vector`
impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Point {
        Point::new(
            self._x - other.get_x(),
            self._y - other.get_y(),
            self._z - other.get_z(),
        )
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point::new(
            -self._x,
            -self._y,
            -self._z
        )
    }
}

/// Multiplication by a scalar, rhs
impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Point::new(
            self._x * rhs,
            self._y * rhs,
            self._z * rhs
        )
    }
}

/// Multiplication by a scalar, lhs
impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point::new(
            self * rhs.get_x(),
            self * rhs.get_y(),
            self * rhs.get_z()
        )
    }
}

/// Scalar division only
impl Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Point::new(
            self._x / rhs,
            self._y / rhs,
            self._z / rhs
        )
    }
}

// * Useful macros
#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        Point::new($x, $y, $z)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_origin_point() {
        let p = Point::origin();
        assert_eq!(0.0, p.get_w());
        assert_eq!(0.0, p.get_x());
        assert_eq!(0.0, p.get_y());
        assert_eq!(0.0, p.get_z());
    }

    #[test]
    fn test_new_point() {
        let p = Point::new(4.0, -4.0, 3.0);
        assert_eq!(4.0, p.get_x());
        assert_eq!(-4.0, p.get_y());
        assert_eq!(3.0, p.get_z());
        assert_eq!(0.0, p.get_w());
    }

    #[test]
    fn test_equality_point() {
        let p1 = Point::origin();
        let p2 = Point::origin();
        assert!(p1 == p2);
    }

    #[test]
    fn test_add_point_vector() {
        let p1 = Point::new(3.0, -2.0, 5.0);
        let v1 = Vector::new(-2.0, 3.0, 1.0);
        let res = Point::new(1.0, 1.0, 6.0);
        assert!((p1 + v1) == res);
    }

    #[test]
    fn test_subtract_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        let res = Vector::new(-2.0, -4.0, -6.0);
        assert!(p1 - p2 == res);
    }

    #[test]
    fn test_subtract_point_vector() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let p1 = Point::new(5.0, 6.0, 7.0);
        let res = Point::new(2.0, 4.0, 6.0);
        assert!(p1 - v1 == res);
    }

    #[test]
    fn negate_point() {
        let p1 = Point::new(1.0, -2.0, 3.0);
        let res = Point::new(-1.0, 2.0, -3.0);
        assert!(-p1 == res);
    }

    #[test]
    fn point_scalar_multiplication_rhs() {
        let p1 = Point::new(1.0, -2.0, 3.0);
        let a = 3.5;
        let result = Point::new(3.5, -7.0, 10.5);
        assert!(a * p1 == result);
    }

    #[test]
    fn point_scalar_multiplication_lhs() {
        let p1 = Point::new(1.0, -2.0, 3.0);
        let a = 3.5;
        let result = Point::new(3.5, -7.0, 10.5);
        assert!(p1 * a == result);
    }

    #[test]
    fn point_scalar_division_rhs() {
        let p1 = Point::new(1.0, -2.0, 3.0);
        let a = 2.0;
        let result = Point::new(0.5, -1.0, 1.5);
        assert!(p1 / a == result);
    }

    #[test]
    fn point_macro() {
        let p1 = point!(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.0);
        assert!(p1 == p2);
    }
}
