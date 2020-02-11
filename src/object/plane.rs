use crate::intersection::{Intersectable, TextureCoords};
use crate::object::material::Material;
use crate::ray::Ray;
use nalgebra::{Point3, Vector3};

pub struct Plane {
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: Material,
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

    fn texture_coords(&self, hit_point: &Point3<f64>) -> TextureCoords {
        let mut x_axis = self.normal.cross(&Vector3::new(0.0, 0.0, 1.0));

        if x_axis.norm() == 0.0 {
            x_axis = self.normal.cross(&Vector3::new(0.0, 1.0, 0.0));
        }

        let y_axis = self.normal.cross(&x_axis);

        let hit_vec = hit_point - &self.point;

        TextureCoords {
            x: hit_vec.dot(&x_axis),
            y: hit_vec.dot(&y_axis),
        }
    }
}
