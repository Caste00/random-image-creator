use std::io;
use std::io::Write;
use std::process::Command;
use rayon::prelude::*;
use image::{DynamicImage, Rgba, RgbaImage};
use crate::random_shape::RandomCircle;
use crate::pixels_comparison::img_value;

fn starting_image(dimension: (u32, u32)) {
    let bianco = Rgba([255, 255, 255, 255]);
    let blank_img = RgbaImage::from_pixel(dimension.0, dimension.1, bianco);
    blank_img.save("start_img.png").expect("Error: I could not create a white image");
}

fn generating_one_image(original_img: &DynamicImage, img_draw: &RgbaImage, iteration: u32, number_frame: u32) -> RandomCircle {
    let best = (0..iteration)
        .into_par_iter()
        .map(|_| {
            let mut random_img = RandomCircle::new(img_draw);
            random_img.draw_shape();
            let value = img_value(original_img, random_img.get_img());
            (random_img, value)
        })
        .min_by_key(|(_, value)| *value)
        .expect("Error: al least one shape should be generated");

    let (best_fit_image, _) = best;
    best_fit_image.save_image(&format!("frames/frame{}.png", number_frame));
    best_fit_image
}

pub fn generating_image(path_original_img: &str, iteration_for_one_img: u32, iteration_img: u32) {
    let original_img = image::open(path_original_img).expect("Error: I could not open the image");
    let dimension = (original_img.width(), original_img.height());
    starting_image(dimension);
    let mut canvas = image::open("start_img.png").expect("Error: I could not create a canvas").to_rgba8();

    for i in 0..iteration_img {
        let best_image = generating_one_image(&original_img, &canvas, iteration_for_one_img, i as u32);
        print_percentage(iteration_img, i);
        canvas = best_image.get_img().clone();
    }

    std::fs::remove_file("start_img.png").expect("Error: I could not remove the start image");
    canvas.save("final_image.png").expect("Error: I could not save the final image");

    println!("Image generated successfully!");
}

fn print_percentage(final_frame: u32, current_frame: u32) {
    let value = 100f32 * current_frame as f32 / final_frame as f32;
    print!("\rpercentage: {:.2}%", value);
    io::stdout().flush().unwrap();
}

fn delete_frame(number_of_frames: u32) {
    for i in 0..number_of_frames {
        let _ = std::fs::remove_file(format!("frames/frame{}.png", i));
    }
}

pub fn video_from_generating_img(frame_rate: u32, number_of_frame: u32) {
    let status = Command::new("ffmpeg")
        .args([
           "-framerate", &frame_rate.to_string(),
            "-i", "frames/frame%d.png",
            "-c:v", "libx264",
            "-pix_fmt", "yuv420p",
            "-vf", "scale=trunc(iw/2)*2:trunc(ih/2)*2",
            "video_generation_image.mp4"
        ])
        .status();
    
    println!("Video generated!");
    delete_frame(number_of_frame);
}