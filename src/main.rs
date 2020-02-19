#![feature(bool_to_option)]
#![feature(clamp)]
#![feature(assoc_int_consts)]

mod color;
mod light;
mod material;
mod object;
mod ray;
mod scene;

use crate::light::{DirectionalLight, Light, SphericalLight};
use crate::material::SurfaceType;
use crate::object::ObjectBuilder;
use crate::scene::Scene;
use glutin_window::GlutinWindow as Window;
use image::{DynamicImage, ImageBuffer, Rgb};
use nalgebra::{Perspective3, Point3, Vector3};
use ncollide3d::shape;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

pub const PIXEL_WIDTH: u32 = 800;
pub const PIXEL_HEIGHT: u32 = 600;

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new("Ray Tracer", [PIXEL_WIDTH, PIXEL_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let scene = Scene {
        perspective: Perspective3::new(
            PIXEL_WIDTH as f64 / PIXEL_HEIGHT as f64,
            90.0_f64.to_radians(),
            1.0,
            1000.0,
        ),
        max_recursion_depth: 5,
        max_rays: 20,
        lights: vec![
            /*Light::Directional(DirectionalLight {
                direction: Vector3::new(0.0, -1.0, 0.0).normalize(),
                color: [1.0; 3].into(),
                intensity: 5.0,
            }),*/
            Light::Spherical(SphericalLight {
                position: Point3::new(0.0, 3.7, -5.0),
                color: [1.0; 3].into(),
                intensity: 300.0,
            }),
        ],
        objects: vec![
            ObjectBuilder::new(shape::Ball::new(1.0))
                .position(-2.5, -0.5, -4.0)
                .color([1.0, 0.0, 0.0])
                .build(),
            ObjectBuilder::new(shape::Ball::new(1.5))
                .position(2.0, 0.0, -5.0)
                .color([1.0, 1.0, 1.0])
                .surface(SurfaceType::Refractive {
                    transparency: 0.9,
                    index: 1.5,
                })
                .build(),
            ObjectBuilder::new(shape::Ball::new(0.5))
                .position(0.0, -1.0, -4.3)
                .color([1.0, 1.0, 0.0])
                .surface(SurfaceType::Reflective {
                    reflectivity: 0.4,
                    fuzz: 0.3,
                })
                .build(),
            ObjectBuilder::new(shape::Cuboid::new(Vector3::new(0.5, 1.5, 0.1)))
                .position(-1.5, -1.4, -3.0)
                .rotation(*Vector3::y_axis(), 20.0)
                .surface(SurfaceType::Refractive {
                    transparency: 0.9,
                    index: 1.5,
                })
                .build(),
            ObjectBuilder::new(shape::Plane::new(-Vector3::y_axis()))
                .position(0.0, -1.5, 0.0)
                .surface(SurfaceType::Diffuse)
                .color([0.73, 0.1, 0.73])
                .build(),
            ObjectBuilder::new(shape::Plane::new(Vector3::y_axis()))
                .position(0.0, 4.0, 0.0)
                .surface(SurfaceType::Diffuse)
                .color([0.1, 0.73, 0.73])
                .build(),
            ObjectBuilder::new(shape::Plane::new(-Vector3::x_axis()))
                .position(-4.0, 0.0, 0.0)
                .surface(SurfaceType::Diffuse)
                .color([0.65, 0.05, 0.05])
                .build(),
            ObjectBuilder::new(shape::Plane::new(Vector3::x_axis()))
                .position(4.0, 0.0, 0.0)
                .surface(SurfaceType::Diffuse)
                .color([0.12, 0.45, 0.15])
                .build(),
            ObjectBuilder::new(shape::Plane::new(-Vector3::z_axis()))
                .position(0.0, 0.0, -7.0)
                .surface(SurfaceType::Diffuse)
                .color([0.1, 0.1, 0.73])
                .build(),
            ObjectBuilder::new(shape::Plane::new(Vector3::z_axis()))
                .position(0.0, 0.0, 1.0)
                .surface(SurfaceType::Diffuse)
                .color([0.73, 0.73, 0.73])
                .build(),
        ],
    };

    let texture_settings = TextureSettings::new();
    let image = scene.create_image();
    image.save("image.png");
    let texture = Texture::from_image(&image, &texture_settings);

    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            gl.draw(render_args.viewport(), |c, gl| {
                graphics::image(&texture, c.transform, gl);
            });
        }
    }
}
