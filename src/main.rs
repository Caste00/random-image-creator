mod random_shape;
mod pixels_comparison;
mod image_generator;

use std::io;
use std::io::Write;
use crate::image_generator::{generating_first_best_image, generating_image, video_from_generating_img};

fn main() {
    let mut img_template_path = String::new();
    let mut iterations = String::new();

    print!("path to template: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut img_template_path).expect("Failed to read input ");

    print!("Number of iterations: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut iterations).expect("Failed to read input ");

    let number_of_iterations = iterations.trim().parse::<u32>().expect("Invalid number");

    // genera un'immagine ma gli si deve dare il numero di iterazioni per ogni immagine e il numero di frame (usa il multithread sulle immagini
    let number_of_frames = generating_image(&img_template_path.trim(), 5000, number_of_iterations);
    println!("number of frames: {}", number_of_frames);
    video_from_generating_img(60, number_of_frames);

    // genera un'immagine prendendo la prima imamgine migliore, termina quando non riesce a migliorarla generando n immagini, non è multithread
    // la generazione è peggiore, dura molto meno
    //let frame = generating_first_best_image(&img_template_path.trim(), number_of_iterations);
    //video_from_generating_img(60, frame);
}
