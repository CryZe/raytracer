use prelude::*;
use {Ray, Collision};
use nalgebra as na;

pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
}

pub struct Mesh<BrdfType: Brdf + 'static> {
    triangles: Vec<Triangle>,
    position: Vec3,
    brdf: BrdfType,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Triangle { a: a, b: b, c: c }
    }

    pub fn collides_with<'a, BrdfType: Brdf + 'static>(&'a self,
                                                       ray: &Ray,
                                                       brdf: &'a BrdfType)
                                                       -> Option<Collision<BrdfType>> {
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;

        let h = na::cross(&ray.direction, &e2);
        let a = na::dot(&e1, &h);

        if a > -0.00001 && a < 0.00001 {
            return None;
        }

        let f = 1.0 / a;

        let s = ray.origin - self.a;
        let u = f * na::dot(&s, &h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = na::cross(&s, &e1);
        let v = f * na::dot(&ray.direction, &q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * na::dot(&e2, &q);

        if t > 0.00001 {
            let hit_position = ray.direction * t + ray.origin;
            let hit_normal = na::cross(&e1, &e2);
            Some(Collision::new(hit_position, hit_normal, brdf))
        } else {
            None
        }
    }
}

impl<BrdfType: Brdf + 'static> Mesh<BrdfType> {
    pub fn new(triangles: Vec<Triangle>, brdf: BrdfType) -> Self {
        Mesh {
            triangles: triangles,
            position: Vec3::new(0.0, 0.0, 0.0),
            brdf: brdf,
        }
    }
}

impl<BrdfType: Brdf + 'static> Entity for Mesh<BrdfType> {
    type BrdfType = BrdfType;

    fn collides_with(&self, ray: &Ray) -> Option<Collision<Self::BrdfType>> {
        for triangle in &self.triangles {
            if let Some(collision) = triangle.collides_with(ray, &self.brdf) {
                return Some(collision);
            }
        }
        None
    }

    fn position(&self) -> Vec3 {
        self.position
    }

    fn set_position(&mut self, p: Vec3) {
        self.position = p;
    }
}
