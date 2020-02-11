use crate::intersection::Intersectable;
use crate::ray::Ray;
use nalgebra::{Point3, Vector3};

pub struct Plane {
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub color: [f64; 3],
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let denom = self.normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = &self.point - &ray.origin;
            let distance = v.dot(&self.normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }

        None
    }

    fn surface_normal(&self, _: &Point3<f64>) -> Vector3<f64> {
        -self.normal.clone()
    }

    fn albedo(&self) -> f64 {
        0.18
    }
}
