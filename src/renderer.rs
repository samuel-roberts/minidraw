use image::{ImageBuffer, Luma, Pixel, Rgba, RgbaImage};
use nalgebra::{Point2, Point3, Vector2, Vector3};
use std::cmp;

use crate::utilities;

type DepthImage = ImageBuffer<Luma<f32>, Vec<f32>>;

pub struct Renderer {
    width: u32,
    height: u32,
    colour_buffer: RgbaImage,
    depth_buffer: DepthImage,
}

impl Renderer {
    ///
    pub fn new(width: u32, height: u32) -> Renderer {
        Renderer {
            width: width,
            height: height,
            colour_buffer: RgbaImage::new(width, height),
            depth_buffer: DepthImage::from_pixel(width, height, Luma([-1.0])),
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
        match self.colour_buffer.save(filepath) {
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
                self.colour_buffer.put_pixel(y as u32, x as u32, colour);
            } else {
                self.colour_buffer.put_pixel(x as u32, y as u32, colour);
            }

            err += derr;
            if err > dx {
                y += if y1 > y0 { 1 } else { -1 };
                err -= 2 * dx;
            }
        }
    }

    ///
    pub fn triangle(
        &mut self,
        p0: Point3<f32>,
        p1: Point3<f32>,
        p2: Point3<f32>,
        colour: Rgba<u8>,
    ) {
        // Get triangle normal
        let normal = (p2 - p0).cross(&(p1 - p0)).normalize();

        // Convert to screen-space
        let p0 = self.to_screen(p0);
        let p1 = self.to_screen(p1);
        let p2 = self.to_screen(p2);

        // Find the screen-space bounding box of this triangle
        let mut bb_min = Vector2::<i32>::new((self.width - 1) as i32, (self.height - 1) as i32);
        let mut bb_max = Vector2::<i32>::new(0, 0);
        let clamp = Vector2::<i32>::new(bb_min.x, bb_min.y);

        for p in &[p0, p1, p2] {
            bb_min.x = cmp::max(0, cmp::min(bb_min.x, p.x as i32));
            bb_min.y = cmp::max(0, cmp::min(bb_min.y, p.y as i32));
            bb_max.x = cmp::min(clamp.x, cmp::max(bb_max.x, p.x as i32));
            bb_max.y = cmp::min(clamp.y, cmp::max(bb_max.y, p.y as i32));
        }

        // Calculate facet lighting
        let light_direction = Vector3::<f32>::new(0.0, 0.0, -1.0);
        let lighting_intensity = normal.dot(&light_direction);
        if lighting_intensity < 0.0 {
            // TODO Replace this check with one for the camera direction (detect back faces)
            return;
        }

        let colour = colour.map_with_alpha(
            |c| utilities::clamp_f32((c as f32) * lighting_intensity, 0.0, 255.0) as u8,
            |a| a,
        );

        // Render
        for x in bb_min.x..=bb_max.x {
            for y in bb_min.y..=bb_max.y {
                // Find the barycentric coordinates of this pixel
                let p = Point3::<f32>::new(x as f32, y as f32, 0.0);
                if let Some(b) = utilities::barycentric(p0, p1, p2, p) {
                    if (b.x > 0.0) && (b.y > 0.0) && (b.z > 0.0) && (lighting_intensity > 0.0) {
                        // Calculate the depth
                        let u = x as u32;
                        let v = y as u32;
                        let depth = utilities::sigmoid((p0.z * b.x) + (p1.z * b.y) + (p2.z * b.z));

                        if depth > self.depth_buffer.get_pixel(u, v)[0] {
                            self.colour_buffer.put_pixel(u, v, colour);
                            self.depth_buffer.put_pixel(u, v, Luma([depth]));
                        }
                    }
                }
            }
        }
    }

    /// Convert from World to Screen coordinate system
    fn to_screen(&self, p: Point3<f32>) -> Point3<f32> {
        Point3::<f32>::new(
            (p.x + 1.0) * ((self.get_width() as f32) / 2.0),
            self.get_height() as f32 - ((p.y + 1.0) * ((self.get_height() as f32) / 2.0)),
            p.z,
        )
    }
}
