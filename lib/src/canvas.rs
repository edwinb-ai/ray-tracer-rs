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

    /// Return the value of the `Canvas` at position (w, h)
    ///
    /// # Examples
    /// ```
    /// use ray_tracer::canvas::Canvas;
    /// use ray_tracer::color::Color;
    /// use ray_tracer::color;
    /// 
    /// let canvas1 = Canvas::new(10, 20);
    /// assert!(canvas1.pixel_at(1, 1) == color!(0.0, 0.0, 0.0))
    /// ```
    pub fn pixel_at(self, w: usize, h: usize) -> Color {
        self.data[h][w]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_canvas() {
        let canvas1 = Canvas::new(10, 20);
        // First, check for the size
        assert_eq!(canvas1.width, 10);
        assert_eq!(canvas1.height, 20);
        // Now, check that all new pixels are black
        let black = color!(0.0, 0.0, 0.0);
        for x in canvas1.data {
            for y in x {
                assert!(y == black)
            }
        }
    }
}
