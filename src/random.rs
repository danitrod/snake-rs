use rand::prelude::*;

#[derive(Clone, Copy)]
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
    let mut food_x: f32 = (self.rng.gen::<f32>() * (grid_x - 1) as f32).round() * cell_size;
    let mut food_y: f32 = (self.rng.gen::<f32>() * (grid_y - 1) as f32).round() * cell_size;
    let food = match snake {
      Some(sn) => {
        while sn.contains(&(food_x, food_y)) {
          food_x = (self.rng.gen::<f32>() * (grid_x - 1) as f32).round() * cell_size;
          food_y = (self.rng.gen::<f32>() * (grid_y - 1) as f32).round() * cell_size;
        }
        (food_x, food_y)
      }
      None => (food_x, food_y),
    };
    println!("New food at {:?}", food);
    food
  }
}
