#![allow(dead_code)]
extern crate wasm_bindgen;
extern crate wee_alloc;
mod gl_setup;
mod programs;
mod helpers;
mod shaders;
mod scene_objects;
pub mod math;
mod input;
mod cameras;
mod output;
mod keycode;
#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use scene_objects::{ SceneObject, Pivot, three_d::{Cube, WireCube, Grid }};
use cameras::PerspectiveCamera;
use std::sync::{ Arc, Mutex };
use math::matrix::Matrix;
use chrono::prelude::*;
use std::cell::RefCell;
use std::f32::consts::PI;
use web_sys::WebGl2RenderingContext as GL;
use input::*;
use output::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

thread_local!{
    static ROOT: RefCell<Pivot> = RefCell::new(Pivot::new());
    static OUTPUT: RefCell<OutputState> = RefCell::new(OutputState::new());
}

lazy_static! {
  static ref INPUT: Mutex<InputState> = Mutex::new(InputState::new());
}

#[wasm_bindgen]
pub struct WebClient{
    gl: Mutex<Arc<GL>>,
    time: f64,
    model_matrix: Matrix,
    camera: PerspectiveCamera,
}

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new(height: f32, width: f32) -> Self {
        console_log!("Creating Client");

        console_error_panic_hook::set_once();
        let (gl, infobox) = gl_setup::init_webgl_context().unwrap();

        INPUT.lock().unwrap().set_window_size(width, height);

        OUTPUT.with(|output|{
            output.borrow_mut().attach_infobox(infobox);
        });

        let mut camera = PerspectiveCamera::new(0.0, PI/3.0, 1.0, 0.1, 10.0);
        camera.matrices.translate(0.0, 0.0, 1.0);

        let mut wire_cube = WireCube::new(&gl);
        wire_cube.matrices.set_scale(0.1, 0.1, 0.1);
        wire_cube.matrices.translate(0.0, 0.0, 0.0);

        let mut cube = Cube::new(&gl);
        cube.matrices.set_scale(0.25, 0.25, 0.25);
        cube.matrices.translate(0.0, 1.0, 0.0);

        let mut grid = Grid::new(&gl);

        ROOT.with(|root|{
            root.borrow_mut().add_child(Box::new(cube));
            root.borrow_mut().add_child(Box::new(wire_cube));
            root.borrow_mut().add_child(Box::new(grid));
        });
        let gl = Mutex::new(Arc::new(gl));

        Self {
          gl: gl,
          time: 0.0,
          model_matrix: Matrix::new(),
          camera: camera
        }
    }

    pub fn update_size(&self, height: f32, width: f32) {
        INPUT.lock().unwrap().set_window_size(width, height);
    }

    pub fn update(&mut self, time: f64) -> Result<(), JsValue> {
        // let (d, x, y) = INPUT.lock().unwrap().mouse;
        // let rot = match d {
        //     true => 0.01,
        //     false => -0.01,
        // };
        let old_time : f64 = self.time;
        self.time = time as f64;
        let dt = (self.time - old_time) as f32;

        self.camera.update(dt, &INPUT.lock().unwrap());
        self.camera.matrices.calc_model_matrix(None);

        // console_log!("{}", dt);
        OUTPUT.with(|output|{
            output.borrow_mut().update_fps(((1.0/dt) * 1000.0).round());
        });
        ROOT.with(|root|{
            // root.borrow_mut().matrices.rotate_z(rot);
            // root.borrow_mut().matrices.set_position(x, y, 0.0);
            root.borrow_mut().update(dt, &INPUT.lock().unwrap());
        });
        Ok(())
    }

    pub fn render(&mut self){
        let gl = self.gl.lock().unwrap();
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        ROOT.with(|root|{
            // let mut rt = root.borrow_mut();
            // let mat = rt.matrices.model_matrix.ident();
            // let mat = rt.matrices.model_matrix.clone();
            root.borrow_mut().draw(Some(&gl), Some(&self.model_matrix), &self.camera);
        });
    }
}