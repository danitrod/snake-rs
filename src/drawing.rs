use crate::{
  constants::{
    graphics::*,
    grid::{CELL_SIZE, SCREEN_CENTER},
  },
  Direction::{self, *},
};
use ggez::{
  graphics::{self, Color, Font, Scale},
  nalgebra as na, Context, GameResult,
};

pub fn drawing(
  ctx: &mut Context,
  snake: &Vec<(f32, f32)>,
  food: (f32, f32),
  direction: &Direction,
  score: usize,
) -> GameResult {
  let mut iter_snake = snake.iter();

  // Draw snake head
  let (r, g, b) = SNAKE_HEAD_COLOR;
  let head = iter_snake.next().expect("No snake head");
  let head_rect = graphics::Mesh::new_rectangle(
    ctx,
    graphics::DrawMode::fill(),
    graphics::Rect::new(
      head.0 + PADDING,
      head.1 + PADDING,
      CELL_SIZE - PADDING * 2.,
      CELL_SIZE - PADDING * 2.,
    ),
    Color::from_rgb(r, g, b),
  )?;
  graphics::draw(ctx, &head_rect, (na::Point2::new(0., 0.),))?;

  let (r, g, b) = SNAKE_EYE_COLOR;
  let left_eye_x = match direction {
    Left | Right => head.0 + CELL_SIZE / 2. - SNAKE_EYE_SIZE / 2.,
    Up => head.0 + CELL_SIZE / 2. - (SNAKE_EYE_SIZE * 3.) / 2.,
    Down => head.0 + CELL_SIZE / 2. + SNAKE_EYE_SIZE / 2.,
  };
  let left_eye_y = match direction {
    Up | Down => head.1 + CELL_SIZE / 2. - SNAKE_EYE_SIZE / 2.,
    Left => head.1 + CELL_SIZE / 2. + SNAKE_EYE_SIZE / 2.,
    Right => head.1 + CELL_SIZE / 2. - (SNAKE_EYE_SIZE * 3.) / 2.,
  };
  let left_eye = graphics::Mesh::new_rectangle(
    ctx,
    graphics::DrawMode::fill(),
    graphics::Rect::new(left_eye_x, left_eye_y, SNAKE_EYE_SIZE, SNAKE_EYE_SIZE),
    Color::from_rgb(r, g, b),
  )?;
  graphics::draw(ctx, &left_eye, (na::Point2::new(0., 0.),))?;
  let right_eye_x = match direction {
    Left | Right => head.0 + CELL_SIZE / 2. - SNAKE_EYE_SIZE / 2.,
    Up => head.0 + CELL_SIZE / 2. + SNAKE_EYE_SIZE / 2.,
    Down => head.0 + CELL_SIZE / 2. - (SNAKE_EYE_SIZE * 3.) / 2.,
  };
  let right_eye_y = match direction {
    Up | Down => head.1 + CELL_SIZE / 2. - SNAKE_EYE_SIZE / 2.,
    Left => head.1 + CELL_SIZE / 2. - (SNAKE_EYE_SIZE * 3.) / 2.,
    Right => head.1 + CELL_SIZE / 2. + SNAKE_EYE_SIZE / 2.,
  };
  let right_eye = graphics::Mesh::new_rectangle(
    ctx,
    graphics::DrawMode::fill(),
    graphics::Rect::new(right_eye_x, right_eye_y, SNAKE_EYE_SIZE, SNAKE_EYE_SIZE),
    Color::from_rgb(r, g, b),
  )?;
  graphics::draw(ctx, &right_eye, (na::Point2::new(0., 0.),))?;

  // Draw snake body
  let (r, g, b) = SNAKE_BODY_COLOR;
  for cell in iter_snake {
    let cell_rect = graphics::Mesh::new_rectangle(
      ctx,
      graphics::DrawMode::fill(),
      graphics::Rect::new(
        cell.0 + PADDING,
        cell.1 + PADDING,
        CELL_SIZE - PADDING * 2.,
        CELL_SIZE - PADDING * 2.,
      ),
      Color::from_rgb(r, g, b),
    )?;
    graphics::draw(ctx, &cell_rect, (na::Point2::new(0., 0.),))?;
  }

  let (r, g, b) = FOOD_COLOR;
  let food_rect = graphics::Mesh::new_circle(
    ctx,
    graphics::DrawMode::fill(),
    na::Point2::new(food.0 + CELL_SIZE / 2., food.1 + CELL_SIZE / 2.),
    APPLE_SIZE / 2.,
    1.,
    Color::from_rgb(r, g, b),
  )?;
  graphics::draw(ctx, &food_rect, (na::Point2::new(0., 0.),))?;

  let (r, g, b) = SCORE_TEXT_COLORS;
  let mut score_txt = graphics::Text::new(format!("Score: {}", score));
  let score_txt = score_txt.set_font(
    Font::default(),
    Scale {
      x: SCORE_TEXT_FONT_SIZE,
      y: SCORE_TEXT_FONT_SIZE,
    },
  );
  let w = score_txt.width(ctx) as f32;
  graphics::draw(
    ctx,
    score_txt,
    (
      na::Point2::new(SCREEN_CENTER.0 - w / 2., 50.),
      Color::from_rgb(r, g, b),
    ),
  )?;

  Ok(())
}
