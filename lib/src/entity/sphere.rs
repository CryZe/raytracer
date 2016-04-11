use nalgebra as na;
use Vec3;
use brdf::Brdf;
use entity::Entity;
use ray::Ray;
use collision::Collision;
use std::f32;

pub struct Sphere<BrdfType: Brdf + 'static> {
    center: Vec3,
    radius: f32,
    brdf: BrdfType,
}

impl<BrdfType: Brdf + 'static> Sphere<BrdfType> {
    pub fn new(center: Vec3, radius: f32, brdf: BrdfType) -> Self {
        Sphere {
            center: center,
            radius: radius,
            brdf: brdf,
        }
    }
}

impl<BrdfType: Brdf + 'static> Entity for Sphere<BrdfType> {
    type BrdfType = BrdfType;

    fn collides_with(&self, ray: &Ray) -> Option<Collision<Self::BrdfType>> {
        let m = ray.origin - self.center;
        let b = na::dot(&m, &ray.direction);
        let c = na::dot(&m, &m) - self.radius * self.radius;

        if c > 0.0 && b > 0.0 {
            return None;
        }

        let discr = b * b - c;

        if discr < 0.0 {
            return None;
        }

        let mut t = -b - f32::sqrt(discr);

        if t < 0.0 {
            t = 0.0;
        }

        let hit_position = ray.direction * t + ray.origin;
        let hit_normal = na::normalize(&(hit_position - self.center));
        Some(Collision::new(hit_position, hit_normal, &self.brdf))
    }

    fn position(&self) -> Vec3 {
        self.center
    }

    fn set_position(&mut self, p: Vec3) {
        self.center = p;
    }
}
