use crate::intersection::{Intersectable, Intersection};
use crate::light::Light;
use crate::object::material::{Color, SurfaceType};
use crate::object::Object;
use crate::ray::Ray;
use image::{ImageBuffer, RgbaImage};
use nalgebra::{Point3, Vector3};
use rayon::prelude::*;
use std::f64::consts::PI;

const SHADOW_BIAS: f64 = 1e-13;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,

    pub max_recursion_depth: u32,
}

impl Scene {
    pub fn create_image(&self) -> RgbaImage {
        let pixels = (0..self.height)
            .into_par_iter()
            .flat_map(|y| {
                (0..self.width)
                    .flat_map(|x| {
                        let ray = Ray::create_prime(x, y, &self);
                        self.cast_ray(&ray, self.max_recursion_depth)
                            .to_u8()
                            .to_vec()
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<u8>>();

        ImageBuffer::from_vec(self.width, self.height, pixels).unwrap()
    }

    fn get_color(&self, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
        let hit_point = &ray.origin + &ray.direction * intersection.distance;
        let surface_normal = intersection.object.surface_normal(&hit_point);

        self.lights
            .iter()
            .map(|light| {
                let mut color =
                    self.shade_diffuse(&intersection, light, &hit_point, &surface_normal);

                if let SurfaceType::Reflective { reflectivity } =
                    intersection.object.material().surface
                {
                    let reflection_ray = Ray::create_reflection(
                        surface_normal,
                        ray.direction,
                        hit_point,
                        SHADOW_BIAS,
                    );
                    color = color * (1.0 - reflectivity);
                    color = color + self.cast_ray(&reflection_ray, depth - 1) * reflectivity;
                }

                color
            })
            .fold(Color([0.0, 0.0, 0.0]), |acc_color, color| acc_color + color)
    }

    fn shade_diffuse(
        &self,
        intersection: &Intersection,
        light: &Light,
        hit_point: &Point3<f64>,
        surface_normal: &Vector3<f64>,
    ) -> Color {
        let direction_to_light = light.direction_to_light(&hit_point);
        let shadow_ray = Ray {
            origin: hit_point + surface_normal * SHADOW_BIAS,
            direction: direction_to_light,
        };

        let shadow_ray_intersection = self.trace(&shadow_ray);

        let is_in_light = shadow_ray_intersection.is_none()
            || shadow_ray_intersection.unwrap().distance > light.distance_to(&hit_point);

        let light_intensity = if is_in_light {
            light.intensity(&hit_point)
        } else {
            0.0
        };

        let light_power = surface_normal.dot(&direction_to_light).max(0.0) * light_intensity;
        let light_reflected = intersection.object.material().albedo / PI;

        intersection
            .object
            .material()
            .color
            .color_at(&intersection.object.texture_coords(&hit_point))
            * light.color()
            * light_power
            * light_reflected
    }

    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|object| {
                object
                    .intersect(ray)
                    .map(|distance| Intersection::new(distance, object))
            })
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }

    pub fn cast_ray(&self, ray: &Ray, depth: u32) -> Color {
        if depth == 0 {
            return Color([0.0; 3]);
        }

        self.trace(ray)
            .map(|intersection| self.get_color(ray, &intersection, depth))
            .unwrap_or(Color([0.1, 0.1, 0.7]))
    }
}
