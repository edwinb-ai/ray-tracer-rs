// Exports
pub mod point;
pub mod utils;
pub mod vector;

// Imports
pub use point::Point;
use std::cmp::PartialEq;
pub use vector::Vector;

/// The `Tuple` trait is used to implement
/// either a `Vector` (which has `w` value of 0.0)
/// and a `Point` (which as a `w` value of 1.0).
///
/// `get_x`, `get_y`, `get_z` and `get_w` are
/// all "getters" for the types that implement
/// this trait.
pub trait Tuple: PartialEq {
    fn origin() -> Self;
    fn new(x: f64, y: f64, z: f64) -> Self;
}
