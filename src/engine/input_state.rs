pub struct InputState {
  pub up: Key,
  pub down: Key,
  pub left: Key,
  pub right: Key,
  pub forward: Key,
  pub back: Key,
  pub look_up: Key,
  pub look_down: Key,
  pub look_left: Key,
  pub look_right: Key,
}

impl InputState {
  pub fn new() -> InputState {
    InputState {
      up: Key {is_down: false},
      down: Key {is_down: false},
      left: Key {is_down: false},
      right: Key {is_down: false},
      forward: Key {is_down: false},
      back: Key {is_down: false},
      look_up: Key {is_down: false},
      look_down: Key {is_down: false},
      look_left: Key {is_down: false},
      look_right: Key {is_down: false},
    }
  }
}

pub struct Key {
  pub is_down: bool,
}