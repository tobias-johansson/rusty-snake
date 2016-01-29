extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

const TICK_LENGTH: f64 = 0.5;

// To be able to copy the enumeration type (not a ref) need this wierd stuff
#[derive(Clone)]
#[derive(Copy)]
enum Direction {
    Up, Down, Left, Right
}

pub struct App {
    gl: GlGraphics,
    time: f64,
    tick_time: f64,
    x: f64,
    y: f64,
    direction: Direction,
    next_direction_set: bool
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GRAY:  [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 10.0);
        let (x, y) = (self.x, self.y);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y)
                                       .trans(-5.0, -5.0);

            rectangle(GRAY, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt;
        self.tick_time += args.dt;
        if self.tick_time >= TICK_LENGTH {
            self.tick_time = 0.0;
            self.tick();
        }
    }

    fn tick(&mut self) {
        match self.direction {
            Direction::Up    => self.y -= 10.0,
            Direction::Down  => self.y += 10.0,
            Direction::Left  => self.x -= 10.0,
            Direction::Right => self.x += 10.0,
        }
        self.next_direction_set = false;
    }

    fn key(&mut self, key_direction: Direction) {
        if !self.next_direction_set {
            self.direction =
                match (self.direction, key_direction) {
                    (Direction::Up,    Direction::Down)  => Direction::Up,
                    (Direction::Down,  Direction::Up)    => Direction::Down,
                    (Direction::Left,  Direction::Right) => Direction::Left,
                    (Direction::Right, Direction::Left)  => Direction::Right,
                    _                                    => key_direction,
                };
            self.next_direction_set = true;
        } 
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let window: Window = WindowSettings::new(
            "rusty-snake",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        time:      0.0,
        tick_time: 0.0,
        x:         0.0,
        y:         0.0,
        direction: Direction::Right,
        next_direction_set: false
    };

    for e in window.events() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up   => app.key(Direction::Up),
                Key::Down => app.key(Direction::Down),
                Key::Left => app.key(Direction::Left),
                Key::Right => app.key(Direction::Right),
                _ => ()
            }
        };

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
