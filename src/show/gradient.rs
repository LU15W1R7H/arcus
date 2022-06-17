use arclib::nl;

use crate::{
  light::{
    color::NormColor,
    controller::{MemoryController, U32MemoryController},
    Lights,
  },
  util::AsmDelay,
};

use super::Show;

pub struct GradientShow([NormColor; 2]);
impl GradientShow {
  pub fn new(colors: [NormColor; 2]) -> Self {
    Self(colors)
  }
}
impl Show for GradientShow {
  fn run(
    &mut self,
    _cancel: &mut crate::app::shared_resources::show_cancellation_token_lock,
    ctrl: &mut U32MemoryController,
    _asm_delay: AsmDelay,
    _remote_input: &mut crate::app::shared_resources::remote_input_lock,
    _configuration: &mut crate::app::shared_resources::configuration_lock,
  ) {
    for l in 0..Lights::N {
      let lf = nl!(l) / nl!(Lights::N - 1);
      ctrl.set(l, self.0[0].gradient_hsv(self.0[1], lf));
    }
    ctrl.display();
  }
}
