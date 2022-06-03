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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_color() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(-0.5, c.get_red());
        assert_eq!(0.4, c.get_green());
        assert_eq!(1.7, c.get_blue());
    }
}
