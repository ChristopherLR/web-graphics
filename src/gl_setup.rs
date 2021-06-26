use wasm_bindgen::{ JsCast, JsValue };
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;

pub fn init_webgl_context() -> Result<WebGl2RenderingContext, JsValue> {
  let window = window().unwrap();
  let document = window.document().unwrap();
  let canvas = document.get_element_by_id("rustCanvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
  let gl: GL = canvas.get_context("webgl2")?.unwrap().dyn_into()?;

  gl.enable(GL::BLEND);
  gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
  gl.clear_color(0.0, 0.0, 0.0, 1.0);
  gl.clear_depth(1.0);

  Ok(gl)
}