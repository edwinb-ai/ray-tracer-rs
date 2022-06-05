use crate::tuple::utils::float_eq;
use crate::tuple::{Point, Tuple};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

// * Methods for `Vector`
impl Vector {
    /// Compute the magnitude of a `Vector`
    ///
    /// Compute the un-normalized magnitude of a `Vector`.
    ///
    /// # Examples
    /// ```
    /// use ray_tracer::tuple::*;
    ///
    /// let v1 = Vector::new(-1.0, -2.0, -3.0);
    /// let res: f64 = 14.0;
    /// assert_eq!(v1.magnitude(), res.sqrt());
    /// ```
    pub fn magnitude(&self) -> f64 {
        let mut magn: f64 = self.x * self.x;
        magn += self.y * self.y;
        magn += self.z * self.z;

        magn.sqrt()
    }

    /// Compute the magnitude of a `Vector` and normalize it.
    ///
    /// This should always return a new `Vector` whose magnitude
    /// should be always 1.0.
    ///
    /// # Examples
    /// ```
    /// use ray_tracer::tuple::{Tuple, Vector};
    /// use ray_tracer::vector;
    ///
    /// let v1 = vector!(4.0, 0.0, 0.0);
    /// let res = vector!(1.0, 0.0, 0.0);
    /// assert!(v1.normalize() == res);
    /// ```
    pub fn normalize(&self) -> Self {
        let magn = self.magnitude();
        Vector::new(self.x / magn, self.y / magn, self.z / magn)
    }

    /// Compute the dot product between two `Vector`s
    ///
    /// The dot product is defined as the sum of the
    /// elementwise multiplication between two vectors.
    /// It should always return a scalar value.
    ///
    /// # Examples
    /// ```
    /// use ray_tracer::tuple::{Tuple, Vector};
    /// use ray_tracer::vector;
    ///
    /// let v1 = vector!(1, 2, 3);
    /// let v2 = vector!(2, 3, 4);
    /// // This is 2*1 + 2*3 + 3*4 = 20
    /// let res: f64 = 20.0;
    /// assert_eq!(v1.dot(&v2), res);
    /// ```
    pub fn dot(&self, rhs: &Vector) -> f64 {
        let mut dot_value = self.x * rhs.x;
        dot_value += self.y * rhs.y;
        dot_value += self.z * rhs.z;

        dot_value
    }

    /// Compute the cross product between two `Vector`s
    ///
    /// The cross product will return a perpendicular `Vector`
    /// to both the input `Vector`s, such that their dot product
    /// should be zero.
    ///
    /// # Examples
    /// ```
    /// use ray_tracer::tuple::{Tuple, Vector};
    /// use ray_tracer::vector;
    ///
    /// let v1 = vector!(1, 2, 3);
    /// let v2 = vector!(2, 3, 4);
    /// let res= vector!(-1, 2, -1);
    /// assert!(v1.cross(&v2) == res);
    /// ```
    pub fn cross(&self, rhs: &Vector) -> Self {
        let x_component = self.y * rhs.z - self.z * rhs.y;
        let y_component = self.z * rhs.x - self.x * rhs.z;
        let z_component = self.x * rhs.y - self.y * rhs.x;

        Vector::new(x_component, y_component, z_component)
    }
}

// * Trait implementations
impl Tuple for Vector {
    /// Create a `Vector` centered at `(0, 0, 0)`
    fn origin() -> Self {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    /// Create a new `Vector` with position `(x, y, z)`
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }
}

impl PartialEq for Vector {
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

/// Add two `Vector`s together
impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

/// Add a `Vector` and a `Point`
impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

/// Subtract two `Vector`s.
impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Subtract a `Vector` and a `Point`
impl Sub<Point> for Vector {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Unary negation on a `Vector`
impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

/// Multiplication by a scalar, rhs
impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

/// Multiplication by a scalar, lhs
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

/// Scalar division only
impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// * Useful macros
#[macro_export]
macro_rules! vector {
    ($x:expr, $y:expr, $z:expr) => {
        Vector::new($x as f64, $y as f64, $z as f64)
    };
}

// * Unit tests for `Vector`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_origin_vector() {
        let v = Vector::origin();
        assert_eq!(0.0, v.x);
        assert_eq!(0.0, v.y);
        assert_eq!(0.0, v.z);
        assert_eq!(1.0, v.w);
    }

    #[test]
    fn test_new_vector() {
        let v = Vector::new(4.0, -4.0, 3.0);
        assert_eq!(4.0, v.x);
        assert_eq!(-4.0, v.y);
        assert_eq!(3.0, v.z);
        assert_eq!(1.0, v.w);
    }

    #[test]
    fn test_equality_vector() {
        let v1 = Vector::origin();
        let v2 = Vector::origin();
        assert!(v1 == v2);
    }

    #[test]
    fn test_add_two_vectors() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        let res = Vector::new(1.0, 1.0, 6.0);
        assert!(v1 + v2 == res);
    }

    #[test]
    fn test_add_vector_point() {
        let p1 = Point::new(3.0, -2.0, 5.0);
        let v1 = Vector::new(-2.0, 3.0, 1.0);
        let res = Point::new(1.0, 1.0, 6.0);
        assert!(v1 + p1 == res);
    }

    #[test]
    fn test_subtract_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        let res = Vector::new(-2.0, -4.0, -6.0);
        assert!(v1 - v2 == res);
    }

    #[test]
    fn test_subtract_vector_point() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let p1 = Point::new(5.0, 6.0, 7.0);
        let res = Point::new(-2.0, -4.0, -6.0);
        assert!(v1 - p1 == res);
    }

    #[test]
    fn subtract_from_origin() {
        let v1 = Vector::origin();
        let v2 = Vector::new(1.0, -2.0, 3.0);
        let res = Vector::new(-1.0, 2.0, -3.0);
        assert!(v1 - v2 == res);
    }

    #[test]
    fn negate_vector() {
        let v1 = Vector::new(1.0, -2.0, 3.0);
        let res = Vector::new(-1.0, 2.0, -3.0);
        assert!(-v1 == res);
    }

    #[test]
    fn vector_scalar_multiplication_rhs() {
        let v1 = Vector::new(1.0, -2.0, 3.0);
        let a = 3.5;
        let result = Vector::new(3.5, -7.0, 10.5);
        assert!(a * v1 == result);
    }

    #[test]
    fn vector_scalar_multiplication_lhs() {
        let v1 = Vector::new(1.0, -2.0, 3.0);
        let a = 3.5;
        let result = Vector::new(3.5, -7.0, 10.5);
        assert!(v1 * a == result);
    }

    #[test]
    fn vector_scalar_division_rhs() {
        let v1 = Vector::new(1.0, -2.0, 3.0);
        let a = 2.0;
        let result = Vector::new(0.5, -1.0, 1.5);
        assert!(v1 / a == result);
    }

    #[test]
    fn unitary_magnitudes() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(v1.magnitude(), 1.0);

        let v1 = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v1.magnitude(), 1.0);

        let v1 = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(v1.magnitude(), 1.0);
    }

    #[test]
    fn vector_magnitude() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let res: f64 = 14.0;
        assert_eq!(v1.magnitude(), res.sqrt());

        let v1 = Vector::new(-1.0, -2.0, -3.0);
        let res: f64 = 14.0;
        assert_eq!(v1.magnitude(), res.sqrt());
    }

    #[test]
    fn vector_macro() {
        let v1 = vector!(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        assert!(v1 == v2);
    }

    #[test]
    fn normalize_vector() {
        let v1 = vector!(4.0, 0.0, 0.0);
        let res = vector!(1.0, 0.0, 0.0);
        assert!(v1.normalize() == res);

        let v1 = vector!(1.0, 2.0, 3.0);
        let res = vector!(0.2672612, 0.5345224, 0.8017837);
        assert!(v1.normalize() == res);

        // Magnitude of a normalized vector
        let v1 = vector!(1.0, 2.0, 3.0);
        let v1 = v1.normalize();
        let res: f64 = 1.0;
        assert_eq!(v1.magnitude(), res);
    }

    #[test]
    fn dot_product() {
        let v1 = vector!(1, 2, 3);
        let v2 = vector!(2, 3, 4);
        let res: f64 = 20.0;
        assert_eq!(v1.dot(&v2), res);
    }

    #[test]
    fn cross_product() {
        let v1 = vector!(1, 2, 3);
        let v2 = vector!(2, 3, 4);
        let res = vector!(-1, 2, -1);

        assert!(v1.cross(&v2) == res);
        // The cross product is not commutative
        let res = vector!(1, -2, 1);
        assert!(v2.cross(&v1) == res);
    }
}
