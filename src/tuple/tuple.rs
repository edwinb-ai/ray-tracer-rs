/// The `Tuple` trait will be used to implement
/// either a Vector (which has `w` value of 0.0)
/// and a Point (which as a `w` value of 1.0).
/// 
/// `get_x`, `get_y`, `get_z` and `get_w` are
/// all "getters" for the types that implement
/// this trait.
pub trait Tuple<T> {
    fn get_x(self) -> T;
    fn get_y(self) -> T;
    fn get_z(self) -> T;
    fn get_w(self) -> T;
}

struct Vector<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T> Tuple<T> for Vector<T> {
    fn get_x(self) -> T {
        self.x
    }

    fn get_y(self) -> T {
        self.y
    }

    fn get_z(self) -> T {
        self.z
    }

    fn get_w(self) -> T {
        self.w
    }
}

struct Point<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T> Tuple<T> for Point<T> {
    fn get_x(self) -> T {
        self.x
    }

    fn get_y(self) -> T {
        self.y
    }

    fn get_z(self) -> T {
        self.z
    }

    fn get_w(self) -> T {
        self.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_vector() {
        let v = Vector::<f64>{x: 4.0, y: 4.0, z: 5.0, w: 1.0};
        assert_eq!(1.0, v.get_w());
    }

    #[test]
    fn test_for_point() {
        let p = Point::<f64>{x: 4.0, y: 4.0, z: 5.0, w: 0.0};
        assert_eq!(0.0, p.get_w());
    }
}