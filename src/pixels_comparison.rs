use image::{DynamicImage, RgbaImage};

pub fn img_value(original_img: &DynamicImage, img_draw: &RgbaImage) -> u32 {
    let binding = original_img.to_rgba8();
    let dimension = original_img.width() * original_img.height();
    let pixels_original_img = binding.as_raw();
    let pixels_draw_img = img_draw.as_raw();
    let mut value = 0;

    for i in 0..dimension {
        let idx = (i * 4) as usize;
        let value_red = pixels_original_img[idx] as i32 - pixels_draw_img[idx] as i32;
        let value_green = pixels_original_img[idx + 1] as i32 - pixels_draw_img[idx + 1] as i32;
        let value_blue = pixels_original_img[idx + 2] as i32 - pixels_draw_img[idx + 2] as i32;

        value += (value_red.abs() + value_green.abs() + value_blue.abs()) as u32;
    }
    value
}