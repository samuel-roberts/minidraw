
use std::cmp;
use nalgebra::{Point2, Vector2, Vector3};
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
    pub fn line(&mut self, p0: Point2<i32>, p1: Point2<i32>, colour: Rgba<u8>) {
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

    ///
    pub fn triangle(&mut self, p0: Point2<i32>, p1: Point2<i32>, p2: Point2<i32>, colour: Rgba<u8>) {
        let mut bb_min = Vector2::<i32>::new((self.width - 1) as i32, (self.height - 1) as i32);
        let mut bb_max = Vector2::<i32>::new(0, 0);
        let clamp = Vector2::<i32>::new(bb_min.x, bb_min.y);

        for p in &[p0, p1, p2] {
            bb_min.x = cmp::max(0, cmp::min(bb_min.x, p.x));
            bb_min.y = cmp::max(0, cmp::min(bb_min.y, p.y));
            bb_max.x = cmp::min(clamp.x, cmp::max(bb_max.x, p.x));
            bb_max.y = cmp::min(clamp.y, cmp::max(bb_max.y, p.y));
        }

        for x in bb_min.x..=bb_max.x {
            for y in bb_min.y..=bb_max.y {
                let p = Point2::<i32>::new(x, y);
                if let Some(b) = Renderer::barycentric(p0, p1, p2, p) {
                    if (b.x > 0.0) && (b.y > 0.0) && (b.z > 0.0) {
                        self.buffer.put_pixel(x as u32, y as u32, colour);
                    }
                }
            }
        }
    }

    ///
    fn barycentric(p0: Point2<i32>, p1: Point2<i32>, p2: Point2<i32>, p: Point2<i32>) -> Option<Vector3<f32>> {
        let a = Vector3::<f32>::new((p2.x - p0.x) as f32, (p1.x - p0.x) as f32, (p0.x - p.x) as f32);
        let b = Vector3::<f32>::new((p2.y - p0.y) as f32, (p1.y - p0.y) as f32, (p0.y - p.y) as f32);
        let u = a.cross(&b);

        if u.z.abs() < 1.0 {
            None
        } else {
            let v = Vector3::<f32>::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
            Some(v)
        }
    }
}
