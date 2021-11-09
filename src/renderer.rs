use image::{ImageBuffer, Luma, Pixel, Rgba, RgbaImage};
use nalgebra::{Matrix4, Point3, Vector2, Vector3};
use std::cmp;

use crate::{camera::Camera, drawable::Drawable, renderer_config::RendererConfig, utilities};

type DepthImage = ImageBuffer<Luma<f32>, Vec<f32>>;

pub struct Renderer {
    width: u32,
    height: u32,
    config: RendererConfig,
    colour_buffer: RgbaImage,
    depth_buffer: DepthImage,
    camera: Camera,
}

impl Renderer {
    ///
    pub fn new(width: u32, height: u32, config: RendererConfig) -> Renderer {
        let camera = Camera::new(
            (width as f32) / (height as f32),
            config.field_of_view,
            0.1,
            10000.0,
        );

        Renderer {
            width: width,
            height: height,
            config: config,
            colour_buffer: RgbaImage::new(width, height),
            depth_buffer: DepthImage::from_pixel(width, height, Luma([f32::NEG_INFINITY])),
            camera: camera,
        }
    }

    ///
    #[inline]
    pub fn get_width(&self) -> u32 {
        self.width
    }

    ///
    #[inline]
    pub fn get_height(&self) -> u32 {
        self.height
    }

    ///
    #[inline]
    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    ///
    #[inline]
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    ///
    #[inline]
    pub fn get_colour_buffer_raw(&self) -> &Vec<u8> {
        self.colour_buffer.as_raw()
    }

    ///
    #[inline]
    pub fn get_depth_buffer_raw(&self) -> &Vec<f32> {
        self.depth_buffer.as_raw()
    }

    ///
    pub fn save(&self, filepath: &str) -> Result<(), &str> {
        match self.colour_buffer.save(filepath) {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to save image"),
        }
    }

    ///
    pub fn clear(&mut self) {
        self.colour_buffer =
            RgbaImage::from_pixel(self.width, self.height, self.config.clear_colour);
        self.depth_buffer = DepthImage::from_pixel(self.width, self.height, Luma([-1.0]));
    }

    ///
    pub fn draw<T: Drawable>(&mut self, drawable: &T) {
        if self.config.wireframe {
            drawable.draw_wireframe(self);
        } else {
            drawable.draw(self);
        }
    }

    ///
    pub fn line(&mut self, p0: Point3<f32>, p1: Point3<f32>, colour: Rgba<u8>) {
        // Convert to screen-space
        let p0 = self.to_screen(p0);
        let p1 = self.to_screen(p1);

        let mut x0 = p0.x as i32;
        let mut y0 = p0.y as i32;
        let mut x1 = p1.x as i32;
        let mut y1 = p1.y as i32;

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

        // Calculate triangle visibility
        let visibility0 = (p0 - self.camera.get_position()).dot(&normal);
        let visibility1 = (p1 - self.camera.get_position()).dot(&normal);
        let visibility2 = (p2 - self.camera.get_position()).dot(&normal);
        if (visibility0 < 0.0) && (visibility1 < 0.0) && (visibility2 < 0.0) {
            return;
        }

        // Convert to screen-space
        let p0 = self.to_screen(p0);
        let p1 = self.to_screen(p1);
        let p2 = self.to_screen(p2);

        // Find the screen-space bounding box of this triangle
        let mut bb_min = Vector2::<i32>::new((self.width - 1) as i32, (self.height - 1) as i32);
        let mut bb_max = Vector2::<i32>::new(0, 0);
        let clamp = Vector2::<i32>::new(bb_min.x, bb_min.y);

        for p in &[p0, p1, p2] {
            bb_min.x = cmp::max(0, cmp::min(bb_min.x, p.x.floor() as i32));
            bb_min.y = cmp::max(0, cmp::min(bb_min.y, p.y.floor() as i32));
            bb_max.x = cmp::min(clamp.x, cmp::max(bb_max.x, p.x.ceil() as i32));
            bb_max.y = cmp::min(clamp.y, cmp::max(bb_max.y, p.y.ceil() as i32));
        }

        if (bb_max.x == bb_min.x) || (bb_max.y == bb_min.y) {
            return;
        }

        // Calculate facet lighting
        let light_direction = Vector3::<f32>::new(-1.0, -0.5, -0.25).normalize();
        let lighting_intensity = normal.dot(&light_direction);
        let colour = colour.map_with_alpha(
            |c| utilities::clamp_f32((c as f32) * lighting_intensity, 0.0, 255.0) as u8,
            |a| a,
        );

        // Calculate barycentric coordinate frame
        let b0 = p1 - p0;
        let b1 = p2 - p0;

        let b00 = b0.dot(&b0);
        let b01 = b0.dot(&b1);
        let b11 = b1.dot(&b1);
        let denominator = (b00 * b11) - (b01 * b01);

        // Render
        for y in bb_min.y..=bb_max.y {
            for x in bb_min.x..=bb_max.x {
                // Screen-space coordinates of this pixel
                let p = Point3::<f32>::new(x as f32, y as f32, 0.0);

                // Find the barycentric coordinates of this pixel
                let b = {
                    let b2 = p - p0;
                    let b20 = b2.dot(&b0);
                    let b21 = b2.dot(&b1);
                    let by = ((b11 * b20) - (b01 * b21)) / denominator;
                    let bz = ((b00 * b21) - (b01 * b20)) / denominator;
                    let bx = 1.0 - by - bz;
                    Vector3::<f32>::new(bx, by, bz)
                };

                // If this pixel is outside of the triangle, ignore it
                if (b.x < 0.0)
                    || (b.x > 1.0)
                    || (b.y < 0.0)
                    || (b.y > 1.0)
                    || (b.z < 0.0)
                    || (b.z > 1.0)
                {
                    continue;
                }

                // Calculate the depth
                let depth = 1.0 / ((b.x / p0.z) + (b.y / p1.z) + (b.z / p2.z));

                // Set pixel
                let (u, v) = (x as u32, y as u32);

                if depth > self.depth_buffer.get_pixel(u, v)[0] {
                    self.colour_buffer.put_pixel(u, v, colour);
                    self.depth_buffer.put_pixel(u, v, Luma([depth]));
                }
            }
        }
    }

    /// Convert from World to Screen coordinate system
    #[inline]
    fn to_screen(&self, p: Point3<f32>) -> Point3<f32> {
        let transformed = self.camera.get_view_projection_matrix().transform_point(&p);
        Point3::<f32>::new(
            ((transformed.x + 1.0) / 2.0) * (self.get_width() as f32),
            ((transformed.y + 1.0) / 2.0) * (self.get_height() as f32),
            transformed.z,
        )
    }
}
