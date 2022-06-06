use crate::color; // for the macro
use crate::color::Color; // for the type

#[derive(Clone, Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width: width,
            height: height,
            // Fill the data with black "pixels"
            data: vec![vec![color!(0.0, 0.0, 0.0); height]; width],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_canvas() {
        let canvas1 = Canvas::new(10, 10);
        assert_eq!(canvas1.width, 10);
    }
}
