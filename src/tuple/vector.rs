use crate::tuple::{Tuple, Point};
use std::ops::{Add, Sub, Neg};
use crate::tuple::utils::float_eq;

pub struct Vector {
    _x: f64,
    _y: f64,
    _z: f64,
    _w: f64,
}

impl Tuple for Vector {
    /// Create a `Vector` centered at `(0, 0, 0)`
    fn origin() -> Self {
        Vector {
            _x: 0.0,
            _y: 0.0,
            _z: 0.0,
            _w: 1.0,
        }
    }

    /// Create a new `Vector` with position `(x, y, z)`
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector {
            _x: x,
            _y: y,
            _z: z,
            _w: 1.0,
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

impl PartialEq for Vector {
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

/// Add two `Vector`s together
impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(
            self._x + other.get_x(),
            self._y + other.get_y(),
            self._z + other.get_z(),
        )
    }
}

/// Add a `Vector` and a `Point`
impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(
            self._x + other.get_x(),
            self._y + other.get_y(),
            self._z + other.get_z(),
        )
    }
}

/// Subtract two `Vector`s.
impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector::new(
            self._x - other.get_x(),
            self._y - other.get_y(),
            self._z - other.get_z(),
        )
    }
}

/// Subtract a `Vector` and a `Point`
impl Sub<Point> for Vector {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(
            self._x - other.get_x(),
            self._y - other.get_y(),
            self._z - other.get_z(),
        )
    }
}

/// Unary negation on a `Vector`
impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new(
            -self._x,
            -self._y,
            -self._z
        )
    }
}

// * Unit tests for `Vector`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_origin_vector() {
        let v = Vector::origin();
        assert_eq!(0.0, v.get_x());
        assert_eq!(0.0, v.get_y());
        assert_eq!(0.0, v.get_z());
        assert_eq!(1.0, v.get_w());
    }

    #[test]
    fn test_new_vector() {
        let v = Vector::new(4.0, -4.0, 3.0);
        assert_eq!(4.0, v.get_x());
        assert_eq!(-4.0, v.get_y());
        assert_eq!(3.0, v.get_z());
        assert_eq!(1.0, v.get_w());
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
}
