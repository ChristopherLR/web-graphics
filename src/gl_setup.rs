use wasm_bindgen::{ JsCast, JsValue };
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use std::convert::TryInto;
use wasm_bindgen::convert::FromWasmAbi;
use crate::log;
use std::cell::Cell;
use std::rc::Rc;


macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn mouse_handler(event: MouseEvent){
  console_log!("x: {}, y: {}", event.offset_x(), event.offset_y());
}

pub fn key_handler(event: KeyboardEvent){
  console_log!("{:?}", event.key_code());
}

pub fn init_webgl_context() -> Result<WebGl2RenderingContext, JsValue> {
  let window = window().unwrap();

  let document = window.document().unwrap();
  let canvas = document.get_element_by_id("rustCanvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
  let gl: GL = canvas.get_context("webgl2")?.unwrap().dyn_into()?;

  attach_handler("mousemove", &canvas, &mouse_handler)?;
  attach_handler("mousedown", &canvas, &mouse_handler)?;
  attach_handler("mouseup", &canvas, &mouse_handler)?;
  attach_handler("keydown", &window, &key_handler)?;

  gl.enable(GL::BLEND);
  gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
  gl.clear_color(0.0, 0.0, 0.0, 1.0);
  gl.clear_depth(1.0);

  Ok(gl)
}

fn attach_handler<T: FromWasmAbi>(name: &str, target: &EventTarget, handle: &'static dyn Fn(T)) -> Result<(), JsValue> {
  let handler = move |event: T| { handle(event) };
  let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
  target.add_event_listener_with_callback(name, handler.as_ref().unchecked_ref())?;
  handler.forget();
  Ok(())
}