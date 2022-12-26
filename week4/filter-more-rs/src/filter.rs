use self::bmp::RGBTriple;
use self::helper::{BlurPixelBox, EdgePixelBox};
pub mod bmp;
mod helper;

// Convert image to grayscale
pub fn grayscale(image: &mut [RGBTriple]) {
    for pixel in image.iter_mut() {
        let gray_shade = pixel.get_average();
        *pixel = RGBTriple {
            rgbt_blue: gray_shade,
            rgbt_green: gray_shade,
            rgbt_red: gray_shade,
        };
    }
}

// Reflect image horizontally
pub fn reflect(width: usize, image: &mut [RGBTriple]) {
    for image_row in image.chunks_mut(width) {
        for pixel_index in 0..(width / 2) {
            let right_pixel = width - (pixel_index + 1);
            let left_pixel = pixel_index;
            image_row.swap(left_pixel, right_pixel);
        }
    }
}

// Blur image
pub fn blur(height: usize, width: usize, image: &mut Vec<RGBTriple>) {
    let limit = ((height * width) - 1) as isize;
    let cloned_image = image.clone();
    let box_positions = [-1, 0, 1, -1, 0, 1, -1, 0, 1];
    let row_position_values = [-1, 0, 1];

    for (pixel_index, pixel) in image.iter_mut().enumerate() {
        let mut blur_pixel_box = BlurPixelBox::new();

        for (column_matrix_position, column_matrix) in box_positions.chunks(3).enumerate() {
            let row_position = pixel_index as isize
                + (row_position_values[column_matrix_position] * (width as isize));

            if row_position.is_negative() || row_position > limit {
                continue;
            }

            for column_position in column_matrix {
                let box_pixel_index = row_position + column_position;
                if box_pixel_index.is_negative() || box_pixel_index > limit {
                    continue;
                }
                let box_pixel = cloned_image.get(box_pixel_index as usize).unwrap();
                blur_pixel_box.add_pixel(box_pixel);
            }
        }

        let (red_blur, green_blur, blue_blur) = blur_pixel_box.get_blur_values();
        pixel.change_color(red_blur, green_blur, blue_blur);
    }
}

// Detect edges
pub fn edges(height: usize, width: usize, image: &mut Vec<RGBTriple>) {
    let limit = ((height * width) - 1) as isize;
    let cloned_image = image.clone();

    let box_positions = [-1, 0, 1, -1, 0, 1, -1, 0, 1];
    let row_position_values = [-1, 0, 1];

    for (pixel_index, pixel) in image.iter_mut().enumerate() {
        let mut edge_pixel_box = EdgePixelBox::new();

        for (column_matrix_position, column_matrix) in box_positions.chunks(3).enumerate() {
            let y_level = pixel_index as isize
                + (row_position_values[column_matrix_position] * (width as isize));

            if y_level.is_negative() || y_level > limit {
                continue;
            }

            for (column_position_index, column_position) in column_matrix.iter().enumerate() {
                let box_pixel_index = y_level + column_position;
                let g_matrix_index = column_matrix_position * 3 + column_position_index as usize;
                if box_pixel_index.is_negative() || box_pixel_index > limit {
                    continue;
                }
                let box_pixel = cloned_image.get(box_pixel_index as usize).expect("Error");
                edge_pixel_box.add_pixel(box_pixel, g_matrix_index);
            }
        }
        let (red_edge, green_edge, blue_edge) = edge_pixel_box.get_edge_values();

        pixel.change_color(red_edge, green_edge, blue_edge);
    }
}
