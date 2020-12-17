use rand::prelude::*;

pub struct Random {
  rng: ThreadRng,
}

impl Random {
  pub fn new() -> Random {
    Random {
      rng: rand::thread_rng(),
    }
  }

  pub fn random_food(
    mut self,
    grid_x: usize,
    grid_y: usize,
    cell_size: f32,
    snake: Option<&Vec<(f32, f32)>>,
  ) -> (f32, f32) {
    let mut food_x: f32 = (self.rng.gen::<f32>() * grid_x as f32 - 1.).round() * cell_size;
    let mut food_y: f32 = (self.rng.gen::<f32>() * grid_y as f32 - 1.).round() * cell_size;
    match snake {
      Some(sn) => {
        while sn.contains(&(food_x, food_y)) {
          food_x = (self.rng.gen::<f32>() * grid_x as f32 - 1.).round() * cell_size;
          food_y = (self.rng.gen::<f32>() * grid_y as f32 - 1.).round() * cell_size;
        }
        (food_x, food_y)
      }
      None => (food_x, food_y),
    }
  }
}
