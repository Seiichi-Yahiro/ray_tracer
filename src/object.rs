use crate::intersection::Intersectable;
use crate::ray::Ray;
use nalgebra::{Point3, Vector3};

pub mod plane;
pub mod sphere;

pub enum Object {
    Sphere(sphere::Sphere),
    Plane(plane::Plane),
}

impl Object {
    pub fn color(&self) -> [f64; 3] {
        match self {
            Object::Sphere(sphere) => sphere.color,
            Object::Plane(plane) => plane.color,
        }
    }
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Object::Sphere(sphere) => sphere.intersect(ray),
            Object::Plane(plane) => plane.intersect(ray),
        }
    }

    fn surface_normal(&self, hit_point: &Point3<f64>) -> Vector3<f64> {
        match self {
            Object::Sphere(sphere) => sphere.surface_normal(hit_point),
            Object::Plane(plane) => plane.surface_normal(hit_point),
        }
    }

    fn albedo(&self) -> f64 {
        match self {
            Object::Sphere(sphere) => sphere.albedo(),
            Object::Plane(plane) => plane.albedo(),
        }
    }
}
