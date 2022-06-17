use alloc::vec::Vec;
use rand::{prelude::Distribution, Rng, SeedableRng};

use crate::{
  app::monotonics,
  light::{
    color::NormColor,
    controller::{MemoryController, MemoryControllerExt, U32MemoryController},
    Lights,
  },
  return_cancel,
  util::AsmDelay,
};

use super::Show;

struct Snake {
  pub pos: usize,
  pub tail: Vec<NormColor>,
}

struct Fruit {
  pub pos: usize,
  pub color: NormColor,
}

#[derive(Default)]
pub struct SnakeShow;
impl Show for SnakeShow {
  fn run(
    &mut self,
    cancel: &mut crate::app::shared_resources::show_cancellation_token_lock,
    ctrl: &mut U32MemoryController,
    _asm_delay: AsmDelay,
    _remote_input: &mut crate::app::shared_resources::remote_input_lock,
    _configuration: &mut crate::app::shared_resources::configuration_lock,
  ) {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(monotonics::now().ticks());
    let pos_distr = rand::distributions::Uniform::new(0, Lights::N);
    let mut snake = Snake {
      pos: pos_distr.sample(&mut rng),
      tail: Vec::new(),
    };
    let mut fruit = Fruit {
      pos: pos_distr.sample(&mut rng),
      color: rng.gen::<NormColor>(),
    };

    loop {
      ctrl.set_all(NormColor::NONE);
      if snake.pos == fruit.pos {
        snake.tail.insert(0, fruit.color);
        // TODO: don't spawn fruits inside snake
        fruit.pos = pos_distr.sample(&mut rng);
        fruit.color = rng.gen::<NormColor>();
      }
      for (i, &segment) in snake.tail.iter().enumerate() {
        let pos = (snake.pos + i) % Lights::N;
        ctrl.set(pos, segment);
      }
      ctrl.set(fruit.pos, fruit.color);
      ctrl.display();
      snake.pos = (snake.pos + Lights::N - 1) % Lights::N;
      return_cancel!(cancel);
    }
  }
}
