// Game grid
pub mod grid {
  pub const CELL_SIZE: f32 = 32.;
  pub const NUM_ROWS: usize = 12;
  pub const NUM_COLS: usize = 16;
  pub const SCREEN_WIDTH: f32 = NUM_COLS as f32 * CELL_SIZE;
  pub const SCREEN_HEIGHT: f32 = NUM_ROWS as f32 * CELL_SIZE;
  pub const SCREEN_CENTER: (f32, f32) = (SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2.);
}

// Game dynamics
pub mod dynamics {
  use std::time::Duration;
  pub const INITIAL_SNAKE_SIZE: usize = 3;
  pub const INITIAL_TICK_INTERVAL: Duration = Duration::from_millis(300);
  pub const ACCELERATION_GAP: usize = 3;
  pub const ACCELERATION_VALUE: Duration = Duration::from_millis(20);
}

// Graphics
pub mod graphics {
  use crate::constants::grid::CELL_SIZE;
  pub const APPLE_SIZE: f32 = (2. * CELL_SIZE) / 3.;
  pub const SNAKE_EYE_SIZE: f32 = CELL_SIZE / 4.;
  pub const PADDING: f32 = 1.;
  pub const SCORE_TEXT_FONT_SIZE: f32 = 32.;

  // Colors
  pub const BG_COLOR: (u8, u8, u8) = (0, 0, 0);
  pub const SNAKE_HEAD_COLOR: (u8, u8, u8) = (90, 177, 90);
  pub const SNAKE_BODY_COLOR: (u8, u8, u8) = (133, 255, 133);
  pub const SNAKE_EYE_COLOR: (u8, u8, u8) = (255, 255, 90);
  pub const FOOD_COLOR: (u8, u8, u8) = (192, 40, 30);
  pub const SCORE_TEXT_COLORS: (u8, u8, u8) = (255, 255, 255);
}
