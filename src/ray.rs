use crate::scene::Scene;
use nalgebra::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = scene.width as f64 / scene.height as f64;

        let sensor_x = {
            let pixel_center = x as f64 + 0.5;
            let normalized_to_width = pixel_center / scene.width as f64;
            normalized_to_width * 2.0 - 1.0
        } * aspect_ratio
            * fov_adjustment;

        let sensor_y = {
            let pixel_center = y as f64 + 0.5;
            let normalized_to_height = pixel_center / scene.height as f64;
            1.0 - normalized_to_height * 2.0
        } * fov_adjustment;

        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(sensor_x, sensor_y, -1.0).normalize(),
        }
    }

    pub fn create_reflection(
        normal: Vector3<f64>,
        incident: Vector3<f64>,
        intersection: Point3<f64>,
        bias: f64,
    ) -> Ray {
        Ray {
            origin: intersection + normal * bias,
            direction: incident - 2.0 * incident.dot(&normal) * &normal,
        }
    }
}
