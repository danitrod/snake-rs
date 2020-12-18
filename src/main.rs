mod constants;
mod drawing;
mod random;

use constants::{dynamics::*, graphics::BG_COLOR, grid::*};
use drawing::drawing;
use ggez::{
    self,
    conf::{FullscreenType, NumSamples, WindowMode, WindowSetup},
    event::{self, KeyCode, KeyMods},
    graphics::{self, Color, Font, Scale},
    input::keyboard,
    nalgebra as na, Context, GameResult,
};
use random::Random;
use std::env;
use std::path;
use std::time::{Duration, Instant};

#[derive(PartialEq)]
pub enum Direction {
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
    tick: Duration,
    food_pos: (f32, f32),
    rng: Random,
    game_mode: u8,
}

impl MainState {
    fn new(rng: Random) -> GameResult<MainState> {
        let s = MainState {
            snake: vec![SCREEN_CENTER],
            snake_size: INITIAL_SNAKE_SIZE,
            direction: Right,
            key_pressed: Right,
            tick: INITIAL_TICK_INTERVAL,
            last_tick: Instant::now(),
            food_pos: rng.random_food(NUM_COLS, NUM_ROWS, CELL_SIZE, None),
            rng,
            game_mode: 0,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let now = Instant::now();
        if now.duration_since(self.last_tick) > self.tick {
            if self.snake[0].0 >= SCREEN_WIDTH {
                self.snake.insert(0, (0., self.snake[0].1));
            } else if self.snake[0].0 < 0. {
                self.snake
                    .insert(0, (SCREEN_WIDTH - CELL_SIZE, self.snake[0].1));
            } else if self.snake[0].1 >= SCREEN_HEIGHT {
                self.snake.insert(0, (self.snake[0].0, 0.));
            } else if self.snake[0].1 < 0. {
                self.snake
                    .insert(0, (self.snake[0].0, SCREEN_HEIGHT - CELL_SIZE));
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
            self.last_tick = now;
        }
        if self.direction != self.key_pressed {
            match self.key_pressed {
                Up => {
                    if !keyboard::is_key_pressed(ctx, KeyCode::Up) {
                        self.direction = Up;
                        self.snake
                            .insert(0, (self.snake[0].0, self.snake[0].1 - CELL_SIZE));
                        self.last_tick = Instant::now();
                    }
                }
                Down => {
                    if !keyboard::is_key_pressed(ctx, KeyCode::Down) {
                        self.direction = Down;
                        self.snake
                            .insert(0, (self.snake[0].0, self.snake[0].1 + CELL_SIZE));
                        self.last_tick = Instant::now();
                    }
                }
                Left => {
                    if !keyboard::is_key_pressed(ctx, KeyCode::Left) {
                        self.direction = Left;
                        self.snake
                            .insert(0, (self.snake[0].0 - CELL_SIZE, self.snake[0].1));
                        self.last_tick = Instant::now();
                    }
                }
                Right => {
                    if !keyboard::is_key_pressed(ctx, KeyCode::Right) {
                        self.direction = Right;
                        self.snake
                            .insert(0, (self.snake[0].0 + CELL_SIZE, self.snake[0].1));
                        self.last_tick = Instant::now();
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

        // Snake eats food
        if self.snake[0].0 == self.food_pos.0 && self.snake[0].1 == self.food_pos.1 {
            self.snake_size += 1;
            self.food_pos = self
                .rng
                .random_food(NUM_COLS, NUM_ROWS, CELL_SIZE, Some(&self.snake));
            // Accelerate game
            if (self.snake_size - INITIAL_SNAKE_SIZE) % ACCELERATION_GAP == 0 {
                self.tick = self.tick - ACCELERATION_VALUE;
            }
        }

        // Kill snake if it hits body
        if self.snake.len() >= self.snake_size {
            if self.snake[1..self.snake_size].contains(&self.snake[0]) {
                self.game_mode = 1;
            }
        }

        // Trim snake to its size
        if self.snake.len() > self.snake_size {
            self.snake = self.snake[0..self.snake_size].to_vec();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match self.game_mode {
            0 => {
                let (r, g, b) = BG_COLOR;
                graphics::clear(ctx, Color::from_rgb(r, g, b).into());
                drawing(
                    ctx,
                    &self.snake,
                    self.food_pos,
                    &self.direction,
                    self.snake_size - INITIAL_SNAKE_SIZE,
                )?;
                graphics::present(ctx)?;
                Ok(())
            }
            1 => {
                graphics::clear(ctx, Color::from_rgb(90, 20, 30).into());
                let mut score_txt = graphics::Text::new(format!(
                    "You lost. Score: {}",
                    self.snake_size - INITIAL_SNAKE_SIZE
                ));
                let score_txt = score_txt.set_font(Font::default(), Scale { x: 32., y: 32. });
                let w = score_txt.width(ctx) as f32;
                let h = score_txt.height(ctx) as f32;
                graphics::draw(
                    ctx,
                    score_txt,
                    (na::Point2::new(
                        SCREEN_CENTER.0 - w / 2.,
                        SCREEN_CENTER.1 - h / 2.,
                    ),),
                )?;
                graphics::present(ctx)?;
                Ok(())
            }
            _ => panic!(),
        }
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
