use bear_lib_terminal::terminal::state;
use image::{ImageReader, Pixel, RgbImage, imageops};
use std::io::Cursor;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let size = state::cell_size();
    // println!("Cell size: {}x{} pixels", size.width, size.height);
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics
    let img_path = "C:\\Users\\ChristophS\\Desktop\\Surprised_Pikachu.jpg"; // Replace with your file path
    let img: RgbImage = ImageReader::open(img_path)?.decode()?.to_rgb8();

    let (width, height) = img.dimensions();
    println!("Image dimensions: {}x{}", width, height);

    let nw = 80;
    let nh = height / (width / nw);

    // Scale the image (e.g., to 300x300; use image::imageops::FilterType::Lanczos3 for quality)
    let img = imageops::resize(&img, nw, nh, imageops::FilterType::Lanczos3);

    let (width, height) = img.dimensions();
    println!("Image dimensions: {}x{}", width, height);

    let mut total_brightness: f32 = 0.0;
    let mut pixel_count = 0;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness = pixel.to_luma()[0] as f32 / 255.0; // Normalize to 0.0-1.0
            let c = if brightness < 0.5 { ',' } else { '#' };
            print!("{c}");
        }
        println!();
    }

    //  Iterate over all pixels
    // for (x, y, pixel) in img.enumerate_pixels() {
    //     // Compute luminance (brightness) using the standard formula
    //     let brightness = pixel.to_luma()[0] as f32 / 255.0; // Normalize to 0.0-1.0

    //     // Example: Print brightness for each pixel (or process it)
    //     // println!("Pixel at ({}, {}): brightness = {:.2}", x, y, brightness);
    //     //
    //     let c = if brightness < 0.5 { ',' } else { '#' };

    //     total_brightness += brightness;
    //     pixel_count += 1;
    // }

    // // Example analysis: Average brightness
    // let avg_brightness = total_brightness / pixel_count as f32;
    // println!("Average image brightness: {:.2}", avg_brightness);

    Ok(())
}
