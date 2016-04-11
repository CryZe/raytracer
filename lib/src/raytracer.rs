use prelude::*;
use image::Image;
use entity::camera::Camera;
use scoped_threadpool::Pool;
use num_cpus;
use ray::Ray;
use nalgebra as na;
use rand;
use rand::{ThreadRng, Rng};
use std::f32::consts::PI;
use std::f32;

pub struct SamplingConfig {
    max_depth: usize,
    starting_samples: usize,
    scale_factor: f32,
}

impl SamplingConfig {
    pub fn new(max_depth: usize, starting_samples: usize, scale_factor: f32) -> Self {
        SamplingConfig {
            max_depth: max_depth,
            starting_samples: starting_samples,
            scale_factor: scale_factor,
        }
    }

    pub fn sample_count(&self, depth: usize) -> usize {
        f32::round(self.starting_samples as f32 * f32::powi(self.scale_factor, -(depth as i32))) as usize
    }
}

pub struct RayTracer<BrdfType: Brdf + 'static> {
    pub image: Image,
    entities: Vec<Box<Entity<BrdfType = BrdfType> + Sync>>,
    camera: Camera,
    thread_pool: Pool,
    sampling_config: SamplingConfig,
    frames_rendered: u64,
}

fn random_dir(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let x1 = rng.gen_range(-1.0, 1.0);
        let x2 = rng.gen_range(-1.0, 1.0);
        let x1sqr = x1 * x1;
        let x2sqr = x2 * x2;
        let sum = x1sqr + x2sqr;
        if sum >= 1.0 {
            continue;
        }
        let sqrt2 = 2.0 * f32::sqrt(1.0 - x1sqr - x2sqr);
        let x = x1 * sqrt2;
        let y = x2 * sqrt2;
        let z = 1.0 - 2.0 * sum;
        return Vec3::new(x, y, z);
    }
}

impl<BrdfType: Brdf + 'static> RayTracer<BrdfType> {
    pub fn new(camera: Camera, sampling_config: SamplingConfig) -> Self {
        RayTracer {
            image: Image::new(camera.dimensions),
            entities: Vec::new(),
            camera: camera,
            thread_pool: Pool::new(num_cpus::get() as u32),
            sampling_config: sampling_config,
            frames_rendered: 0,
        }
    }

    pub fn entity_mut(&mut self, index: usize) -> &mut Entity<BrdfType = BrdfType> {
        self.entities[index].as_mut()
    }

    pub fn add_entity<T: Entity<BrdfType = BrdfType> + 'static + Sync>(&mut self, entity: T) {
        self.entities.push(Box::new(entity))
    }

    fn trace(entities: &[Box<Entity<BrdfType = BrdfType> + Sync>],
             ray: &Ray,
             depth: usize,
             config: &SamplingConfig)
             -> Rgb {
        for entity in entities {
            if let Some(collision) = entity.collides_with(&ray) {
                let view_direction = ray.direction * -1.0;
                let mut brightness = Rgb::new(0.0, 0.0, 0.0);
                let mut count = 0;

                if depth < config.max_depth {
                    count = config.sample_count(depth);
                    let mut rng = rand::thread_rng();
                    for _ in 0..count {
                        let mut direction = random_dir(&mut rng);

                        let mut n_dot_l = na::dot(&collision.normal, &direction);

                        if n_dot_l < 0.0 {
                            direction = direction * -1.0;
                            n_dot_l = na::dot(&collision.normal, &direction);
                        }

                        let new_ray = Ray::new(collision.position + collision.normal * 0.001, direction);
                        let ray_brightness = Self::trace(entities, &new_ray, depth + 1, config);
                        let mut brdf = collision.brdf
                                                .solve(direction,
                                                       collision.normal,
                                                       view_direction);
                        //brdf = brdf.saturate().fix_nan();
                        brightness = brightness + brdf * ray_brightness * n_dot_l;
                    }
                }

                if count > 0 {
                    brightness = brightness / count as f32;
                }

                brightness = brightness * PI + collision.brdf.solve_emissive();

                return brightness;

                // return Rgb::new(0.5 * collision.normal.x + 0.5, 0.5 * collision.normal.y + 0.5, 0.5 * collision.normal.z + 0.5);
            }
        }
        let sun_dir = na::normalize(&Vec3::new(0.75, 0.75, -0.75));
        if na::dot(&ray.direction, &sun_dir) > 0.995 {
            Rgb::new(100.0, 100.0, 100.0)
        } else {
            let t = 0.5 * ray.direction.y + 1.0;
            Rgb::new(1.0, 1.0, 1.0) * (1.0 - t) + Rgb::new(0.5, 0.7, 1.0) * t
        }
    }

    pub fn clear_image(&mut self) {
        self.frames_rendered = 0;
    }

    pub fn render(&mut self) {
        let image = &mut self.image;
        let thread_count = self.thread_pool.thread_count() as usize;
        let camera = &self.camera;
        let entities = &self.entities;
        let sampling_config = &self.sampling_config;

        let old_frames_rendered = self.frames_rendered as f32;
        let new_frames_rendered = (self.frames_rendered + 1) as f32;
        let factor_new = 1.0 / new_frames_rendered;
        let factor_old = old_frames_rendered / new_frames_rendered;
        self.frames_rendered = self.frames_rendered + 1;

        self.thread_pool.scoped(|scope| {
            for chunk in image.chunks_mut(thread_count) {
                scope.execute(move || {
                    for (coord, cell) in chunk {
                        let ray = camera.get_ray_for_coordinate(coord);
                        *cell = *cell * factor_old + Self::trace(entities, &ray, 0, sampling_config) * factor_new;
                    }
                });
            }
        });
    }
}
