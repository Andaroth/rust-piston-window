extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::*;

static SOFTWARE_TITLE: &str = "PistOwl";

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
        const DARK_GREEN: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        // containers sizing
        let window_height = args.window_size[1] as f64 + 24.;
        let window_width = args.window_size[0] as f64 + 24.;
        let mut menu_width = &args.window_size[0] / 4.;
        if menu_width <= 240. { menu_width = 240.; } // min-size
        let padding_size = 12.;
        let container_width = window_width - menu_width - padding_size;
        // containers position
        let menu_box_shape: [f64; 4] = [0., 0., menu_width, window_height];
        let content_box_shape: [f64; 4] = [menu_width + padding_size, 0., container_width, window_height];

        // render
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl); // background
            let transform = c.transform.trans(-25.0, -25.0); // relative to self
            // Draw boxes
            rectangle(DARK_GREEN, menu_box_shape, transform, gl);
            rectangle(DARK_GREEN, content_box_shape, transform, gl);

            let texts = [ // name, X, Y
                (SOFTWARE_TITLE, menu_width + (padding_size * 2.), padding_size),
                ("lorem", padding_size, padding_size + (24. * 0.)),
                ("ipsum", padding_size, padding_size + (24. * 1.)),
                ("dolor", padding_size, padding_size + (24. * 2.)),
                ("sit", padding_size, padding_size + (24. * 3.)),
                ("amet", padding_size, padding_size + (24. * 4.))
            ];

            let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets/fonts").unwrap();
            let mut glyphs = Glyphs::new( // generate text
                assets.join("FreeMono.ttf"), // font
                self.view.factory.clone() // factory
            ).unwrap();
            for elem in texts {
                let (name, x, y): (&str, f32, f32) = (elem.0 as f32,elem.1 as f32,elem.2 as f32);
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                    name,
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(x, y),
                    gl
                );
            }
        });
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new(SOFTWARE_TITLE, [640, 480])
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

        // if let Some(args) = e.update_args() {
        //     app.update(&args);
        // }
    }
}