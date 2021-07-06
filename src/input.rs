use crate::log;
use std::collections::HashMap;
use crate::keycode::KeyCode;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
pub struct InputState {
  pub mouse: (bool, f32, f32),
  pub keys_pressed: HashMap<u32, KeyCode>,
  pub width: f32,
  pub height: f32,
  pub aspect_ratio: f32,
}

impl InputState {
  pub fn new() -> InputState {
    InputState {
      mouse: (false, 0.0, 0.0),
      keys_pressed: HashMap::new(),
      width: 0.0,
      height: 0.0,
      aspect_ratio: 0.0,
    }
  }

  pub fn set_mouse_pos(&mut self, x: f32, y: f32, down: bool) {
    self.mouse = (down, x, y);
  }

  pub fn set_mouse_pos_from_event(&mut self, x: i32, y: i32, down: Option<bool>) {
    let w = self.width/2.0;
    let h = self.height/2.0;
    let mouse_pos_x = (x as f32 - w)/w;
    let mouse_pos_y = (-y as f32 + h)/h;
    let is_down = match down {
      Some(d) => d,
      None => self.mouse.0,
    };
    self.mouse = (is_down, mouse_pos_x, mouse_pos_y);
    // console_log!("x: {}, y: {}, d: {}", mouse_pos_x, mouse_pos_y, is_down);
  }

  pub fn set_key_down(&mut self, keycode: u32) {
    // self.key_pressed = (true, keycode);
    self.keys_pressed.insert(keycode, KeyCode::from(keycode));
    // console_log!("c: {}, d: {}", keycode, true);
  }

  pub fn set_key_released(&mut self, keycode: u32) {
    // self.key_pressed = (false, keycode);
    self.keys_pressed.remove(&keycode);
    // console_log!("c: {}, d: {}", keycode, false);
  }

  pub fn set_window_size(&mut self, width: f32, height: f32){
    // console_log!("w: {}, h: {}", width, height);
    self.width = width;
    self.height = height;
    self.aspect_ratio = width / height;
  }

  pub fn get_keys_pressed(&self) -> &HashMap<u32, KeyCode> {
    &self.keys_pressed
  }

}