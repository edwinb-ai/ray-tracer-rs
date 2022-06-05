use crate::tuple::utils::float_eq;
use crate::tuple::{Tuple, Vector};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

// * Trait implementations
impl Tuple for Point {
    /// Create a `Point` centered at `(0, 0, 0)`
    fn origin() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    /// Create a new `Point` with position `(x, y, z)`
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }
}

impl PartialEq for Point {
    /// Allow for the comparison between a `Point` and
    /// another `Point`
    fn eq(&self, other: &Self) -> bool {
        let xtrue = float_eq(self.x, other.x);
        let ytrue = float_eq(self.y, other.y);
        let ztrue = float_eq(self.z, other.z);
        let wtrue = float_eq(self.w, other.w);

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
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

/// Subtract two `Point`s.
impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Vector {
        Vector::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

/// Subtract a `Point` and a `Vector`
impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Point {
        Point::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point::new(-self.x, -self.y, -self.z)
    }
}

/// Multiplication by a scalar, rhs
impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Point::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

/// Multiplication by a scalar, lhs
impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

/// Scalar division only
impl Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Point::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// * Useful macros
#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        Point::new($x as f64, $y as f64, $z as f64)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_origin_point() {
        let p = Point::origin();
        assert_eq!(0.0, p.w);
        assert_eq!(0.0, p.x);
        assert_eq!(0.0, p.y);
        assert_eq!(0.0, p.z);
    }

    #[test]
    fn test_new_point() {
        let p = Point::new(4.0, -4.0, 3.0);
        assert_eq!(4.0, p.x);
        assert_eq!(-4.0, p.y);
        assert_eq!(3.0, p.z);
        assert_eq!(0.0, p.w);
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
