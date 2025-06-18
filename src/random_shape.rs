use rand::prelude::*;
use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, Canvas};

pub enum Shape {
    Circle,
}

pub struct RandomCircle {
    original_img: DynamicImage,
    pub img_draw: RgbaImage,
    color: image::Rgba<u8>,
    position: (i32, i32),
}

impl RandomCircle {
    pub fn new(original_img: DynamicImage) -> Self {
        let (img_width, img_height) = original_img.dimensions();
        let img_draw = original_img.clone().into_rgba8();

        Self {
            original_img,
            img_draw,
            color: Self::random_color(),
            position: Self::random_position(img_width, img_height)
        }
    }

    fn random_color() -> Rgba<u8> {
        let mut rng = rand::rng();
        let r = rng.random_range(0..=255);
        let g = rng.random_range(0..=255);
        let b = rng.random_range(0..=255);
        Rgba([r, g, b, 255])
    }

    fn random_position(img_width: u32, img_height: u32) -> (i32, i32){
        let mut rng = rand::rng();
        let x = rng.random_range(0..=img_width) as i32;
        let y = rng.random_range(0..=img_height) as i32;
        (x, y)
    }

    fn draw_shape(&mut self) {
        let mut rng = rand::rng();
        let radius = rng.random_range(0..=250);

        draw_filled_circle_mut(&mut self.img_draw, self.position, radius, self.color);
    }
}

