extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    selected_menu: u8, // cursor in left menu
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        // helpers
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        // containers sizing
        let window_height = args.window_size[1] as f64 + 24.;
        let window_width = args.window_size[0] as f64 + 24.;
        let mut menu_width = &args.window_size[0] / 4.;
        if menu_width <= 240. { menu_width = 240.; } // min-size
        let container_margin = 24.;
        let container_width = window_width - menu_width - container_margin;
        // containers position
        let menu_box_shape: [f64; 4] = [0., 0., menu_width, window_height];
        let content_box_shape: [f64; 4] = [menu_width + (container_margin / 2.), 0., container_width, window_height];
        // render
        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl); // background
            let menu_transform = c.transform // piston transform
                .trans(0., 0.) // absolute
                .trans(-25.0, -25.0); // relative to self
            let content_transform = c.transform // piston transform
                .trans(0., 0.) // absolute
                .trans(-25.0, -25.0); // relative to self
            // Draw boxes
            rectangle(GREEN, menu_box_shape, menu_transform, gl);
            rectangle(GREEN, content_box_shape, content_transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("PistOwl", [640, 480])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        selected_menu: 0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}