use prelude::*;
use nalgebra as na;
use std::f32;
use std::f32::consts::PI;

pub struct UnlimitedChromatic {
    kd: Rgb,
    ks: Rgb,
    roughness: f32,
}

impl UnlimitedChromatic {
    pub fn new(kd: Rgb, ks: Rgb, roughness: f32) -> Self {
        UnlimitedChromatic {
            kd: kd,
            ks: ks,
            roughness: roughness,
        }
    }
}

const WHITE: Rgb = Rgb {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

impl Brdf for UnlimitedChromatic {
    fn solve(&self, l: Vec3, n: Vec3, v: Vec3) -> Rgb {
        let h = na::normalize(&(l + v));

        let cf0 = WHITE - self.ks;

        let n_dot_h = na::dot(&n, &h);
        let l_dot_h = na::dot(&l, &h);
        let n_dot_l = na::dot(&n, &l);
        let n_dot_v = na::dot(&n, &v);

        let roughness2 = self.roughness * self.roughness;
        let roughness_sqrt = f32::sqrt(self.roughness);

        let c_roughness_sqrt = 1.0 - roughness_sqrt;

        let diffuse_pre_factor = cf0 / PI;

        let diffuse_a = diffuse_pre_factor * (0.85 * c_roughness_sqrt);
        let diffuse_b = diffuse_pre_factor * (0.1 * c_roughness_sqrt + 1.0 * roughness_sqrt);

        let roughness2_sub1 = roughness2 - 1.0;

        let n_dot_v2 = n_dot_v * n_dot_v;
        let geo_denominator = 1.0 + f32::sqrt(1.0 + roughness2 * (1.0 - n_dot_v2) / n_dot_v2);

        let specular_nominator = roughness2 / (PI * n_dot_v * geo_denominator);

        // =================================================================

        let cl_dot_h = 1.0 - l_dot_h;
        let cl_dot_h2 = cl_dot_h * cl_dot_h;
        let cl_dot_h5 = cl_dot_h2 * cl_dot_h2 * cl_dot_h;
        let fresnel = self.ks + cf0 * cl_dot_h5;

        let ggx_sqrt = n_dot_h * n_dot_h * roughness2_sub1 + 1.0;
        let ggx = ggx_sqrt * ggx_sqrt;

        let n_dot_l2 = n_dot_l * n_dot_l;
        let geometric_term = 1.0 + f32::sqrt(1.0 + roughness2 * (1.0 - n_dot_l2) / n_dot_l2);

        let specular = fresnel * (specular_nominator / (ggx * n_dot_l * geometric_term));

        let diffuse = diffuse_a * n_dot_l + diffuse_b;

        let result = self.kd * diffuse + specular;

        result
    }

    fn solve_emissive(&self) -> Rgb {
        Rgb::new(0.0, 0.0, 0.0)
    }
}
