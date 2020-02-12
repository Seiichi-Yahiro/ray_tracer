mod intersection;
mod light;
mod object;
mod ray;
mod scene;

use crate::light::{DirectionalLight, Light, SphericalLight};
use crate::object::material::{Coloration, Material, SurfaceType};
use crate::object::plane::Plane;
use crate::object::sphere::Sphere;
use crate::object::Object;
use crate::scene::Scene;
use glutin_window::GlutinWindow as Window;
use image::{DynamicImage, ImageBuffer, Rgb};
use nalgebra::{Perspective3, Point3, Vector3};
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
        lights: vec![
            Light::Directional(DirectionalLight {
                direction: Vector3::new(1.0, -1.0, 0.0).normalize(),
                color: [1.0; 3].into(),
                intensity: 5.0,
            }),
            Light::Spherical(SphericalLight {
                position: Point3::new(-2.5, 1.0, -2.0),
                color: [1.0; 3].into(),
                intensity: 120.0,
            }),
            Light::Spherical(SphericalLight {
                position: Point3::new(5.0, 3.0, -6.0),
                color: [1.0; 3].into(),
                intensity: 200.0,
            }),
        ],
        objects: vec![
            Object::Sphere(Sphere {
                position: Point3::new(-1.0, 1.5, -3.0),
                radius: 1.0,
                material: Material {
                    color: Coloration::Color([1.0, 0.0, 0.0].into()),
                    albedo: 0.18,
                    surface: SurfaceType::Diffuse,
                },
            }),
            Object::Sphere(Sphere {
                position: Point3::new(4.0, 0.0, -5.0),
                radius: 1.5,
                material: Material {
                    color: Coloration::Color([1.0, 1.0, 1.0].into()),
                    albedo: 0.18,
                    surface: SurfaceType::Refractive {
                        transparency: 0.4,
                        index: 2.0,
                    },
                },
            }),
            Object::Sphere(Sphere {
                position: Point3::new(0.0, 0.0, -4.3),
                radius: 0.5,
                material: Material {
                    color: Coloration::Color([1.0, 1.0, 0.0].into()),
                    albedo: 0.18,
                    surface: SurfaceType::Reflective { reflectivity: 0.4 },
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
                    surface: SurfaceType::Reflective { reflectivity: 0.1 },
                },
            }),
            Object::Plane(Plane {
                point: Point3::new(0.0, 5.0, 0.0),
                normal: Vector3::y(),
                material: Material {
                    color: Coloration::Texture(DynamicImage::ImageRgb8(ImageBuffer::from_fn(
                        10,
                        10,
                        |x, y| {
                            let color = if x / 5 == y / 5 {
                                [20, 20, 230]
                            } else {
                                [100; 3]
                            };
                            Rgb(color)
                        },
                    ))),
                    albedo: 0.18,
                    surface: SurfaceType::Diffuse,
                },
            }),
            Object::Plane(Plane {
                point: Point3::new(7.0, 0.0, 0.0),
                normal: Vector3::x(),
                material: Material {
                    color: Coloration::Texture(DynamicImage::ImageRgb8(ImageBuffer::from_fn(
                        10,
                        10,
                        |x, y| {
                            let color = if x / 5 == y / 5 {
                                [230, 150, 80]
                            } else {
                                [100; 3]
                            };
                            Rgb(color)
                        },
                    ))),
                    albedo: 0.18,
                    surface: SurfaceType::Diffuse,
                },
            }),
            Object::Plane(Plane {
                point: Point3::new(-7.0, 0.0, 0.0),
                normal: -Vector3::x(),
                material: Material {
                    color: Coloration::Texture(DynamicImage::ImageRgb8(ImageBuffer::from_fn(
                        10,
                        10,
                        |x, y| {
                            let color = if x / 5 == y / 5 {
                                [230, 150, 80]
                            } else {
                                [100; 3]
                            };
                            Rgb(color)
                        },
                    ))),
                    albedo: 0.18,
                    surface: SurfaceType::Diffuse,
                },
            }),
            Object::Plane(Plane {
                point: Point3::new(0.0, 0.0, -10.0),
                normal: -Vector3::z(),
                material: Material {
                    color: Coloration::Texture(DynamicImage::ImageRgb8(ImageBuffer::from_fn(
                        10,
                        10,
                        |x, y| {
                            let color = if x / 5 == y / 5 {
                                [230, 230, 80]
                            } else {
                                [100; 3]
                            };
                            Rgb(color)
                        },
                    ))),
                    albedo: 0.18,
                    surface: SurfaceType::Diffuse,
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
