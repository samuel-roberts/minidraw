
use image::{RgbaImage, Rgba};
use obj::{Obj};

#[derive(Copy, Clone)]
struct Point
{
    x: i32,
    y: i32
}

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

fn main() {

    let mut image = RgbaImage::new(WIDTH, HEIGHT);

    let data = Obj::load("test.obj").unwrap().data;

    for object in &data.objects {
        for group in &object.groups {
            for polygon in &group.polys {
                
                // Ignore polygons that are not triangles
                if polygon.0.len() != 3 {
                    continue;
                }

                // Get the indicies of the points of the triangle
                let idx0 = polygon.0[0].0;
                let idx1 = polygon.0[1].0;
                let idx2 = polygon.0[2].0;

                let indices = vec!(idx0, idx1, idx2);
                let positions: Vec<_> = indices
                    .iter()
                    .map(|i| data.position[*i])
                    .map(|p| Point { 
                        x: (((p[0] + 1.0) * ((WIDTH as f32) / 2.0)) as i32), 
                        y: (((p[1] + 1.0) * ((HEIGHT as f32) / 2.0)) as i32) 
                    })
                    .collect();

                line(&mut image, positions[0], positions[1], Rgba([0, 0, 0, 255]));
                line(&mut image, positions[1], positions[2], Rgba([0, 0, 0, 255]));
                line(&mut image, positions[2], positions[0], Rgba([0, 0, 0, 255]));
            }
        }
    }

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
        if x < 0 || x as u32 >= WIDTH || y < 0 || y as u32 >= HEIGHT {
            continue;
        }

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