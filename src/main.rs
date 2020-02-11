mod intersection;
mod light;
mod object;
mod ray;
mod scene;

use crate::light::{DirectionalLight, Light, SphericalLight};
use crate::object::material::{Coloration, Material};
use crate::object::plane::Plane;
use crate::object::sphere::Sphere;
use crate::object::Object;
use crate::scene::Scene;
use glutin_window::GlutinWindow as Window;
use image::{DynamicImage, ImageBuffer, Rgb};
use nalgebra::{Point3, Vector3};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V4_5;
    let (pixel_width, pixel_height) = (800, 600);

    let mut window: Window = WindowSettings::new("Ray tracing", [pixel_width, pixel_height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let scene = Scene {
        width: pixel_width,
        height: pixel_height,
        fov: 90.0,
        lights: vec![
            Light::Directional(DirectionalLight {
                direction: Vector3::new(1.0, -1.0, 0.0).normalize(),
                color: [1.0; 3].into(),
                intensity: 5.0,
            }),
            Light::Spherical(SphericalLight {
                position: Point3::new(0.0, 2.0, -2.0),
                color: [1.0; 3].into(),
                intensity: 80.0,
            }),
        ],
        objects: vec![
            Object::Sphere(Sphere {
                position: Point3::new(-1.0, 1.5, -3.0),
                radius: 1.0,
                material: Material {
                    color: Coloration::Color([1.0, 0.0, 0.0].into()),
                    albedo: 0.18,
                },
            }),
            Object::Sphere(Sphere {
                position: Point3::new(5.0, 0.0, -7.0),
                radius: 1.5,
                material: Material {
                    color: Coloration::Color([0.0, 0.0, 1.0].into()),
                    albedo: 0.18,
                },
            }),
            Object::Sphere(Sphere {
                position: Point3::new(0.0, 0.0, -4.3),
                radius: 0.5,
                material: Material {
                    color: Coloration::Color([0.0, 1.0, 0.0].into()),
                    albedo: 0.18,
                },
            }),
            Object::Plane(Plane {
                point: Point3::new(0.0, -1.5, 0.0),
                normal: -Vector3::y(),
                material: Material {
                    color: Coloration::Texture(DynamicImage::ImageRgb8(ImageBuffer::from_fn(
                        10,
                        10,
                        |x, y| {
                            let color = if x / 5 == y / 5 {
                                [120, 230, 80]
                            } else {
                                [100; 3]
                            };
                            Rgb(color)
                        },
                    ))),
                    albedo: 0.18,
                },
            }),
        ],
    };

    let texture_settings = TextureSettings::new();
    let texture = Texture::from_image(&scene.create_image(), &texture_settings);

    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            gl.draw(render_args.viewport(), |c, gl| {
                graphics::image(&texture, c.transform, gl);
            });
        }
    }
}
