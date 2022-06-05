use crate::tuple::utils::float_eq;
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            blue: self.blue + rhs.blue,
            green: self.green + rhs.green,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            blue: self.blue - rhs.blue,
            green: self.green - rhs.green,
        }
    }
}

// Multiplication of `Color` with `Color`
impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let new_red = self.red * rhs.red;
        let new_green = self.green * rhs.green;
        let new_blue = self.blue * rhs.blue;
        
        Self::new(new_red, new_green, new_blue)
    }
}

// Multiplication of `Color` with scalar
impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let new_red = self.red * rhs;
        let new_green = self.green * rhs;
        let new_blue = self.blue * rhs;
        
        Self::new(new_red, new_green, new_blue)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let xtrue = float_eq(self.red, other.red);
        let ytrue = float_eq(self.blue, other.blue);
        let ztrue = float_eq(self.green, other.green);

        if xtrue && ytrue && ztrue {
            true
        } else {
            false
        }
    }
}

// * Useful macros
#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Color::new($r as f64, $g as f64, $b as f64)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_color() {
        let c = color!(-0.5, 0.4, 1.7);
        assert_eq!(-0.5, c.red);
        assert_eq!(0.4, c.green);
        assert_eq!(1.7, c.blue);
    }

    #[test]
    fn sum_two_colors() {
        let c1 = color!(0.9, 0.6, 0.75);
        let c2 = color!(0.7, 0.1, 0.25);
        let result = color!(1.6, 0.7, 1.0);
        assert_eq!(c1 + c2, result);
    }

    #[test]
    fn subtract_two_colors() {
        let c1 = color!(0.9, 0.6, 0.75);
        let c2 = color!(0.7, 0.1, 0.25);
        let result = color!(0.2, 0.5, 0.5);
        assert_eq!(c1 - c2, result);
    }

    #[test]
    fn multiply_two_colors() {
        let c1 = color!(1.0, 0.2, 0.4);
        let c2 = color!(0.9, 1.0, 0.1);
        let result = color!(0.9, 0.2, 0.04);
        assert_eq!(c1 * c2, result);
    }

    #[test]
    fn multiply_color_and_scalar() {
        let c1 = color!(0.2, 0.3, 0.4);
        let s1 = 2.0;
        let result = color!(0.4, 0.6, 0.8);
        assert_eq!(c1 * s1, result);
    }
}
