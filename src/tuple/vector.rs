use crate::tuple::Tuple;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_origin_vector() {
        let v = Vector::origin();
        assert_eq!(1.0, v.get_w());
        assert_eq!(0.0, v.get_x());
        assert_eq!(0.0, v.get_y());
        assert_eq!(0.0, v.get_z());
    }
}
