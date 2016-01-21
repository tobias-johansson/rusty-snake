extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


pub struct App {
    gl: GlGraphics,
    time: f64,
    rotation: f64,
    offset: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GRAY:  [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let offset = self.offset;
        let (x, y) = ((args.width  / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(x + offset, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(GRAY, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt;
        self.rotation += 2.0 * args.dt;
        self.offset = 100.0 * (self.time * 10.0).sin();
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        time: 0.0,
        rotation: 0.0,
        offset: 0.0
    };

    for e in window.events() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::C {
                println!("Got a C!");
            }

            println!("Pressed keyboard key '{:?}'", key);
        };

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
