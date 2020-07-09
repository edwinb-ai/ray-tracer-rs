use crate::tuple::Tuple;
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
}
