use crate::tuple::Tuple;
use crate::tuple::utils::float_eq;

pub struct Point {
    _x: f64,
    _y: f64,
    _z: f64,
    _w: f64,
}

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
    fn eq(&self, other: &Self) -> bool {
        let xtrue = float_eq(self._x, other.get_x());
        let ytrue = float_eq(self._y, other.get_y());
        let ztrue = float_eq(self._z, other.get_z());
        let wtrue = float_eq(self._w, other.get_w());

        // NOTE: I am skipping over floating point
        // comparison, and trusting on the compiler for this
        if xtrue && ytrue && ztrue && wtrue {
            true
        } else {
            false
        }
    }
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

    // #[test]
    // fn test_addition() {
    //     let p1 = Point::new(x: f64, y: f64, z: f64)
    // }
}
