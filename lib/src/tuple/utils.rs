//! Miscelaneous utility functions

/// comparison constant
pub const EPSILON: f64 = 0.000001;

/// returns if both numbers are equal to an arbitrary number
/// called epsilon
///
/// # Arguments
/// * `a` - a float
/// * `b` - a float
///
/// # Examples
/// ```
/// use ray_tracer::tuple::utils;
/// let f = utils::float_eq(0.00001, 0.00002);
/// ```
pub fn float_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
