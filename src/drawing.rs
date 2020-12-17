use crate::constants::{CELL_SIZE, PADDING};
use ggez::{
  graphics::{self, Color},
  nalgebra as na, Context, GameResult,
};

pub fn drawing(ctx: &mut Context, snake: &Vec<(f32, f32)>, food: (f32, f32)) -> GameResult {
  let mut iter_snake = snake.iter();

  // Draw snake head
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
    Color::from_rgb(70, 205, 70),
  )?;
  graphics::draw(ctx, &head_rect, (na::Point2::new(0., 0.),))?;
  let left_eye = graphics::Mesh::new_rectangle(
    ctx,
    graphics::DrawMode::fill(),
    graphics::Rect::new(
      head.0 + CELL_SIZE / 3.,
      head.1 + CELL_SIZE / 4.,
      CELL_SIZE / 4.,
      CELL_SIZE / 4.,
    ),
    Color::from_rgb(30, 30, 30),
  )?;
  graphics::draw(ctx, &left_eye, (na::Point2::new(0., 0.),))?;
  let right_eye = graphics::Mesh::new_rectangle(
    ctx,
    graphics::DrawMode::fill(),
    graphics::Rect::new(
      head.0 + (CELL_SIZE / 3.) * 2.,
      head.1 + CELL_SIZE / 4.,
      CELL_SIZE / 4.,
      CELL_SIZE / 4.,
    ),
    Color::from_rgb(30, 30, 30),
  )?;
  graphics::draw(ctx, &right_eye, (na::Point2::new(0., 0.),))?;

  // Draw snake body
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
      Color::from_rgb(90, 255, 90),
    )?;
    graphics::draw(ctx, &cell_rect, (na::Point2::new(0., 0.),))?;
  }

  let food_rect = graphics::Mesh::new_rectangle(
    ctx,
    graphics::DrawMode::fill(),
    graphics::Rect::new(food.0, food.1, CELL_SIZE, CELL_SIZE),
    Color::from_rgb(192, 40, 30),
  )?;
  graphics::draw(ctx, &food_rect, (na::Point2::new(0., 0.),))?;

  Ok(())
}
