use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use graphics::{DrawState, Transformed};
use libraytracer::prelude::*;
use libraytracer::{RayTracer, Camera, SamplingConfig};
use libraytracer::entity::Sphere;
use libraytracer::brdf;
use std::f32;
use graphics;

type BrdfType = brdf::UnlimitedChromatic;

pub struct RayTracerApp {
    gl: GlGraphics,
    raytracer: RayTracer<BrdfType>,
    mouse_coord: (f64, f64),
    left_mouse_down: bool,
    right_mouse_down: bool,
    arrow_up_pressed: bool,
    arrow_down_pressed: bool,
    arrow_left_pressed: bool,
    arrow_right_pressed: bool,
    window_scale: f64,
}

impl RayTracerApp {
    pub fn new(opengl: OpenGL, dimensions: (usize, usize)) -> Self {
        let camera = Camera::new(dimensions, Vec3::new(0.0, 0.0, 0.0), f32::consts::FRAC_PI_2);
        let config = SamplingConfig::new(5, 1, 1.0);

        let mut raytracer = RayTracer::new(camera, config);

        // let brdf = brdf::Broken::new(Rgb::new(0.1, 0.1, 0.0), 0.9, 0.4, Rgb::new(0.0, 0.0, 0.0));
        // let brdf = brdf::Lambert::new(Rgb::new(0.1, 0.1, 0.0));
        let brdf = brdf::UnlimitedChromatic::new(Rgb::new(0.8, 0.4, 0.0),
                                                 Rgb::new(0.1, 0.1, 0.1),
                                                 0.2);
        let sphere = Sphere::new(Vec3::new(3.0, 0.0, 9.0), 2.0, brdf);
        raytracer.add_entity(sphere);

        // let brdf = brdf::Broken::new(Rgb::new(0.0, 0.2, 0.9), 0.2, 0.8, Rgb::new(0.0, 0.0, 0.0));
        // let brdf = brdf::Lambert::new(Rgb::new(0.0, 0.2, 0.9));
        let brdf = brdf::UnlimitedChromatic::new(Rgb::new(0.0, 0.02, 0.8),
                                                 Rgb::new(0.99, 0.9, 0.95),
                                                 0.4);
        let sphere = Sphere::new(Vec3::new(-3.0, 0.0, 7.0), 2.0, brdf);
        raytracer.add_entity(sphere);

        let brdf = brdf::UnlimitedChromatic::new(Rgb::new(0.7, 0.23, 0.12),
                                                 Rgb::new(0.0, 0.0, 0.0),
                                                 0.8);
        let ground = Sphere::new(Vec3::new(0.0, -1002.0, 8.0), 1000.0, brdf);
        raytracer.add_entity(ground);

        RayTracerApp {
            gl: GlGraphics::new(opengl),
            raytracer: raytracer,
            mouse_coord: (0.0, 0.0),
            left_mouse_down: false,
            right_mouse_down: false,
            arrow_up_pressed: false,
            arrow_down_pressed: false,
            arrow_left_pressed: false,
            arrow_right_pressed: false,
            window_scale: 5.0,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let scale = &mut self.window_scale;
        let raytracer = &self.raytracer;

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            let image = raytracer.image.to_rgba_image(1.0);
            let texture = Texture::from_image(&image, &TextureSettings::new());
            let image = graphics::Image::new();
            let w = args.width as f64 / 800.0;
            let h = args.height as f64 / 800.0;
            *scale = if w < h {
                w
            } else {
                h
            };
            image.draw(&texture,
                       &DrawState::default(),
                       c.transform.scale(*scale, *scale),
                       gl);
        });
    }

    pub fn update(&mut self, _: &UpdateArgs) {
        let invalidated = {
            let sphere = self.raytracer.entity_mut(0);
            if self.arrow_up_pressed {
                let new_position = sphere.position() + Vec3::new(0.0, 0.0, 0.1);
                sphere.set_position(new_position);
                true
            } else if self.arrow_down_pressed {
                let new_position = sphere.position() + Vec3::new(0.0, 0.0, -0.1);
                sphere.set_position(new_position);
                true
            } else if self.arrow_left_pressed {
                let new_position = sphere.position() + Vec3::new(-0.1, 0.0, 0.0);
                sphere.set_position(new_position);
                true
            } else if self.arrow_right_pressed {
                let new_position = sphere.position() + Vec3::new(0.1, 0.0, 0.0);
                sphere.set_position(new_position);
                true
            } else {
                false
            }
        };
        if invalidated {
            self.raytracer.clear_image();
        }
        self.raytracer.render();
    }

    fn handle_key_press(&mut self, key: Key, press: bool) {
        match key {
            Key::Up => {
                self.arrow_up_pressed = press;
            }
            Key::Down => {
                self.arrow_down_pressed = press;
            }
            Key::Left => {
                self.arrow_left_pressed = press;
            }
            Key::Right => {
                self.arrow_right_pressed = press;
            }
            _ => {}
        }
    }

    fn handle_mouse_click(&mut self, button: MouseButton, press: bool) {
        match button {
            MouseButton::Left => {
                self.left_mouse_down = press;
            }
            MouseButton::Right => {
                self.right_mouse_down = press;
            }
            _ => {}
        }
    }

    fn handle_mouse_move(&mut self, motion: Motion) {
        match motion {
            Motion::MouseCursor(x, y) => {
                self.mouse_coord = (x, y);
            }
            _ => {}
        }
    }

    pub fn handle_input(&mut self, input: &Input) {
        match input {
            &Input::Press(Button::Keyboard(x)) => self.handle_key_press(x, true),
            &Input::Press(Button::Mouse(x)) => self.handle_mouse_click(x, true),
            &Input::Release(Button::Keyboard(x)) => self.handle_key_press(x, false),
            &Input::Release(Button::Mouse(x)) => self.handle_mouse_click(x, false),
            &Input::Move(x) => self.handle_mouse_move(x),
            _ => {}
        }
    }
}
