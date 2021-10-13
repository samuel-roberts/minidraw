use crate::point::Point;
use image::{Rgba, RgbaImage};

pub struct Renderer {
    width: u32,
    height: u32,
    buffer: RgbaImage,
}

impl Renderer {
    ///
    pub fn new(width: u32, height: u32) -> Renderer {
        Renderer {
            width: width,
            height: height,
            buffer: RgbaImage::new(width, height),
        }
    }

    ///
    pub fn get_width(&self) -> u32 {
        self.width
    }

    ///
    pub fn get_height(&self) -> u32 {
        self.height
    }

    ///
    pub fn save(&self, filepath: &str) -> Result<(), &str> {
        match self.buffer.save(filepath) {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to save image"),
        }
    }

    ///
    pub fn line(&mut self, p0: Point, p1: Point, colour: Rgba<u8>) {
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
            if x < 0 || x as u32 >= self.width || y < 0 || y as u32 >= self.height {
                continue;
            }

            if steep {
                self.buffer.put_pixel(y as u32, x as u32, colour);
            } else {
                self.buffer.put_pixel(x as u32, y as u32, colour);
            }

            err += derr;
            if err > dx {
                y += if y1 > y0 { 1 } else { -1 };
                err -= 2 * dx;
            }
        }
    }
}
