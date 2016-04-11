extern crate libraytracer;
extern crate image;
extern crate palette;
extern crate nalgebra;

use libraytracer::prelude::*;
use libraytracer::{RayTracer, Camera};
use libraytracer::entity::Sphere;
use libraytracer::raytracer::SamplingConfig;
use libraytracer::brdf;
use std::f32;

fn main() {
    let dimensions = (500, 500);
    let config = SamplingConfig::new(50, 1, 1.0);

    let camera = Camera::new(dimensions, Vec3::new(0.0, 0.0, 0.0), f32::consts::FRAC_PI_2);
    let mut raytracer = RayTracer::new(camera, config);

    // let brdf = brdf::Broken::new(Rgb::new(0.1, 0.1, 0.0), 0.9, 0.4, Rgb::new(0.0, 0.0, 0.0));
    // let brdf = brdf::Lambert::new(Rgb::new(0.4, 0.4, 0.0));
    // let brdf = brdf::UnlimitedChromatic::new(Rgb::new(0.4, 0.4, 0.0), Rgb::new(0.1, 0.1, 0.1), 0.2);
    let brdf = brdf::BlinnPhong::new(20.0);
    let sphere = Sphere::new(Vec3::new(3.0, 0.0, 9.0), 2.0, brdf);
    raytracer.add_entity(sphere);

    //let brdf = brdf::Broken::new(Rgb::new(0.0, 0.2, 0.9), 0.2, 0.8, Rgb::new(0.0, 0.0, 0.0));
    // let brdf = brdf::Lambert::new(Rgb::new(0.0, 0.5, 1.0));
    /*let brdf = brdf::UnlimitedChromatic::new(Rgb::new(0.0, 0.5, 1.0),
                                             Rgb::new(0.95, 0.95, 0.95),
                                             0.1);*/
    let brdf = brdf::BlinnPhong::new(80.0);
    let sphere = Sphere::new(Vec3::new(-3.0, 0.0, 7.0), 2.0, brdf);

    raytracer.add_entity(sphere);

    for _ in 0..1000 {
        raytracer.render();
    }
    
    let image = raytracer.image.to_rgba_image(2.2);

    image.save("rendered.png").expect("Couldn't save the image");
}
