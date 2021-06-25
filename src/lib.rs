extern crate wasm_bindgen;
extern crate wee_alloc;
mod gl_setup;
mod programs;
mod helpers;
mod shaders;
mod scene_objects;
mod math;

use wasm_bindgen::prelude::*;
use scene_objects::{ SceneObject, Pivot, two_d::TriDown };
use std::sync::{ Arc, Mutex };
use programs::{ Color2D };
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct WebClient{
    gl: Mutex<Arc<WebGlRenderingContext>>,
    width: f32,
    height: f32,
    root: Pivot,
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new(height: f32, width: f32) -> Self {
        log("Creating Client");

        console_error_panic_hook::set_once();
        let gl = gl_setup::init_webgl_context().unwrap();
        let mut root = Pivot::new();
        root.add_child(Box::new(Color2D::new(&gl)));
        root.add_child(Box::new(TriDown::new(&gl)));
        let gl = Mutex::new(Arc::new(gl));

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
        let gl = self.gl.lock().unwrap();
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.root.draw(Some(&gl));
    }
}
