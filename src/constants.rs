use std::time::Duration;

pub const INITIAL_SNAKE_SIZE: usize = 3;
pub const CELL_SIZE: f32 = 16.;
pub const NUM_ROWS: usize = 48;
pub const NUM_COLS: usize = 64;
pub const SCREEN_WIDTH: f32 = NUM_COLS as f32 * CELL_SIZE;
pub const SCREEN_HEIGHT: f32 = NUM_ROWS as f32 * CELL_SIZE;
pub const SCREEN_CENTER: (f32, f32) = (SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2.);
pub const TICK_INTERVAL: Duration = Duration::from_millis(300);
pub const PADDING: f32 = 1.;
