use crate::color::Color;
use nalgebra::{Point3, Vector3};
use std::f64::consts::PI;

pub struct DirectionalLight {
    pub direction: Vector3<f64>,
    pub color: Color,
    pub intensity: f64,
}

pub struct SphericalLight {
    pub position: Point3<f64>,
    pub color: Color,
    pub intensity: f64,
}

pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

impl Light {
    pub fn direction_to_light(&self, hit_point: &Point3<f64>) -> Vector3<f64> {
        match self {
            Light::Directional(directional) => -directional.direction.clone(),
            Light::Spherical(spherical) => (&spherical.position - hit_point).normalize(),
        }
    }

    pub fn intensity(&self, hit_point: &Point3<f64>) -> f64 {
        match self {
            Light::Directional(directional) => directional.intensity,
            Light::Spherical(spherical) => {
                let r2 = (&spherical.position - hit_point).norm();
                spherical.intensity / (4.0 * PI * r2)
            }
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Light::Directional(directional) => directional.color,
            Light::Spherical(spherical) => spherical.color,
        }
    }

    pub fn distance_to(&self, point: &Point3<f64>) -> f64 {
        match self {
            Light::Directional(_) => f64::INFINITY,
            Light::Spherical(spherical) => (spherical.position - point).norm(),
        }
    }
}
