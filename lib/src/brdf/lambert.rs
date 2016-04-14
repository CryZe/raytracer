use prelude::*;
use std::f32::consts::PI;

pub struct Lambert {
    albedo: Rgb,
}

impl Lambert {
    pub fn new(albedo: Rgb) -> Self {
        Lambert { albedo: albedo }
    }
}

impl Brdf for Lambert {
    fn solve(&self, _: Vec3, _: Vec3, _: Vec3) -> Rgb {
        self.albedo / PI
    }

    fn solve_emissive(&self) -> Rgb {
        Rgb::new(0.0, 0.0, 0.0)
    }
}
