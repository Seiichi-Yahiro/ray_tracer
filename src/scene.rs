use crate::color::Color;
use crate::light::Light;
use crate::material::SurfaceType;
use crate::object::Object;
use crate::{ray, PIXEL_HEIGHT, PIXEL_WIDTH};
use image::{ImageBuffer, RgbaImage};
use nalgebra::{Perspective3, Point3, Vector3};
use ncollide3d::query::{Ray, RayIntersection};
use rayon::prelude::*;
use std::f64::consts::PI;

const SHADOW_BIAS: f64 = 1e-13;

pub struct Scene {
    pub perspective: Perspective3<f64>,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,

    pub max_recursion_depth: u32,
}

impl Scene {
    pub fn create_image(&self) -> RgbaImage {
        let pixels = (0..PIXEL_HEIGHT)
            .into_par_iter()
            .flat_map(|y| {
                (0..PIXEL_WIDTH)
                    .flat_map(|x| {
                        let ray = ray::create_prime(x, y, &self.perspective);
                        self.cast_ray(&ray, self.max_recursion_depth)
                            .to_u8()
                            .to_vec()
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<u8>>();

        ImageBuffer::from_vec(PIXEL_WIDTH, PIXEL_HEIGHT, pixels).unwrap()
    }

    fn get_color(
        &self,
        ray: &Ray<f64>,
        object: &Object,
        intersection: &RayIntersection<f64>,
        depth: u32,
    ) -> Color {
        let hit_point = ray.point_at(intersection.toi);

        match object.material.surface {
            SurfaceType::Diffuse => self.shade_diffuse(object, &hit_point, &intersection.normal),
            SurfaceType::Reflective { reflectivity } => {
                let reflection_ray =
                    ray::create_reflection(intersection.normal, ray.dir, hit_point, SHADOW_BIAS);
                let mut color = self.shade_diffuse(object, &hit_point, &intersection.normal);
                color = color * (1.0 - reflectivity);
                color + self.cast_ray(&reflection_ray, depth - 1) * reflectivity
            }
            SurfaceType::Refractive {
                transparency,
                index,
            } => {
                let mut refraction_color = Color([0.0; 3]);
                let kr = Self::fresnel(ray.dir, intersection.normal, index);
                let surface_color = object.material.color;
                //.color_at(&intersection.object.texture_coords(&hit_point));

                if kr < 1.0 {
                    let transmission_ray = ray::create_transmission(
                        intersection.normal,
                        ray.dir,
                        hit_point,
                        SHADOW_BIAS,
                        index,
                    )
                    .unwrap();

                    refraction_color = self.cast_ray(&transmission_ray, depth - 1);
                }

                let reflection_ray =
                    ray::create_reflection(intersection.normal, ray.dir, hit_point, SHADOW_BIAS);
                let reflection_color = self.cast_ray(&reflection_ray, depth - 1);

                let color = reflection_color * kr + refraction_color * (1.0 - kr);
                color * transparency * surface_color
            }
        }
    }

    fn shade_diffuse(
        &self,
        object: &Object,
        hit_point: &Point3<f64>,
        surface_normal: &Vector3<f64>,
    ) -> Color {
        self.lights
            .iter()
            .map(|light| {
                let direction_to_light = light.direction_to_light(&hit_point);
                let shadow_ray =
                    Ray::new(hit_point + surface_normal * SHADOW_BIAS, direction_to_light);

                let light_intensity = self
                    .trace(&shadow_ray)
                    .map(|(_, intersection)| intersection.toi > light.distance_to(&hit_point)) // is hitted object behind light
                    .unwrap_or(true)
                    .then(|| light.intensity(&hit_point))
                    .unwrap_or(0.0);

                let light_power =
                    surface_normal.dot(&direction_to_light).max(0.0) * light_intensity;
                let light_reflected = object.material.albedo / PI;

                object
                    .material.color
                    //.color_at(&intersection.object.texture_coords(&hit_point))
                    * light.color()
                    * light_power
                    * light_reflected
            })
            .fold(Color([0.0, 0.0, 0.0]), |acc_color, color| acc_color + color)
    }

    fn fresnel(incident: Vector3<f64>, normal: Vector3<f64>, index: f64) -> f64 {
        let i_dot_n = incident.dot(&normal);
        let mut eta_i = 1.0;
        let mut eta_t = index;
        if i_dot_n > 0.0 {
            eta_i = eta_t;
            eta_t = 1.0;
        }

        let sin_t = eta_i / eta_t * (1.0 - i_dot_n * i_dot_n).max(0.0).sqrt();
        if sin_t > 1.0 {
            //Total internal reflection
            1.0
        } else {
            let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
            let cos_i = cos_t.abs();
            let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
            let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
            (r_s * r_s + r_p * r_p) / 2.0
        }
    }

    fn trace(&self, ray: &Ray<f64>) -> Option<(&Object, RayIntersection<f64>)> {
        self.objects
            .iter()
            .filter_map(|object| {
                object
                    .intersect(ray)
                    .map(|intersection| (object, intersection))
            })
            .min_by(|(_, a), (_, b)| a.toi.partial_cmp(&b.toi).unwrap())
    }

    pub fn cast_ray(&self, ray: &Ray<f64>, depth: u32) -> Color {
        if depth == 0 {
            return Color([0.0; 3]);
        }

        self.trace(ray)
            .map(|(object, intersection)| self.get_color(ray, &object, &intersection, depth))
            .unwrap_or(Color([0.0; 3]))
    }
}
