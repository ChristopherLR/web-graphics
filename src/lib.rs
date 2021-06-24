extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
mod gl_setup;
mod programs;
mod helpers;
mod shaders;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[wasm_bindgen]
pub struct WebClient {
    gl: WebGlRenderingContext,
    program: programs::Color2D,
}

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log("Creating Client");

        console_error_panic_hook::set_once();
        let gl = gl_setup::init_webgl_context().unwrap();

        Self {
          program: programs::Color2D::new(&gl),
          gl: gl
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self){
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.program.render(&self.gl, 0., 1., 0., 1., 1., 1.);
    }
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen]
    fn log(s: &str);
}