use crate::intersection::{Intersectable, Intersection};
use crate::light::Light;
use crate::object::Object;
use crate::ray::Ray;
use image::{ImageBuffer, RgbaImage};
use itertools::Itertools;
use rayon::prelude::*;
use std::f64::consts::PI;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn create_image(&self) -> RgbaImage {
        let pixels = (0..self.height)
            .into_par_iter()
            .flat_map(|y| {
                (0..self.width)
                    .flat_map(|x| {
                        let ray = Ray::create_prime(x, y, &self);

                        if let Some(intersection) = self.trace(&ray) {
                            let hit_point = ray.origin + ray.direction * intersection.distance;
                            let surface_normal = intersection.object.surface_normal(&hit_point);

                            self.lights
                                .iter()
                                .map(|light| {
                                    let direction_to_light = light.direction_to_light(&hit_point);
                                    let shadow_ray = Ray {
                                        origin: hit_point + surface_normal * 1e-13,
                                        direction: direction_to_light,
                                    };

                                    let shadow_ray_intersection = self.trace(&shadow_ray);

                                    let is_in_light = shadow_ray_intersection.is_none()
                                        || shadow_ray_intersection.unwrap().distance
                                            > light.distance_to(&hit_point);

                                    let light_intensity = if is_in_light {
                                        light.intensity(&hit_point)
                                    } else {
                                        0.0
                                    };

                                    let light_power =
                                        surface_normal.dot(&direction_to_light).max(0.0)
                                            * light_intensity;
                                    let light_reflected = intersection.object.albedo() / PI;

                                    intersection
                                        .object
                                        .color()
                                        .iter()
                                        .zip(light.color().iter())
                                        .map(|(object_color_channel, light_color_channel)| {
                                            *object_color_channel
                                                * *light_color_channel
                                                * light_power
                                                * light_reflected
                                        })
                                        .collect_vec()
                                })
                                .fold(vec![0.0, 0.0, 0.0], |mut acc_color, color| {
                                    acc_color.iter_mut().zip(color.iter()).for_each(
                                        |(acc_color_channel, color_channel)| {
                                            *acc_color_channel += *color_channel;
                                        },
                                    );
                                    acc_color
                                })
                                .iter()
                                .map(|color_channel| {
                                    (*color_channel * 255.0).min(255.0).max(0.0) as u8
                                })
                                .pad_using(4, |_| 255)
                                .collect_vec()
                        } else {
                            vec![25, 25, 100, 255]
                        }
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<u8>>();

        ImageBuffer::from_vec(self.width, self.height, pixels).unwrap()
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
}
