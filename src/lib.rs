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
use web_sys::*;
use chrono::prelude::*;
use std::cell::RefCell;
use std::f32::consts::PI;
use web_sys::WebGl2RenderingContext as GL;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local!{
    static ROOT: RefCell<Pivot> = RefCell::new(Pivot::new());
}

#[wasm_bindgen]
pub struct WebClient{
    gl: Mutex<Arc<GL>>,
    width: f32,
    height: f32,
    time: f64,
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
        let mut piv1 = Pivot::new();
        let mut tri_1 = TriDown::new(&gl);
        tri_1.color = [1.0, 0.0, 0.0, 1.0];
        tri_1.matrices.translate(0.0, 0.5, 0.0);
        tri_1.matrices.set_scale(0.5, 0.5, 1.0);
        piv1.add_child(Box::new(tri_1));

        let mut piv2 = Pivot::new();
        piv2.matrices.rotate_z(PI/2.0);
        let mut tri_2 = TriDown::new(&gl);
        tri_2.color = [0.0, 1.0, 0.0, 1.0];
        tri_2.matrices.translate(0.0, 0.5, 0.0);
        tri_2.matrices.set_scale(0.5, 0.5, 1.0);
        piv2.add_child(Box::new(tri_2));

        let mut piv3 = Pivot::new();
        piv3.matrices.rotate_z(PI);
        let mut tri_3 = TriDown::new(&gl);
        tri_3.color = [0.0, 0.0, 1.0, 1.0];
        tri_3.matrices.translate(0.0, 0.5, 0.0);
        tri_3.matrices.set_scale(0.5, 0.5, 1.0);
        piv3.add_child(Box::new(tri_3));

        let mut piv4 = Pivot::new();
        piv4.matrices.rotate_z(1.5*PI);
        let mut tri_4 = TriDown::new(&gl);
        tri_4.color = [1.0, 1.0, 1.0, 1.0];
        tri_4.matrices.translate(0.0, 0.5, 0.0);
        tri_4.matrices.set_scale(0.5, 0.5, 1.0);
        piv4.add_child(Box::new(tri_4));

        ROOT.with(|root|{
            root.borrow_mut().add_child(Box::new(piv1));
            root.borrow_mut().add_child(Box::new(piv2));
            root.borrow_mut().add_child(Box::new(piv3));
            root.borrow_mut().add_child(Box::new(piv4));
        });
        let gl = Mutex::new(Arc::new(gl));

        Self {
          width: width,
          height: height,
          gl: gl,
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
        ROOT.with(|root|{
            let mut rot = root.borrow().matrices.get_rotation();
            rot[2] += 0.00001;
            root.borrow_mut().matrices.set_rotation_arr(rot);

            root.borrow_mut().update(dt as f32);
        });
        // log(&format!("{}", dt));
        Ok(())
    }

    pub fn render(&mut self){
        let gl = self.gl.lock().unwrap();
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        ROOT.with(|root|{
            let mut rt = root.borrow_mut();
            let mat = rt.matrices.model_matrix.clone();
            rt.draw(Some(&gl), Some(&mat));
        });
    }
}
