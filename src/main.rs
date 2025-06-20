mod random_shape;
mod pixels_comparison;
mod image_generator;

use std::io;
use std::io::Write;
use crate::image_generator::{generating_image, video_from_generating_img};

fn main() {
    let mut img_template_path = String::new();

    print!("path to template: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut img_template_path).expect("Failed to read input");

    generating_image(&img_template_path.trim(), 5000, 5000);
    video_from_generating_img(60, 5000);
}
