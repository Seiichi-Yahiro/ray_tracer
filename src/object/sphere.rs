use crate::intersection::{Intersectable, TextureCoords};
use crate::object::material::Material;
use crate::ray::Ray;
use nalgebra::{Point3, Vector3};
use std::f64::consts::PI;

pub struct Sphere {
    pub position: Point3<f64>,
    pub radius: f64,
    pub material: Material,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let ray_origin_to_sphere_center = &self.position - &ray.origin;
        let ray_with_length = ray_origin_to_sphere_center.dot(&ray.direction);

        let d2 =
            ray_origin_to_sphere_center.dot(&ray_origin_to_sphere_center) - ray_with_length.powi(2);
        let radius2 = self.radius.powi(2);

        if d2 > radius2 {
            return None;
        }

        let thc = (radius2 - d2).sqrt();
        let t0 = ray_with_length - thc;
        let t1 = ray_with_length + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }

    fn surface_normal(&self, hit_point: &Point3<f64>) -> Vector3<f64> {
        (*hit_point - self.position).normalize()
    }

    fn texture_coords(&self, hit_point: &Point3<f64>) -> TextureCoords {
        let hit_vec = hit_point - &self.position;
        TextureCoords {
            x: ((1.0 + hit_vec.z.atan2(hit_vec.x)) / PI) * 0.5,
            y: (hit_vec.y / self.radius).acos() / PI,
        }
    }
}
