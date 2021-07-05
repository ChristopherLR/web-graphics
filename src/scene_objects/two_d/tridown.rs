use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use crate::helpers::*;
use crate::input::InputState;
use crate::shaders::{ color_2d_frag, color_2d_vert };
use crate::math::matrix::*;
use crate::scene_objects::SceneObject;
use crate::cameras::PerspectiveCamera;
use crate::log;
use std::mem::size_of;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


pub struct TriDown {
  program: WebGlProgram,
  vertex_length: usize,
  vertex_buffer: WebGlBuffer, 
  index_buffer: WebGlBuffer,
  index_length: usize,
  pub color: [f32; 4],
  pub matrices: Matrices,
  pub u_color: WebGlUniformLocation,
  u_transform: WebGlUniformLocation,
}

impl TriDown {
  pub fn new(gl: &GL) -> Self {
    let program = link_program(gl, color_2d_vert::SHADER, color_2d_frag::SHADER).unwrap();

    let vertices: [f32; 6] = [
      -1., 1.,
       1., 1.,
       0., 0.,
    ];

    let indices: [u16; 3] = [
      0, 1, 2
    ];

    let vert_buffer = create_buffer(gl, &vertices, GL::ARRAY_BUFFER);
    let index_buffer = create_buffer(gl, &indices, GL::ELEMENT_ARRAY_BUFFER);

    Self {
      u_color: gl.get_uniform_location(&program, "u_color").unwrap(),
      u_transform: gl.get_uniform_location(&program, "u_transform").unwrap(),
      vertex_buffer: vert_buffer,
      vertex_length: vertices.len(),
      index_buffer: index_buffer,
      index_length: indices.len(),
      program: program,
      matrices: Matrices::new(),
      color: [0.0, 0.0, 0.0, 0.0]
    }
  }
}

impl SceneObject for TriDown {
  fn name(&self) -> &str {
    "TriDown"
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
    self.matrices.rotate_y(0.01);
    // log(&format!("{:?}", self.matrices.get_rotation()))
    // self.matrices.rotate_z(0.01);
  }

  fn draw_self(&mut self, gl: Option<&GL>, camera: &PerspectiveCamera){
    let gl = gl.unwrap();
    gl.use_program(Some(&self.program));
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
    // Position attrib location, size, type, normalise, stride, offset
    gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    gl.uniform4f(Some(&self.u_color), self.color[0], self.color[1], self.color[2], self.color[3]);

    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &self.matrices.model_matrix.0);
    gl.draw_elements_with_i32(GL::TRIANGLES, self.index_length as i32, GL::UNSIGNED_SHORT, 0)
  }
}