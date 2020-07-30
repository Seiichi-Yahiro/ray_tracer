use crate::color::Color;
use crate::material::{Material, SurfaceType};
use nalgebra::{Isometry3, Translation3, Unit, UnitQuaternion, Vector3};
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
            .toi_and_normal_with_ray(&self.isometry, ray, 100.0, false)
    }
}

pub struct ObjectBuilder<S: Shape<f64>> {
    translation: Translation3<f64>,
    rotation: UnitQuaternion<f64>,
    shape: S,
    albedo: f64,
    color: Color,
    surface: SurfaceType,
}

impl<S: Shape<f64>> ObjectBuilder<S> {
    pub fn new(shape: S) -> Self {
        Self {
            shape,
            translation: Translation3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.0),
            albedo: 0.18,
            color: [1.0; 3].into(),
            surface: SurfaceType::Diffuse,
        }
    }

    pub fn position(mut self, x: f64, y: f64, z: f64) -> Self {
        self.translation = Translation3::new(x, y, z);
        self
    }

    pub fn rotation(mut self, axis: Vector3<f64>, degree: f64) -> Self {
        self.rotation =
            UnitQuaternion::from_axis_angle(&Unit::new_normalize(axis), degree.to_radians());
        self
    }

    pub fn albedo(mut self, value: f64) -> Self {
        self.albedo = value;
        self
    }

    pub fn color(mut self, value: impl Into<Color>) -> Self {
        self.color = value.into();
        self
    }

    pub fn surface(mut self, value: SurfaceType) -> Self {
        self.surface = value;
        self
    }

    pub fn build(self) -> Object {
        Object {
            isometry: Isometry3::from_parts(self.translation, self.rotation),
            shape: Box::new(self.shape),
            material: Material {
                color: self.color,
                albedo: self.albedo,
                surface: self.surface,
            },
        }
    }
}
