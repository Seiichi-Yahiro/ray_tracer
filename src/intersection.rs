use crate::object::Object;
use crate::ray::Ray;
use nalgebra::{Point3, Vector3};

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, object: &'a Object) -> Intersection<'a> {
        Intersection { distance, object }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Point3<f64>) -> Vector3<f64>;
    fn albedo(&self) -> f64;
}
