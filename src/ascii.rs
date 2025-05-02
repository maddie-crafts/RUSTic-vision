use image;

const ASCII_VALUES: &[u8] = "@%#*+=-:. ".as_bytes();

fn image_average(tile: image::GrayImage) -> u32 {
    let pixels = tile.pixels();
    let mut total: u128 = 0;
    let pixel_count = pixels.count() as u128;
    for pixel in tile.pixels() {
        total += pixel.0[0] as u128;
    }
    (total / pixel_count) as u32
}

pub fn convert(img: image::DynamicImage, columns: u32, scale: f32) -> Vec<String> {
    let mut grayscale = img.to_luma8();
    let width = grayscale.dimensions().0;
    let height = grayscale.dimensions().1;
    let tile_width = width / columns;
    let tile_height = tile_width as f32 / scale;

    let rows = (height as f32 / tile_height) as u32;
    let mut ascii_image: Vec<String> = Vec::new();

    for i in 0..rows {
        let y = i * tile_height as u32;
        let mut y_offset = tile_height as u32;

        if i == rows - 1 {
            y_offset = height - y;
        }

        ascii_image.push(String::from(""));

        for j in 0..columns {
            let x: u32 = j * tile_width;
            let mut x_offset = tile_width;

            if j == columns - 1 {
                x_offset = width - x;
            }
            let cropped_section = image::imageops::crop(&mut grayscale, x, y, x_offset, y_offset);
            let avg_pixel = image_average(cropped_section.to_image());
            let idx = ((avg_pixel * 9) / 255) as usize;
            let ascii_val = ASCII_VALUES[idx] as char;
            ascii_image[i as usize].push(ascii_val);
        }

    }
    ascii_image
}