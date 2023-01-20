#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod vector;
mod color;
mod ray;
mod plane;
mod shape;
mod scene;
mod sphere; 
mod material;
mod matrix;

use scene::Scene;
use scene::Camera;
use ray::Ray;
use vector::V3;
use plane::Plane;
use shape::RayTraceShape;
use color::Col3;
use material::Material;
use matrix::Matrix3;
const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() -> Result<(), Error> {
    let mut scene = scene::Scene {
        geometry : vec![
            Box::new(plane::Plane {
                origin : V3::new(0.0, 0.0, 6.5),
                base_one : V3::new(1.0, 0.0, 0.0),
                base_two : V3::new(0.0, 1.0, 0.0),
                material : Material::new(0.9, 0.95, Col3::new(100, 255, 120), 0.95)
            }),
            Box::new(plane::Plane {
                origin : V3::new(0.0, 0.0, -6.5),
                base_one : V3::new(1.0, 0.0, 0.0),
                base_two : V3::new(0.0, 1.0, 0.0),
                material : Material::new(0.9, 0.95, Col3::new(100, 255, 120), 0.95)
            }), 

            Box::new(plane::Plane {
                origin : V3::new(3.5, 0.0, 0.0),
                base_one : V3::new(0.0, 0.0, 1.0),
                base_two : V3::new(0.0, 1.0, 0.0),
                material : Material::new(0.9, 0.95, Col3::new(100, 120, 255), 0.8)
            }),

            Box::new(plane::Plane {
                origin : V3::new(-3.5, 0.0, 0.0),
                base_one : V3::new(0.0, 0.0, 1.0),
                base_two : V3::new(0.0, 1.0, 0.0),
                material : Material::new(0.9, 0.4, Col3::new(255, 120, 100), 0.9)
            }),
            
            Box::new(plane::Plane {
                origin : V3::new(0.0, -2.0, 0.0),
                base_one : V3::new(1.0, 0.0, 0.0),
                base_two : V3::new(0.0, 0.0, 1.0),
                material : Material::new(0.9, 0.7, Col3::new(255, 255, 255), 0.87)
            }),
            Box::new(plane::Plane {
                origin : V3::new(0.0, 3.0, 0.0),
                base_one : V3::new(1.0, 0.0, 0.0),
                base_two : V3::new(0.0, 0.0, 1.0),
                material : Material::new(0.9, 0.7, Col3::new(0, 255, 255), 0.87)
            }),
            Box::new(sphere::Sphere {
                origin : V3::new(0.0, 1.0, 2.0),
                radius : 1.0,
                material : Material::new(0.5, 0.9, Col3::new(255, 0, 0), 0.9)

            }),
            Box::new(sphere::Sphere {
                origin : V3::new(1.2, 0.5, 1.0),
                radius : 0.5,
                material : Material::new(0.5, 0.9, Col3::new(255, 255, 240), 1.0)

            }),
            Box::new(sphere::Sphere {
                origin : V3::new(2.0, 1.0, 3.0),
                radius : 1.0,
                material : Material::new(0.5, 0.7, Col3::new(120, 200, 150), 0.7)

            })
        ],
        camera : Camera {
            location : V3::new(1.8, 1.5, 0.0),
            rotation_y : 0.0,
            rotation_x : 3.141025 * 0.25,
            viewport_anchor : V3::new(-1.0, 0.0, 0.25), // relative to the camera. The botleft corner of the viewport in camera
                                  // space
            size_x : 512,
            size_y : 512,
            rays_per_pixel : 2,
            bounce_depth : 2,
            max_steps : 1,
            step_len : 1000.0
        }
    };
    let mut img = scene.render();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let mut velocity : V3 = V3::zero();
    let mut rot_vel : f32 = 0.0;
    let mut raycnt : u16 = 1;
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    
    
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            img = scene.render();
            for (i, pixel) in pixels.get_frame_mut().chunks_exact_mut(4).enumerate() {
                let x = (i % WIDTH as usize) as i16;
                let y = (i / WIDTH as usize) as i16;

                let a = img[i];
                let rgba = [a.r, a.g, a.b, 0xff];

                pixel.copy_from_slice(&rgba);
            }
            if let Err(err) = pixels.render() {
                panic!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            scene.camera.location = scene.camera.location + Matrix3::rotation_y(scene.camera.rotation_y).transform_vec3(velocity);
            scene.camera.rays_per_pixel = raycnt;
            scene.camera.rotation_y = scene.camera.rotation_y + rot_vel;
            velocity = 0.3 * velocity;
            rot_vel = 0.5 * rot_vel;
            let accel = 0.1;
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::R) {
                scene.camera.bounce_depth = scene.camera.bounce_depth + 1;
            }
            if input.key_pressed(VirtualKeyCode::E) {
                rot_vel = rot_vel + 3.141502 * 0.125;
            }
            if input.key_pressed(VirtualKeyCode::Q) {
                rot_vel = rot_vel - 3.141502 * 0.125;

            }

            if input.key_pressed(VirtualKeyCode::W) {
                velocity = velocity + V3::new(0.0, 0.0, accel);
            }
            if input.key_pressed(VirtualKeyCode::A) {
                velocity = velocity + V3::new(-accel, 0.0, 0.0);
            }
            if input.key_pressed(VirtualKeyCode::S) {
                velocity = velocity + V3::new(0.0, 0.0, -accel);
            }
            if input.key_pressed(VirtualKeyCode::D) {
                velocity = velocity + V3::new(accel, 0.0, 0.0);
            }
            if input.key_pressed(VirtualKeyCode::Space) {
                velocity = velocity + V3::new(0.0, accel, 0.0);
            }
            if input.key_pressed(VirtualKeyCode::C) {
                velocity = velocity + V3::new(0.0, -accel, 0.0);
            }
            if input.key_pressed(VirtualKeyCode::O) {
                raycnt = raycnt + 20;
            }
            if input.key_pressed(VirtualKeyCode::P) {
                raycnt = 2;
                scene.camera.bounce_depth = 1;
            }

            println!("velocity {:?}", velocity);
            println!("position {:?}", scene.camera.location);
            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    panic!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
    
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
            
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
