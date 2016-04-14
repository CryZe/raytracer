extern crate libraytracer;
extern crate image;
extern crate palette;
extern crate nalgebra;

use libraytracer::prelude::*;
use libraytracer::{RayTracer, Camera};
use libraytracer::entity::{Sphere, Triangle, Mesh};
use libraytracer::raytracer::SamplingConfig;
use libraytracer::brdf;
use std::f32;

fn main() {
    let dimensions = (500, 500);
    let camera = Camera::new(dimensions, Vec3::new(0.0, 0.0, 0.0), f32::consts::FRAC_PI_2);
    let config = SamplingConfig::new(5, 1, 1.0);

    let mut raytracer = RayTracer::new(camera, config);

    //let brdf = brdf::Broken::new(Rgb::new(0.8, 0.4, 0.0), 0.1, 0.2, Rgb::new(0.0, 0.0, 0.0));
    // let brdf = brdf::Lambert::new(Rgb::new(0.1, 0.1, 0.0));
    /*let brdf = brdf::UnlimitedChromatic::new(Rgb::new(0.8, 0.4, 0.0), Rgb::new(0.1, 0.1, 0.1), 0.2);
    let sphere = Sphere::new(Vec3::new(3.0, 0.0, 9.0), 2.0, brdf);
    raytracer.add_entity(sphere);

    //let brdf = brdf::Broken::new(Rgb::new(0.0, 0.02, 0.8), 0.9, 0.4, Rgb::new(0.0, 0.0, 0.0));
    // let brdf = brdf::Lambert::new(Rgb::new(0.0, 0.2, 0.9));
     let brdf = brdf::UnlimitedChromatic::new(Rgb::new(0.0, 0.02, 0.8),
     Rgb::new(0.99, 0.9, 0.95),
     0.4);

    let sphere = Sphere::new(Vec3::new(-3.0, 0.0, 7.0), 2.0, brdf);
    raytracer.add_entity(sphere);*/

    //let brdf = brdf::Broken::new(Rgb::new(0.7, 0.23, 0.12), 0.0, 0.8, Rgb::new(0.0, 0.0, 0.0));
    let brdf = brdf::UnlimitedChromatic::new(Rgb::new(0.7, 0.23, 0.12),
    Rgb::new(0.0, 0.0, 0.0),
    0.8);
    let ground = Sphere::new(Vec3::new(0.0, -1002.0, 8.0), 1000.0, brdf);
    raytracer.add_entity(ground);

    let brdf = brdf::UnlimitedChromatic::new(Rgb::new(0.0, 0.8, 0.0), Rgb::new(0.1, 0.1, 0.1), 0.2);
    let triangle = Triangle::new(Vec3::new(0.0, 3.0, 9.0),
                                 Vec3::new(3.0, 3.0, 9.0),
                                 Vec3::new(3.0, 6.0, 9.0));
    let mesh = Mesh::new(vec![triangle], brdf);

    raytracer.add_entity(mesh);

    for _ in 0..500 {
        raytracer.render();
    }

    let image = raytracer.image.to_rgba_image(2.2);

    std::fs::remove_file("rendered.png").expect("Couldn't remove the image");
    image.save("rendered.png").expect("Couldn't save the image");
}
