use image::{ImageBuffer, Luma, Pixel, Rgba, RgbaImage};
use nalgebra::{Matrix4, Point3, Vector3};
use minifb::{Key, Window, WindowOptions};

mod renderer;
mod utilities;
mod mesh;
mod drawable;

use renderer::Renderer;
use mesh::Mesh;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    // Init renderer
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let view = Matrix4::<f32>::look_at_lh(
        &Point3::<f32>::new(25.0, 25.0, 25.0),
        &Point3::<f32>::origin(),
        &Vector3::<f32>::z_axis(),
    );
    let projection = Matrix4::<f32>::new_perspective(
        (renderer.get_width() as f32) / (renderer.get_height() as f32),
        std::f32::consts::PI / 2.0,
        1e-5,
        1e5,
    );

    renderer.push_view_matrix(view);
    renderer.push_projection_matrix(projection);

    // Load mesh
    let mesh = Mesh::load_obj("models/calibration.obj").expect("Failed to load model");

    // Init window
    let mut window = Window::new(
        "Minidraw",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    ).expect("Failed to create window");

    // Limit to max ~60 fps update rate
    //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        // Draw
        renderer.clear(Rgba([0, 0, 0, 255]));
        renderer.draw(&mesh);

        // Adapt to frame buffer
        let src = renderer.get_colour_buffer_raw();
        let dst = &mut buffer;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let offset_dst = ((y * WIDTH) + x) as usize;
                let offset_src = offset_dst * 4;

                let r = src[offset_src] as u32;
                let g = src[offset_src + 1] as u32;
                let b = src[offset_src + 2] as u32;
                let a = src[offset_src + 3] as u32;
                
                dst[offset_dst] = (a << 24) + (r << 16) + (g << 8) + b;
            }
        }

        // Save if S key is pressed
        if window.is_key_released(Key::S) {
            renderer.save("output.png").unwrap();
        }
        
        // Display
        window.update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize).unwrap();
    }
}
