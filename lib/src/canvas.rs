use crate::color; // for the macro
use crate::color::Color; // for the type

const MAX_COLOR_VALUE: usize = 255;

#[derive(Clone, Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<Color>>,
}

fn scale_color(c: f64) -> usize {
    let float_max_value = MAX_COLOR_VALUE as f64;

    ((c * float_max_value)
        .floor()
        .min(float_max_value as f64)
        .max(0.0)) as usize
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width: width,
            height: height,
            // Fill the data with black "pixels"
            data: vec![vec![color!(0.0, 0.0, 0.0); width]; height],
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

        // TODO: Make it so that it can print the three channels column-wise
        for col in 0..self.width {
            for row in 0..self.height {
                let current_color = self.pixel_at(col, row);
                let new_red = scale_color(current_color.red);
                let new_green = scale_color(current_color.green);
                let new_blue = scale_color(current_color.blue);
                // Write the colors to the PPM string we have so far
                ppm_string.push_str(format!("{} {} {}\n", new_red, new_green, new_blue).as_str());
                println!("{}", ppm_string);
            }
        }

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
    fn header_ppm() {
        let c = Canvas::new(5, 3);
        let ppm = c.canvas_to_ppm();
        let mut lines = ppm.lines();
        assert_eq!(lines.next().unwrap(), "P3");
        assert_eq!(lines.next().unwrap(), "5 3");
        assert_eq!(lines.next().unwrap(), "255");
    }

    #[test]
    fn test_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, color!(1.5, 0, 0));
        c.write_pixel(2, 1, color!(0, 0.5, 0));
        c.write_pixel(4, 2, color!(-0.5, 0, 1));

        let ppm = c.canvas_to_ppm();
        let mut lines = ppm.lines();
        // Ignore the header
        lines.next();
        lines.next();
        lines.next();

        assert_eq!(lines.next().unwrap(), "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines.next().unwrap(), "0 0 0 0 0 0 0 127 0 0 0 0 0 0 0");
        assert_eq!(lines.next().unwrap(), "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }
}
