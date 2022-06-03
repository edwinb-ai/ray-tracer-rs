use crate::tuple::utils::float_eq;
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    _red: f64,
    _green: f64,
    _blue: f64,
}

impl Color {
    fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            _red: r,
            _green: g,
            _blue: b,
        }
    }

    fn get_red(&self) -> f64 {
        self._red
    }

    fn get_green(&self) -> f64 {
        self._green
    }

    fn get_blue(&self) -> f64 {
        self._blue
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            _red: self.get_red() + rhs.get_red(),
            _blue: self.get_blue() + rhs.get_blue(),
            _green: self.get_green() + rhs.get_green(),
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            _red: self.get_red() - rhs.get_red(),
            _blue: self.get_blue() - rhs.get_blue(),
            _green: self.get_green() - rhs.get_green(),
        }
    }
}

// Multiplication of `Color` with `Color`
impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let new_red = self.get_red() * rhs.get_red();
        let new_green = self.get_green() * rhs.get_green();
        let new_blue = self.get_blue() * rhs.get_blue();
        
        Self::new(new_red, new_green, new_blue)
    }
}

// Multiplication of `Color` with scalar
impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let new_red = self.get_red() * rhs;
        let new_green = self.get_green() * rhs;
        let new_blue = self.get_blue() * rhs;
        
        Self::new(new_red, new_green, new_blue)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let xtrue = float_eq(self.get_red(), other.get_red());
        let ytrue = float_eq(self.get_blue(), other.get_blue());
        let ztrue = float_eq(self.get_green(), other.get_green());

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
        assert_eq!(-0.5, c.get_red());
        assert_eq!(0.4, c.get_green());
        assert_eq!(1.7, c.get_blue());
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
