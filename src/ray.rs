use crate::{PIXEL_HEIGHT, PIXEL_WIDTH};
use nalgebra::{Perspective3, Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, perspective: &Perspective3<f64>) -> Ray {
        const SIZE: f64 = 2.0;
        const NORMALIZED_WIDTH: f64 = SIZE / PIXEL_WIDTH as f64;
        const NORMALIZED_HEIGHT: f64 = SIZE / PIXEL_HEIGHT as f64;

        let normalized_x = NORMALIZED_WIDTH * (x as f64 + 0.5) - 1.0;
        let normalized_y = 1.0 - NORMALIZED_HEIGHT * (y as f64 + 0.5);

        let near_point = Point3::new(normalized_x, normalized_y, -1.0);
        let far_point = Point3::new(normalized_x, normalized_y, 1.0);

        let origin = perspective.unproject_point(&near_point);
        let direction = (perspective.unproject_point(&far_point) - &origin).normalize();

        Ray { origin, direction }
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

    pub fn create_transmission(
        normal: Vector3<f64>,
        incident: Vector3<f64>,
        intersection: Point3<f64>,
        bias: f64,
        index: f64,
    ) -> Option<Ray> {
        let mut ref_n = normal.clone();
        let mut eta_t = index as f64;
        let mut eta_i = 1.0f64;
        let mut i_dot_n = incident.dot(&normal);
        if i_dot_n < 0.0 {
            //Outside the surface
            i_dot_n = -i_dot_n;
        } else {
            //Inside the surface; invert the normal and swap the indices of refraction
            ref_n = -normal;
            eta_t = 1.0;
            eta_i = index as f64;
        }

        let eta = eta_i / eta_t;
        let k = 1.0 - (eta * eta) * (1.0 - i_dot_n * i_dot_n);
        if k < 0.0 {
            None
        } else {
            Some(Ray {
                origin: intersection + (&ref_n * -bias),
                direction: (incident + i_dot_n * &ref_n) * eta - &ref_n * k.sqrt(),
            })
        }
    }
}
