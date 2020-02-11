use crate::intersection::{Intersectable, TextureCoords};
use crate::object::material::Material;
use crate::ray::Ray;
use nalgebra::{Point3, Vector3};

pub mod material;
pub mod plane;
pub mod sphere;

pub enum Object {
    Sphere(sphere::Sphere),
    Plane(plane::Plane),
}

impl Object {
    pub fn material(&self) -> &Material {
        match self {
            Object::Sphere(sphere) => &sphere.material,
            Object::Plane(plane) => &plane.material,
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

    fn texture_coords(&self, hit_point: &Point3<f64>) -> TextureCoords {
        match self {
            Object::Sphere(sphere) => sphere.texture_coords(hit_point),
            Object::Plane(plane) => plane.texture_coords(hit_point),
        }
    }
}
