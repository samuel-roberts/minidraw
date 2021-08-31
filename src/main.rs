
use image::{RgbImage, Rgb};

fn main() {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;

    let mut image = RgbImage::new(WIDTH, HEIGHT);
    image.put_pixel(10, 10, Rgb([255, 0, 0]));

    image.save("output.png").expect("Failed to save image");
}
