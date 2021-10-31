#![allow(dead_code)]

use minifb::{Key, Window, WindowOptions};
use nalgebra::{Matrix4, Point3, Vector3};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::time::Instant;

mod drawable;
mod mesh;
mod renderer;
mod renderer_config;
mod transformable;
mod utilities;

use mesh::mesh::Mesh;
use renderer::Renderer;

use crate::{renderer_config::RendererConfig, transformable::Transformable};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    // Init window
    let mut window = Window::new(
        "Loading...",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    // Init renderer
    let config = RendererConfig::default();
    let mut renderer = Renderer::new(WIDTH, HEIGHT, config);

    let view = Matrix4::<f32>::look_at_lh(
        &Point3::<f32>::new(25.0, 25.0, 25.0),
        &Point3::<f32>::origin(),
        &Vector3::<f32>::z_axis(),
    );
    let projection = Matrix4::<f32>::new_perspective(
        (renderer.get_width() as f32) / (renderer.get_height() as f32),
        std::f32::consts::PI / 2.0,
        0.1,
        10000.0,
    );

    renderer.push_view_matrix(view);
    renderer.push_projection_matrix(projection);

    // Load mesh
    let mut mesh = Mesh::load_obj("models/teapot.obj").expect("Failed to load model");

    let mut buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];
    let mut frame_timer = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update framerate
        let frame_duration = frame_timer.elapsed().as_secs_f32();
        let frame_rate = 1.0 / frame_duration;
        frame_timer = Instant::now();
        window.set_title(&format!("Minidraw ({:.2}fps)", frame_rate));

        // Draw
        renderer.clear();
        renderer.draw(&mesh);

        // Adapt to frame buffer
        frame_copy(renderer.get_colour_buffer_raw(), &mut buffer);

        // Save if S key is pressed
        if window.is_key_released(Key::S) {
            renderer.save("output.png").unwrap();
        } else if window.is_key_down(Key::Right) {
            mesh.rotate(0.0, 0.0, std::f32::consts::PI * frame_duration);
        } else if window.is_key_down(Key::Left) {
            mesh.rotate(0.0, 0.0, -std::f32::consts::PI * frame_duration);
        } else if window.is_key_down(Key::RightBracket) {
            mesh.scale(1.0 + frame_duration);
        } else if window.is_key_down(Key::LeftBracket) {
            mesh.scale(1.0 - frame_duration);
        }

        // Display
        window
            .update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize)
            .expect("Failed to update window");
    }
}

///
fn frame_copy(src: &Vec<u8>, dst: &mut Vec<u32>) {
    dst.par_iter_mut()
        .enumerate()
        .for_each(|(offset_dst, pixel)| {
            let offset_src = offset_dst * 4;

            let r = src[offset_src] as u32;
            let g = src[offset_src + 1] as u32;
            let b = src[offset_src + 2] as u32;
            let a = src[offset_src + 3] as u32;

            *pixel = (a << 24) + (r << 16) + (g << 8) + b;
        });
}
