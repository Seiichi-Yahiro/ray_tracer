use crate::material::Material;
use nalgebra::Isometry3;
use ncollide3d::query::{Ray, RayCast, RayIntersection};
use ncollide3d::shape::Shape;

pub struct Object {
    pub isometry: Isometry3<f64>,
    pub shape: Box<dyn Shape<f64>>,
    pub material: Material,
}

impl Object {
    pub fn new(isometry: Isometry3<f64>, shape: impl Shape<f64>, material: Material) -> Object {
        Object {
            isometry,
            shape: Box::new(shape),
            material,
        }
    }

    pub fn intersect(&self, ray: &Ray<f64>) -> Option<RayIntersection<f64>> {
        self.shape
            .toi_and_normal_with_ray(&self.isometry, ray, false)
    }
}
