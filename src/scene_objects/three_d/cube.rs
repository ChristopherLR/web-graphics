use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
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

pub struct Cube {
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

impl Cube {
  pub fn new(gl: &GL) -> Self {
    let program = link_program(gl, color_3d_vert::SHADER, color_3d_frag::SHADER).unwrap();

    let vertices: [f32; 24] = [
      -1.,  1.,  1., // BTL 0
      -1., -1.,  1., // BBL 1
       1.,  1.,  1., // BTR 2
       1., -1.,  1., // BBR 3
      -1.,  1., -1., // FTL 4
      -1., -1., -1., // FBL 5
       1.,  1., -1., // FTR 6
       1., -1., -1., // FBR 7
    ];

    let indices: [u16; 36] = [
      5, 6, 4, // Front Top
      5, 7, 6, // Front Bottom
      4, 2, 0, // Top Top
      4, 6, 2, // Top bottom
      3, 1, 0, // Back Bottom
      0, 2, 3, // Back Top
      1, 3, 7, // Bottom Back
      1, 7, 5, // Bottom Front
      1, 5, 0, // Left bottom
      0, 5, 4, // Left Top
      3, 6, 7, // Right Bottom
      3, 6, 2, // Right Top
    ];

    let vertex_buffer = create_buffer(gl, &vertices, GL::ARRAY_BUFFER);
    let index_buffer = create_buffer(gl, &indices, GL::ELEMENT_ARRAY_BUFFER);

    Self {
      // u_color: gl.get_uniform_location(&program, "u_color").unwrap(),
      u_model: gl.get_uniform_location(&program, "u_model").unwrap(),
      u_proj: gl.get_uniform_location(&program, "u_proj").unwrap(),
      u_view: gl.get_uniform_location(&program, "u_view").unwrap(),
      vertex_buffer: vertex_buffer,
      vertex_length: vertices.len(),
      index_buffer: index_buffer,
      index_length: indices.len(),
      program: program,
      matrices: Matrices::new(),
      color: [1.0, 1.0, 0.0, 0.0],
      gl: gl.clone()
    }
  }
}

impl SceneObject for Cube {
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
    self.matrices.calc_model_matrix(parent_matrix, true);
  }

  fn update_self(&mut self, dt: f32, input: &InputState) {
    // self.matrices.rotate_x(0.01);
    self.matrices.rotate_y(0.01);
    // self.matrices.rotate_z(0.01);
  }

  fn draw_self(&mut self, gl: Option<&GL>, camera: &PerspectiveCamera){
    let gl = gl.unwrap();
    gl.use_program(Some(&self.program));
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
    let location = gl.get_attrib_location(&self.program, &"a_position");
    // Position attrib location, size, type, normalise, stride, offset
    gl.vertex_attrib_pointer_with_i32(location as u32, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    // gl.uniform4f(Some(&self.u_color), self.color[0], self.color[1], self.color[2], self.color[3]);

    let mut view = camera.get_view_matrix();
    // let mut view = Matrix::new();
    let mut proj = camera.get_perspective_matrix();
    // view.ident();
    // proj.ident();
    self.matrices.model_matrix.print();
    // view.print();
    // proj.print();
    // console_log!("{:?}", proj);
    // console_log!("{:?}", &self.matrices.model_matrix.as_slice());
    // print_matrix(self.matrices.model_matrix.as_slice());
    // print_matrix(&view.as_slice());
    // print_matrix(&proj.as_slice());

    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_model), false, &self.matrices.model_matrix.0);
    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_view), false, &view.0);
    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_proj), false, &proj.0);

    gl.draw_elements_with_i32(GL::TRIANGLES, self.index_length as i32, GL::UNSIGNED_SHORT, 0)
  }
}

impl Drop for Cube {
  fn drop(&mut self) {
      console_log!("drop");
      self.gl.delete_buffer(Some(&self.vertex_buffer));
      self.gl.delete_buffer(Some(&self.index_buffer));
  }
}