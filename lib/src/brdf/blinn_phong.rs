use prelude::*;
use nalgebra as na;
use std::f32;

pub struct BlinnPhong {
    n: f32,
}

impl BlinnPhong {
    pub fn new(n: f32) -> Self {
        BlinnPhong { n: n }
    }
}

impl Brdf for BlinnPhong {
    fn solve(&self, l: Vec3, n: Vec3, v: Vec3) -> Rgb {
        let h = na::normalize(&(l + v));

        let val = f32::powf(na::dot(&n, &h).saturate(), self.n);

        Rgb::new(val, val, val)
    }

    fn solve_emissive(&self) -> Rgb {
        Rgb::new(0.0, 0.0, 0.0)
    }
}
