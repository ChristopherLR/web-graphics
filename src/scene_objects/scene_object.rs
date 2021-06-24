use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use std::sync::Arc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use js_sys::WebAssembly;
use super::super::helpers::*;
use super::super::shaders::{ color_2d_frag, color_2d_vert };
use super::super::log;

pub trait SceneObject {
  fn draw_self(&self, gl: Option<&GL>);
  fn draw(&self, gl: Option<&GL>){
    self.draw_self(gl);
    match self.children() {
      Some(children) => {
        for child in children.iter() {
          child.draw(gl);
        }
      },
      None => ()
    }
  }
  fn children(&self) -> Option<Vec<Arc<dyn SceneObject>>> {
    None
  }
}