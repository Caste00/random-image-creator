use image::{DynamicImage, Rgba, RgbaImage};
use crate::pixels_comparison::Comparison;

fn starting_image(dimension: (u32, u32)) {
    let bianco = Rgba([255, 255, 255, 255]);
    let mut blank_img = RgbaImage::from_pixel(dimension.0, dimension.1, bianco);
    blank_img.save("start_img.png").expect("Error: I could not create a white image");
}

fn generating_image(original_img: RgbaImage, iteration: u32) {
    
    
}
