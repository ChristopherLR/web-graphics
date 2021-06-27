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
use chrono::prelude::*;
use web_sys::WebGl2RenderingContext as GL;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct WebClient{
    gl: Mutex<Arc<GL>>,
    width: f32,
    height: f32,
    time: f64,
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
          root: root,
          time: 0.0,
        }
    }

    pub fn update_size(&mut self, height: f32, width: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn update(&mut self, time: f64) -> Result<(), JsValue> {
        let old_time : f64 = self.time;
        // log(&format!("{}", old_time));
        self.time = time as f64;
        // log(&format!("{}", time));
        let dt = self.time - old_time;
        self.root.update(dt as f32);
        // log(&format!("{}", dt));
        Ok(())
    }

    pub fn render(&mut self){
        let gl = self.gl.lock().unwrap();
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.root.draw(Some(&gl));
    }
}
