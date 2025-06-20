use image::{DynamicImage, RgbaImage};

pub fn img_value(original_img: &DynamicImage, img_draw: &RgbaImage) -> u32 {
    let binding = original_img.to_rgba8();
    let pixels_original = binding.as_raw();
    let pixels_draw = img_draw.as_raw();
    
    pixels_original
        .chunks_exact(4)
        .zip(pixels_draw.chunks_exact(4))
        .map(|(orig, draw)| {
            let r = orig[0] as i32 - draw[0] as i32;
            let g = orig[1] as i32 - draw[1] as i32;
            let b = orig[2] as i32 - draw[2] as i32;
            (r.abs() + g.abs() + b.abs()) as u32
        })
        .sum()
}