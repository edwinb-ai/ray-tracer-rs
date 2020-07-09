/// The `Tuple` trait will be used to implement
/// either a Vector (which has `w` value of 0.0)
/// and a Point (which as a `w` value of 1.0).
/// 
/// `get_x`, `get_y`, `get_z` and `get_w` are
/// all "getters" for the types that implement
/// this trait.
pub trait Tuple {
    fn origin () -> Self;
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_z(&self) -> f64;
    fn get_w(&self) -> f64;
}

struct Vector {
    _x: f64,
    _y: f64,
    _z: f64,
    _w: f64,
}

impl Tuple for Vector {
    fn origin() -> Self {
        Vector{ _x: 0.0, _y: 0.0, _z: 0.0, _w: 1.0}
    }

    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector{ _x: x, _y: y, _z: z, _w: 1.0}
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

struct Point {
    _x: f64,
    _y: f64,
    _z: f64,
    _w: f64
}

impl Tuple for Point {
    fn origin() -> Self {
        Point{ _x: 0.0, _y: 0.0, _z: 0.0, _w: 0.0}
    }

    fn new(x: f64, y: f64, z: f64) -> Self {
        Point{ _x: x, _y: y, _z: z, _w: 0.0}
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

    #[test]
    fn test_for_point() {
        let p = Point::origin();
        assert_eq!(0.0, p.get_w());
        assert_eq!(0.0, p.get_x());
        assert_eq!(0.0, p.get_y());
        assert_eq!(0.0, p.get_z());
    }
}