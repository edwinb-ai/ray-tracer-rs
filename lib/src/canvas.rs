use crate::color; // for the macro
use crate::color::Color; // for the type

const MAX_COLOR_VALUE: usize = 255;

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

    /// Return the value of the `Canvas` at position (`w`, `h`)
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
    pub fn pixel_at(&self, w: usize, h: usize) -> Color {
        self.data[h][w]
    }

    /// Write to the data the value of the pixel color given by `c`,
    /// at position (`h`, `w`). This function overwrites the original value
    /// in the `data` field of the `Canvas` type.
    ///
    /// # Examples
    /// ```
    /// use ray_tracer::canvas::Canvas;
    /// use ray_tracer::color::Color;
    /// use ray_tracer::color;
    ///
    /// let mut canvas1 = Canvas::new(10, 20);
    /// let new_color = color!(1.0, 0.0, 0.0);
    /// canvas1.write_pixel(1, 1, color!(1.0, 0.0, 0.0));
    /// assert!(canvas1.pixel_at(1, 1) == new_color);
    /// ```
    pub fn write_pixel(&mut self, w: usize, h: usize, c: Color) {
        if h <= self.height && w <= self.width {
            self.data[h][w] = c;
        }
    }

    pub fn canvas_to_ppm(&self) -> String {
        let mut ppm_string = String::new();
        // Create the first three lines, the header
        ppm_string.push_str("P3\n");
        // Add the size of canvas to the header
        let size_header = format!("{} {}\n", self.width, self.height);
        ppm_string.push_str(size_header.as_str());
        // Add the maximum value that the colors can take
        ppm_string.push_str(format!("{}\n", MAX_COLOR_VALUE).as_str());

        ppm_string
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

    #[test]
    fn read_pixel() {
        let canvas1 = Canvas::new(10, 20);
        assert!(canvas1.pixel_at(1, 2) == color!(0.0, 0.0, 0.0));
    }

    #[test]
    fn change_value_of_pixel() {
        let mut canvas1 = Canvas::new(10, 20);
        let new_color = color!(1.0, 0.0, 0.0);
        canvas1.write_pixel(1, 1, color!(1.0, 0.0, 0.0));
        assert!(canvas1.pixel_at(1, 1) == new_color);
    }

    #[test]
    fn write_to_ppm() {
        let canvas1 = Canvas::new(5, 3);
        let ppm_string = canvas1.canvas_to_ppm();
        println!("{}", ppm_string);
    }
}
