use {Rgb, Vec3};

pub trait Brdf {
    fn solve(&self, l: Vec3, n: Vec3, v: Vec3) -> Rgb;
    fn solve_emissive(&self) -> Rgb;
}

pub mod broken;
pub mod lambert;
pub mod unlimited_chromatic;
pub mod blinn_phong;

pub use self::broken::Broken;
pub use self::lambert::Lambert;
pub use self::unlimited_chromatic::UnlimitedChromatic;
pub use self::blinn_phong::BlinnPhong;