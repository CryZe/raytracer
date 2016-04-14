use prelude::*;
use ray::Ray;
use collision::Collision;

pub trait Entity {
    type BrdfType: Brdf;
    fn collides_with(&self, ray: &Ray) -> Option<Collision<Self::BrdfType>>;
    fn position(&self) -> Vec3;
    fn set_position(&mut self, p: Vec3);
}

pub mod sphere;
pub mod camera;
pub mod mesh;

pub use self::sphere::Sphere;
pub use self::camera::Camera;
pub use self::mesh::{Triangle, Mesh};
