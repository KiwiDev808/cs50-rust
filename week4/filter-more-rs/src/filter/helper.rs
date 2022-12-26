use std::cmp;

use crate::RGBTriple;

const GX_MATRIX: [i32; 9] = [-1, 0, 1, -2, 0, 2, -1, 0, 1];
const GY_MATRIX: [i32; 9] = [-1, -2, -1, 0, 0, 0, 1, 2, 1];

pub struct BlurPixelBox {
    red_sum: u16,
    green_sum: u16,
    blue_sum: u16,
    pixel_count: u16,
}

impl BlurPixelBox {
    pub fn new() -> BlurPixelBox {
        Self {
            red_sum: 0,
            green_sum: 0,
            blue_sum: 0,
            pixel_count: 0,
        }
    }
    pub fn add_pixel(&mut self, pixel: &RGBTriple) {
        self.red_sum += pixel.rgbt_red as u16;
        self.green_sum += pixel.rgbt_green as u16;
        self.blue_sum += pixel.rgbt_blue as u16;
        self.pixel_count += 1;
    }
    pub fn get_blur_values(&self) -> (u8, u8, u8) {
        let red_blur = (self.red_sum / self.pixel_count) as u8;
        let green_blur = (self.green_sum / self.pixel_count) as u8;
        let blue_blur = (self.blue_sum / self.pixel_count) as u8;
        (red_blur, green_blur, blue_blur)
    }
}

pub struct EdgePixelBox {
    red_matrix: (i32, i32),
    green_matrix: (i32, i32),
    blue_matrix: (i32, i32),
}

impl EdgePixelBox {
    pub fn new() -> EdgePixelBox {
        Self {
            red_matrix: (0, 0),
            green_matrix: (0, 0),
            blue_matrix: (0, 0),
        }
    }

    pub fn add_pixel(&mut self, pixel: &RGBTriple, matrix_index: usize) {
        self.red_matrix.0 += pixel.rgbt_red as i32 * GX_MATRIX[matrix_index];
        self.red_matrix.1 += pixel.rgbt_red as i32 * GY_MATRIX[matrix_index];
        self.green_matrix.0 += pixel.rgbt_green as i32 * GX_MATRIX[matrix_index];
        self.green_matrix.1 += pixel.rgbt_green as i32 * GY_MATRIX[matrix_index];
        self.blue_matrix.0 += pixel.rgbt_blue as i32 * GX_MATRIX[matrix_index];
        self.blue_matrix.1 += pixel.rgbt_blue as i32 * GY_MATRIX[matrix_index];
    }

    fn calculate_matrix_value(&self, matrix: (i32, i32)) -> isize {
        (((matrix.0).pow(2) as f64) + ((matrix.1).pow(2)) as f64).sqrt() as isize
    }

    pub fn get_edge_values(&self) -> (u8, u8, u8) {
        let mut red_edge = self.calculate_matrix_value(self.red_matrix);
        red_edge = cmp::min(red_edge, 255);

        let mut green_edge = self.calculate_matrix_value(self.green_matrix);
        green_edge = cmp::min(green_edge, 255);

        let mut blue_edge = self.calculate_matrix_value(self.blue_matrix);
        blue_edge = cmp::min(blue_edge, 255);

        (red_edge as u8, green_edge as u8, blue_edge as u8)
    }
}
