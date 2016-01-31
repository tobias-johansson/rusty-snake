extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::vec::Vec;

const TICK_LENGTH: f64 = 0.25;

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
    snake: Vec<SnakePart>,
    next_direction_set: bool
}

pub struct SnakePart {
    x: f64,
    y: f64,
    direction: Direction,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        
        use graphics::*;

        const GRAY:  [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 10.0);

        let parts = self.snake.iter();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for part in parts {
                let (x, y) = (part.x, part.y);
                let transform = c.transform.trans(x, y)
                                       .trans(-5.0, -5.0);

                rectangle(GRAY, square, transform, gl);
            }
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
        for part in self.snake.iter_mut() {
            match part.direction {
                Direction::Up    => part.y -= 10.0,
                Direction::Down  => part.y += 10.0,
                Direction::Left  => part.x -= 10.0,
                Direction::Right => part.x += 10.0,
            }
        }
        // Update tail by copying the position of the snake part in front of it
        let snake_length = self.snake.len();
        for i in 0..snake_length-1 {
            self.snake[i].direction = self.snake[i+1].direction;
        }
        self.next_direction_set = false;
    }

    fn key(&mut self, key_direction: Direction) {
        if !self.next_direction_set {
            let snake_length = self.snake.len();
            
            self.snake[snake_length-1].direction =
                    match (self.snake[snake_length-1].direction, key_direction) {
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
        snake: vec![
                    SnakePart{x: 20.0, y: 50.0, direction: Direction::Right},
                    SnakePart{x: 30.0, y: 50.0, direction: Direction::Right},
                    SnakePart{x: 40.0, y: 50.0, direction: Direction::Right}, 
                    SnakePart{x: 50.0, y: 50.0, direction: Direction::Right},
                    SnakePart{x: 60.0, y: 50.0, direction: Direction::Right}],
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
