
use image::{RgbaImage, Rgba};

fn main() {
    const WIDTH: u32 = 100;
    const HEIGHT: u32 = 100;

    let mut image = RgbaImage::new(WIDTH, HEIGHT);
    
    line(&mut image, 13, 20, 80, 40, Rgba([255, 255, 255, 255])); 
    line(&mut image, 20, 13, 40, 80, Rgba([255, 0, 0, 255])); 
    line(&mut image, 80, 40, 13, 20, Rgba([255, 0, 0, 255]));

    image.save("output.png").expect("Failed to save image");
}

fn line(image: &mut RgbaImage, x0: u32, y0: u32, x1: u32, y1: u32, colour: Rgba<u8>) {
    for x in x0..=x1 {
        let t: f32 = ((x - x0) as f32) / ((x1 - x0) as f32);
        let y: f32 = ((y0 as f32) * (1.0 - t)) + ((y1 as f32) * t);
        image.put_pixel(x, y as u32, colour);
    }
}