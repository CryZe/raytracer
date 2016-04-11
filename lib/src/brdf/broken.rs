use {Rgb, Vec3};
use nalgebra as na;
use std::f32;
use super::Brdf;

pub struct Broken {
    albedo: Rgb,
    reflectivity: f32,
    roughness: f32,
    emissive: Rgb,
}

impl Broken {
    pub fn new(albedo: Rgb, reflectivity: f32, roughness: f32, emissive: Rgb) -> Self {
        Broken {
            albedo: albedo,
            reflectivity: reflectivity,
            roughness: roughness,
            emissive: emissive,
        }
    }
}

impl Brdf for Broken {
    fn solve(&self, l: Vec3, n: Vec3, v: Vec3) -> Rgb {
        let h = na::normalize(&(l + v));

        let n_dot_v = f32::min(1.0, f32::max(0.0, na::dot(&l, &v)));
        let n_dot_l = f32::min(1.0, f32::max(0.0, na::dot(&n, &l)));
        let l_dot_h = na::dot(&l, &h);

        let n_dot_v2 = n_dot_v * n_dot_v;
        let l_dot_h2 = l_dot_h * l_dot_h;

        let roughness_times_2 = 2.0 * self.roughness;
        let c_reflectivity = 1.0 - self.reflectivity;
        let c_n_dot_v = 1.0 - n_dot_v;
        let c_n_dot_v2 = c_n_dot_v * c_n_dot_v;
        let c_n_dot_v5 = c_n_dot_v2 * c_n_dot_v2 * c_n_dot_v;
        let transmittance_a = c_reflectivity;
        let transmittance_b = c_reflectivity * c_n_dot_v5;

        let c_n_dot_l = 1.0 - n_dot_l;
        let c_n_dot_l2 = c_n_dot_l * c_n_dot_l;
        let c_n_dot_l5 = c_n_dot_l2 * c_n_dot_l2 * c_n_dot_l;

        let fd90 = l_dot_h2 * roughness_times_2 - 0.5;
        let transmittance = (1.0 + fd90 * c_n_dot_l5) * (transmittance_a + fd90 * transmittance_b);

        let diffuse = self.albedo * transmittance;

        if n_dot_v > 0.0 {
            let n_dot_h = na::dot(&n, &h);

            let c_l_dot_h = 1.0 - l_dot_h;
            let c_l_dot_h2 = c_l_dot_h * c_l_dot_h;
            let c_l_dot_h5 = c_l_dot_h2 * c_l_dot_h2 * c_l_dot_h;

            let n_dot_l2 = n_dot_l * n_dot_l;

            let ag = self.roughness / 2.0 + 0.5;
            let ag2 = ag * ag;
            let a2 = self.roughness * self.roughness;
            let a4 = a2 * a2;

            let c0 = a4 /
                     (n_dot_v + n_dot_v * f32::sqrt(1.0 + ag2 * ((1.0 - n_dot_v2) / n_dot_v2)));
            let c1 = self.reflectivity * c0;
            let c2 = c0 - c1;
            let c3 = a4 - 1.0;

            let f = c1 + c2 * c_l_dot_h5;
            let g = 1.0 + f32::sqrt(1.0 + ag2 * ((1.0 - n_dot_l2) / n_dot_l2));
            let d = (n_dot_h * n_dot_h) * c3 + 1.0;

            let specular = f / (d * d * g);

            (diffuse + specular) / f32::consts::PI
        } else {
            diffuse / f32::consts::PI
        }
    }

    fn solve_emissive(&self) -> Rgb {
        self.emissive
    }
}
