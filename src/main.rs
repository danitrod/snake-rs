mod constants;
mod drawing;
mod random;

use constants::*;
use drawing::drawing;
use ggez::{
    self,
    conf::{FullscreenType, NumSamples, WindowMode, WindowSetup},
    event::{self, KeyCode, KeyMods},
    graphics::{self, Color},
    input::keyboard,
    Context, GameResult,
};
use random::Random;
use std::env;
use std::path;
use std::time::Instant;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

struct MainState {
    snake: Vec<(f32, f32)>,
    snake_size: usize,
    direction: Direction,
    key_pressed: Direction,
    last_tick: Instant,
    food_pos: (f32, f32),
}

impl MainState {
    fn new(rng: Random) -> GameResult<MainState> {
        let s = MainState {
            snake: vec![SCREEN_CENTER],
            snake_size: INITIAL_SNAKE_SIZE,
            direction: Right,
            key_pressed: Right,
            last_tick: Instant::now(),
            food_pos: rng.random_food(NUM_COLS, NUM_ROWS, CELL_SIZE, None),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let now = Instant::now();
        if now.duration_since(self.last_tick) > TICK_INTERVAL {
            if self.snake.len() > 1 {
                self.snake = self.snake[0..self.snake_size - 1].to_vec();
            }
            if self.snake[0].0 > SCREEN_WIDTH {
                println!("Setting {:?} to zero", self.snake);
                self.snake.insert(0, (0., self.snake[0].1));
                println!("{:?}", self.snake);
            } else if self.snake[0].0 < 0. {
                self.snake.insert(0, (SCREEN_WIDTH, self.snake[0].1));
            } else if self.snake[0].1 > SCREEN_HEIGHT {
                self.snake.insert(0, (self.snake[0].0, 0.));
            } else if self.snake[0].1 < 0. {
                self.snake.insert(0, (self.snake[0].0, SCREEN_HEIGHT));
            } else {
                match self.direction {
                    Up => self
                        .snake
                        .insert(0, (self.snake[0].0, self.snake[0].1 - CELL_SIZE)),
                    Down => self
                        .snake
                        .insert(0, (self.snake[0].0, self.snake[0].1 + CELL_SIZE)),
                    Left => self
                        .snake
                        .insert(0, (self.snake[0].0 - CELL_SIZE, self.snake[0].1)),
                    Right => self
                        .snake
                        .insert(0, (self.snake[0].0 + CELL_SIZE, self.snake[0].1)),
                }
            }
            self.food_pos.1 += 1.;
            self.last_tick = now;
        }
        if self.direction != self.key_pressed {
            match self.key_pressed {
                Up => {
                    if !keyboard::is_key_pressed(ctx, KeyCode::Up) {
                        self.direction = Up;
                        self.snake
                            .insert(0, (self.snake[0].0, self.snake[0].1 - CELL_SIZE));
                    }
                }
                Down => {
                    if !keyboard::is_key_pressed(ctx, KeyCode::Down) {
                        self.direction = Down;
                        self.snake
                            .insert(0, (self.snake[0].0, self.snake[0].1 + CELL_SIZE));
                    }
                }
                Left => {
                    if !keyboard::is_key_pressed(ctx, KeyCode::Left) {
                        self.direction = Left;
                        self.snake
                            .insert(0, (self.snake[0].0 - CELL_SIZE, self.snake[0].1));
                    }
                }
                Right => {
                    if !keyboard::is_key_pressed(ctx, KeyCode::Right) {
                        self.direction = Right;
                        self.snake
                            .insert(0, (self.snake[0].0 + CELL_SIZE, self.snake[0].1));
                    }
                }
            }
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Up)
            && self.direction != Up
            && self.direction != Down
        {
            self.key_pressed = Up;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down)
            && self.direction != Up
            && self.direction != Down
        {
            self.key_pressed = Down;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left)
            && self.direction != Left
            && self.direction != Right
        {
            self.key_pressed = Left;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Right)
            && self.direction != Left
            && self.direction != Right
        {
            self.key_pressed = Right;
        }

        // println!("snake updated: {:?} (sw: {})", self.snake, SCREEN_WIDTH);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(144, 144, 255).into());
        drawing(ctx, &self.snake, self.food_pos)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
        match key {
            // Quit if Shift+Ctrl+Q is pressed.
            KeyCode::Q => {
                if mods.contains(KeyMods::SHIFT) && mods.contains(KeyMods::CTRL) {
                    println!("Terminating!");
                    event::quit(ctx);
                } else if mods.contains(KeyMods::SHIFT) || mods.contains(KeyMods::CTRL) {
                    println!("You need to hold both Shift and Control to quit.");
                } else {
                    println!("Now you're not even trying!");
                }
            }
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    let rng = Random::new();
    let mut cb = ggez::ContextBuilder::new("snake.rs", "danitrod");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }
    cb = cb.window_mode(WindowMode {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    });
    cb = cb.window_setup(WindowSetup {
        title: "Snake".to_owned(),
        icon: path::Path::new("/thumbnail.jpg")
            .to_str()
            .unwrap()
            .to_owned(),
        samples: NumSamples::One,
        srgb: true,
        vsync: true,
    });
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(rng)?;
    event::run(ctx, event_loop, state)
}
