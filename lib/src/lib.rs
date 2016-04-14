extern crate nalgebra;
extern crate palette;
extern crate num_cpus;
extern crate scoped_threadpool;
extern crate image as im;
extern crate rand;

pub mod entity;
mod image;
pub mod ray;
pub mod raytracer;
pub mod collision;
pub mod brdf;
pub mod clamp;

pub use entity::Entity;
pub use ray::Ray;
pub use raytracer::{RayTracer, SamplingConfig};
pub use collision::Collision;
pub use brdf::Brdf;
pub use entity::camera::Camera;

pub type Vec3 = nalgebra::Vec3<f32>;
pub type Rgb = palette::Rgb<f32>;
pub type RgbaImage = im::RgbaImage;

pub mod prelude {
    pub use entity::Entity;
    pub use brdf::Brdf;
    pub use {Vec3, Rgb};
    pub use clamp::Clamp;
}
