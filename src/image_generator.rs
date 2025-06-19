use image::{DynamicImage, Rgba, RgbaImage};
use crate::random_shape::RandomCircle;
use crate::pixels_comparison::img_value;

fn starting_image(dimension: (u32, u32)) {
    let bianco = Rgba([255, 255, 255, 255]);
    let blank_img = RgbaImage::from_pixel(dimension.0, dimension.1, bianco);
    blank_img.save("start_img.png").expect("Error: I could not create a white image");
}

fn generating_one_image(original_img: &DynamicImage, img_draw: &RgbaImage, iteration: u32) -> RandomCircle {
    let mut best_fit_image: RandomCircle = RandomCircle::new(img_draw);
    let mut best_value = u32::MAX;

    for _ in 0..iteration {
        let mut random_img = RandomCircle::new(img_draw);
        random_img.draw_shape();
        let value = img_value(original_img, random_img.get_img());
        if value < best_value {
            best_fit_image = random_img;
            best_value = value;
        }
    }

    best_fit_image
}

pub fn generating_image(path_original_img: &str, iteration_for_one_img: u32, iteration_img: u32) {
    let original_img = image::open(path_original_img).expect("Error: I could not open the image");
    let dimension = (original_img.width(), original_img.height());
    starting_image(dimension);
    let mut canvas = image::open("start_img.png").expect("Error: I could not create a canvas").to_rgba8();

    for _ in 0..iteration_img {
        let best_image = generating_one_image(&original_img, &canvas, iteration_for_one_img);
        canvas = best_image.get_img().clone();
    }

    canvas.save("final_image.png").expect("Error: I could not save the final image");
    println!("Final image created!");
}