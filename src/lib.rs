extern crate wasm_bindgen;
#[macro_use]
extern crate lazy_static;
use wasm_bindgen::prelude::*;
mod gl_setup;
mod programs;
mod helpers;
mod shaders;
mod scene_objects;

use scene_objects::SceneObject;
use std::sync::{ Arc, Mutex };
use programs::{ Color2D, TriDown};
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[wasm_bindgen]
pub struct WebClient{
    gl: WebGlRenderingContext,
    width: f32,
    height: f32,
    root: Vec<Box<dyn SceneObject>>,
}

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new(height: f32, width: f32) -> Self {
        log("Creating Client");

        console_error_panic_hook::set_once();
        let gl = gl_setup::init_webgl_context().unwrap();
        let mut root = Vec::new();
        root.push(Box::new(Color2D::new(Some(&gl))) as Box<dyn SceneObject>);
        root.push(Box::new(TriDown::new(Some(&gl))) as Box<dyn SceneObject>);

        Self {
          width: width,
          height: height,
          gl: gl,
          root: root
        }
    }

    pub fn update_size(&mut self, height: f32, width: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn update(&mut self, _time: f32) -> Result<(), JsValue> {
        // log(&format!("{}", _time));
        Ok(())
    }

    pub fn render(&self){
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        for child in self.root.iter() {
            child.draw(Some(&self.gl))
        }
    }
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen]
    fn log(s: &str);
}