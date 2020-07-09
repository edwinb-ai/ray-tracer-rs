pub mod point;
pub mod vector;
pub use point::Point;
pub use vector::Vector;

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
