
use image::{RgbaImage, Rgba};

struct Point
{
    x: i32,
    y: i32
}

fn main() {
    const WIDTH: u32 = 100;
    const HEIGHT: u32 = 100;

    let mut image = RgbaImage::new(WIDTH, HEIGHT);
    
    line(&mut image, Point { x: 13, y: 20 }, Point { x: 80, y: 40 }, Rgba([0, 0, 255, 255])); 
    line(&mut image, Point { x: 20, y: 13 }, Point { x: 40, y: 80 }, Rgba([0, 255, 0, 255])); 
    line(&mut image, Point { x: 80, y: 80 }, Point { x: 13, y: 13 }, Rgba([255, 0, 0, 255]));

    image.save("output.png").expect("Failed to save image");
}

fn line(image: &mut RgbaImage, p0: Point, p1: Point, colour: Rgba<u8>) {
    let mut x0 = p0.x;
    let mut y0 = p0.y;
    let mut x1 = p1.x;
    let mut y1 = p1.y;

    let steep = (x0 - x1).abs() < (y0 - y1).abs();

    if steep {
        // Swap x and y
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        // Swap ends
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx: i32 = x1 - x0;
    let dy: i32 = y1 - y0;
    let derr: i32 = 2 * dy.abs();

    let mut err: i32 = 0;
    let mut y: i32 = y0;

    for x in x0..=x1 {
        if steep {
            image.put_pixel(y as u32, x as u32, colour);
        } else {
            image.put_pixel(x as u32, y as u32, colour);
        }

        err += derr;
        if err > dx {
            y += if y1 > y0 { 1 } else { -1 };
            err -= 2 * dx;
        }
    }
}