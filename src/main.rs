extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate nalgebra;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::vec::Vec;
use nalgebra::{Vec4};

const TICK_LENGTH: f64 = 0.25;
const STEP_SIZE:   f64 = 10.0;

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
    x: i32,
    y: i32,
    direction: Direction,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let gray: Vec4<f32> = Vec4::new(0.5, 0.5, 0.5, 1.0);
        let rust: Vec4<f32> = Vec4::new(0.8, 0.37, 0.14, 0.7);

        let square = rectangle::square(0.0, 0.0, 10.0);

        let snake_length_f32 = self.snake.len() as f32;
        let parts = self.snake.iter();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);	

            let mut i = 0.0;
            let mut alpha: f32;
            for part in parts {
                let (x, y) = (part.x as f64 * STEP_SIZE,
                              part.y as f64 * STEP_SIZE);
                
                // Using linear equation to transition color from gray to rust
                alpha = i/snake_length_f32;
                let color = gray*(1.0-alpha)+rust*alpha;
                let color_array = [color.x, color.y, color.z, color.w];
                
                let transform = c.transform.trans(x, y)
                                           .trans(-5.0, -5.0);

                rectangle(color_array, square, transform, gl);

                i=i+1.0;
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
                Direction::Up    => part.y -= 1,
                Direction::Down  => part.y += 1,
                Direction::Left  => part.x -= 1,
                Direction::Right => part.x += 1,
            }
        }
        // Update tail by copying the direction of the snake part in front of it
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
                    SnakePart{x: 2, y: 5, direction: Direction::Right},
                    SnakePart{x: 3, y: 5, direction: Direction::Right},
                    SnakePart{x: 4, y: 5, direction: Direction::Right},
                    SnakePart{x: 5, y: 5, direction: Direction::Right},
                    SnakePart{x: 6, y: 5, direction: Direction::Right}],
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
