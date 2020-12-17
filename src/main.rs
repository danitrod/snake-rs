mod random;

use ggez::{
    self,
    conf::{FullscreenType, WindowMode},
    event::{self, KeyCode, KeyMods},
    graphics::{self, Color},
    // input::keyboard,
    nalgebra as na,
    Context,
    GameResult,
};
use random::Random;
use std::time::{Duration, Instant};

const INITIAL_SNAKE_SIZE: usize = 3;
const CELL_SIZE: f32 = 16.;
const NUM_ROWS: usize = 48;
const NUM_COLS: usize = 64;
const SCREEN_WIDTH: f32 = NUM_COLS as f32 * CELL_SIZE;
const SCREEN_HEIGHT: f32 = NUM_ROWS as f32 * CELL_SIZE;
const SCREEN_CENTER: (f32, f32) = (SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2.);

struct MainState {
    snake: Vec<(f32, f32)>,
    snake_size: usize,
    last_tick: Instant,
    food_pos: (f32, f32),
}

impl MainState {
    fn new(rng: Random) -> GameResult<MainState> {
        let s = MainState {
            snake: vec![SCREEN_CENTER],
            snake_size: INITIAL_SNAKE_SIZE,
            last_tick: Instant::now(),
            food_pos: rng.random_food(NUM_COLS, NUM_ROWS),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let now = Instant::now();
        if now.duration_since(self.last_tick) > Duration::from_millis(300) {
            if self.snake.len() > 1 {
                self.snake = self.snake[0..self.snake_size - 1].to_vec();
            }
            self.snake
                .insert(0, (self.snake[0].0 + CELL_SIZE, self.snake[0].1));
            self.food_pos.1 += 1.;
            self.last_tick = now;
        }

        // if keyboard::is_key_pressed(ctx, KeyCode::X) {
        //     self.pos_x = self.pos_x % 800.0 + 1.0;
        // }
        // if keyboard::is_key_pressed(ctx, KeyCode::Y) {
        //     self.pos_y = self.pos_y % 800.0 + 1.0;
        // }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(144, 144, 255).into());

        let mut iter_snake = self.snake.iter();
        let head = iter_snake.next().expect("No snake head");
        let head_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(head.0, head.1, CELL_SIZE, CELL_SIZE),
            Color::from_rgb(90, 255, 90),
        )?;
        graphics::draw(ctx, &head_rect, (na::Point2::new(0., 0.),))?;

        for cell in iter_snake {
            let cell_rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(cell.0, cell.1, CELL_SIZE, CELL_SIZE),
                Color::from_rgb(90, 255, 90),
            )?;
            graphics::draw(ctx, &cell_rect, (na::Point2::new(0., 0.),))?;
        }

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
    let cb = ggez::ContextBuilder::new("snake.rs", "danitrod").window_mode(WindowMode {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: true,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    });
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(rng)?;
    event::run(ctx, event_loop, state)
}
