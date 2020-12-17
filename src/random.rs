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

  pub fn random_food(mut self, grid_x: usize, grid_y: usize) -> (f32, f32) {
    let food_x: f32 = (self.rng.gen::<f32>() * grid_x as f32 - 1.).round();
    let food_y: f32 = (self.rng.gen::<f32>() * grid_y as f32 - 1.).round();
    (food_x, food_y)
  }
}
