use image::DynamicImage;
use crate::random_shape::RandomCircle;

pub struct Comparison {
    original_img: DynamicImage,
    img_draw: RandomCircle,
    img_dimensions: (u32, u32),
}

impl Comparison {
    pub fn new(original_img: DynamicImage, previous_draw: DynamicImage, img_dimensions: (u32, u32)) -> Self {
        let mut new_draw = RandomCircle::new(previous_draw);
        new_draw.draw_shape();

        Self {
            original_img,
            img_draw: new_draw,
            img_dimensions
        }
    }

    pub fn img_value(&self) -> u32 {
        let mut red_value = 255;
        let mut green_value = 255;
        let mut blue_value = 255;
        let binding = self.original_img.to_rgba8();
        let pixels_original_img = binding.as_raw();
        let pixels_draw_img = self.img_draw.get_img().as_raw();
        let mut value = 0;

        for i in 0..self.img_dimensions.0 * self.img_dimensions.1 {
            let idx = (i * 4) as usize;
            let value_red = pixels_original_img[idx] as i32 - pixels_draw_img[idx] as i32;
            let value_green = pixels_original_img[idx + 1] as i32 - pixels_draw_img[idx + 1] as i32;
            let value_blue = pixels_original_img[idx + 2] as i32 - pixels_draw_img[idx + 2] as i32;

            value += (value_red.abs() + value_green.abs() + value_blue.abs()) as u32;
        }
        value
    }
}