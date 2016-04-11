extern crate libraytracer;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate palette;
extern crate nalgebra;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;

mod app;
use app::RayTracerApp;

fn main() {
    let opengl = OpenGL::V2_1;

    let dimensions = (800, 800);
    let (nx, ny) = dimensions;

    let mut window: Window = WindowSettings::new("Raytracer", [1 * nx as u32, 1 * ny as u32])
                                 .opengl(opengl)
                                 .exit_on_esc(true)
                                 .build()
                                 .unwrap();

    let mut app = RayTracerApp::new(opengl, dimensions);

    let mut event_loop = window.events();
    event_loop.set_max_fps(20);
    event_loop.set_ups(20);

    while let Some(e) = event_loop.next(&mut window) {
        match e {
            Event::Render(r) => app.render(&r),
            Event::Update(u) => app.update(&u),
            Event::Input(i) => app.handle_input(&i),
            _ => {}
        }
    }
}
