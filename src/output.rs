use crate::log;
use web_sys::Element;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct OutputState {
  pub infobox: Option<Element>,
}

impl OutputState {
  pub fn new() -> OutputState {
    OutputState {
      infobox: None,
    }
  }

  pub fn attach_infobox(&mut self, element: Element) {
    self.infobox = Some(element)
  }

  pub fn update_fps(&mut self, fps: f32){
    self.infobox.as_ref().unwrap().set_inner_html(&fps.to_string());
  }
}