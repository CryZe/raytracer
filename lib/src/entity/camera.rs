use Vec3;
use ray::Ray;
use nalgebra as na;
use std::f32;
use rand;
use rand::Rng;

pub struct Camera {
    pub dimensions: (usize, usize),
    pub position: Vec3,
    pub field_of_view: f32,
}

impl Camera {
    pub fn new(dimensions: (usize, usize), position: Vec3, field_of_view: f32) -> Self {
        Camera {
            dimensions: dimensions,
            position: position,
            field_of_view: field_of_view,
        }
    }

    pub fn get_ray_for_coordinate(&self, coord: (usize, usize)) -> Ray {
        let (width, height) = self.dimensions;
        let (width, height) = (width as f32, height as f32);
        let (x, y) = coord;
        let (x, y) = (x as f32, y as f32);

        let mut rng = rand::thread_rng();
        let x = x + rng.gen_range(-0.5, 0.5);
        let y = y + rng.gen_range(-0.5, 0.5);

        let origin = self.position;

        let d = 1.0 / f32::tan(self.field_of_view / 2.0);

        let x = 2.0 * x / width - 1.0;
        let y = -2.0 * y / height + 1.0;

        let x = (width / height) * x / d;
        let y = y / d;

        let v = Vec3::new(x, y, 1.0);
        let direction = na::normalize(&v);

        Ray::new(origin, direction)
    }
}
