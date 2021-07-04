use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use js_sys::{ Float32Array, Float64Array, Uint16Array };
use web_sys::*;
use js_sys::WebAssembly;
use crate::helpers::*;
use crate::input::InputState;
use crate::shaders::{ color_3d_frag, color_3d_vert };
use crate::math::matrix::*;
use crate::scene_objects::SceneObject;
use crate::cameras::PerspectiveCamera;
use crate::log;
use std::mem::size_of;

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct Grid {
  program: WebGlProgram,
  vertex_length: usize,
  vertex_buffer: WebGlBuffer, 
  index_buffer: WebGlBuffer,
  index_length: usize,
  pub color: [f32; 4],
  pub matrices: Matrices,
  // pub u_color: WebGlUniformLocation,
  u_model: WebGlUniformLocation,
  u_view: WebGlUniformLocation,
  u_proj: WebGlUniformLocation,
  gl: GL,
}

impl Grid {
  pub fn new(gl: &GL) -> Self {
    let program = link_program(gl, color_3d_vert::SHADER, color_3d_frag::SHADER).unwrap();

    let n_lines = 20;
    let mut vertices: [[f32; 3]; 20 * 4] = [[0.0; 3]; 80];
    let mut indices: [u16; 20 * 4] = [0; 80];

    for i in 0..20 {
      let x: f32 = 2.0*(i as f32)/(20.0-1.0)-1.0;
      vertices[4*i] = [-1.0, 0.0, x];
      vertices[4*i + 1] = [ 1.0, 0.0, x];
      indices[4*i] = 4*i as u16;
      indices[4*i + 1] = 4*i as u16 + 1;

      vertices[4*i + 2] = [x, 0.0, -1.0];
      vertices[4*i + 3] = [x, 0.0, 1.0];
      indices[4*i+2] = 4*i as u16 + 2;
      indices[4*i+3] = 4*i as u16 + 3;
    }

    let mut unwrapped: [f32; 240] = [0.0; 240];
    let mut idx = 0;
    for i in 0..vertices.len() {
      unwrapped[idx] = vertices[i][0];
      idx+=1;
      unwrapped[idx] = vertices[i][1];
      idx+=1;
      unwrapped[idx] = vertices[i][2];
      idx+=1;
    }
    
    // console_log!("{:?}", unwrapped);
    let buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    let memory = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    let location = get_ptr(&unwrapped);
    let data_array = Float32Array::new(&memory).subarray(location, location + unwrapped.len() as u32);
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);

    let index_buffer = create_buffer(gl, &indices, GL::ELEMENT_ARRAY_BUFFER);

    Self {
      // u_color: gl.get_uniform_location(&program, "u_color").unwrap(),
      u_model: gl.get_uniform_location(&program, "u_model").unwrap(),
      u_proj: gl.get_uniform_location(&program, "u_proj").unwrap(),
      u_view: gl.get_uniform_location(&program, "u_view").unwrap(),
      vertex_buffer: buffer,
      vertex_length: unwrapped.len(),
      index_buffer: index_buffer,
      index_length: indices.len(),
      program: program,
      matrices: Matrices::new(),
      color: [1.0, 1.0, 0.0, 0.0],
      gl: gl.clone()
    }
  }
}

impl SceneObject for Grid {
  fn name(&self) -> &str {
    "Cube"
  }

  fn get_model_matrix(&self) -> &Matrix {
    &self.matrices.model_matrix
  }

  fn get_mut_model_matrix(&mut self) -> &Matrix {
    &self.matrices.model_matrix
  }

  fn calc_model_matrix(&mut self, parent_matrix: Option<&Matrix>){
    self.matrices.calc_model_matrix(parent_matrix);
  }

  fn update_self(&mut self, dt: f32, input: &InputState) {
    // self.matrices.rotate_x(0.01);
    // self.matrices.rotate_y(0.01);
    // self.matrices.rotate_z(0.01);
  }

  fn draw_self(&mut self, gl: Option<&GL>, camera: &PerspectiveCamera){
    let gl = gl.unwrap();
    gl.use_program(Some(&self.program));
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
    // Position attrib location, size, type, normalise, stride, offset
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    // gl.uniform4f(Some(&self.u_color), self.color[0], self.color[1], self.color[2], self.color[3]);

    let mut view = camera.get_view_matrix();
    let mut proj = camera.get_perspective_matrix();
    // view.ident();
    // proj.ident();
    // self.matrices.model_matrix.print();
    //view.print();

    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_view), false, &view.as_slice());
    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_model), false, &self.matrices.model_matrix.as_slice());
    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_proj), false, &proj.as_slice());

    gl.draw_elements_with_i32(GL::LINES, self.index_length as i32, GL::UNSIGNED_SHORT, 0)
  }
}

impl Drop for Grid {
  fn drop(&mut self) {
      console_log!("drop");
      self.gl.delete_buffer(Some(&self.vertex_buffer));
      self.gl.delete_buffer(Some(&self.index_buffer));
  }
}