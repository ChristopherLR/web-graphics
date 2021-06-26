use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use crate::helpers::*;
use crate::shaders::{ color_2d_frag, color_2d_vert };
use crate::math::matrix::*;
use crate::scene_objects::SceneObject;
use crate::log;
use std::mem::size_of;
use std::any::type_name;

pub struct TriDown {
  program: WebGlProgram,
  vertex_length: usize,
  vertex_buffer: WebGlBuffer, 
  index_buffer: WebGlBuffer,
  index_length: usize,
  matrices: Matrices,
  u_color: WebGlUniformLocation,
  u_transform: WebGlUniformLocation,
}

pub fn size_of_type<T, const N: usize>(_: &[T; N]) -> usize {
  size_of::<T>()
}

pub fn get_ptr<T, const N: usize>(item: &[T; N]) -> u32 {
  item.as_ptr() as u32 / (size_of_type(item) as u32)
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

    // let (vert_array, buffer) = create_buffer(gl, &vertices);
    let vert_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    let vert_memory = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vert_buffer));
    let vert_location = get_ptr(&vertices);
    let vert_data_array = js_sys::Float32Array::new(&vert_memory).subarray(vert_location, vert_location + vertices.len() as u32);
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_data_array, GL::STATIC_DRAW);

    let index_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    let index_memory = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    let index_location = get_ptr(&indices);
    let index_data_array = js_sys::Uint16Array::new(&index_memory).subarray(index_location, index_location + indices.len() as u32);
    gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &index_data_array, GL::STATIC_DRAW);

    Self {
      u_color: gl.get_uniform_location(&program, "u_color").unwrap(),
      u_transform: gl.get_uniform_location(&program, "u_transform").unwrap(),
      vertex_buffer: vert_buffer,
      vertex_length: vertices.len(),
      index_buffer: index_buffer,
      index_length: indices.len(),
      program: program,
      matrices: Matrices::new(),
    }
  }
}

impl SceneObject for TriDown {
  fn draw_self(&self, gl: Option<&GL>){
    let gl = gl.unwrap();
    gl.use_program(Some(&self.program));
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
    // Position attrib location, size, type, normalise, stride, offset
    gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);
    gl.uniform4f(Some(&self.u_color), 0.0, 0.5, 0.5, 1.0);

    let mut model_matrix = self.matrices.calc_model_matrix();
    let mut t_mat = Matrix::new();
    t_mat.translate(0.25, 0.25, 0.0);
    model_matrix = model_matrix * t_mat;
    model_matrix.print();

    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &model_matrix.0);
    gl.draw_elements_with_i32(GL::TRIANGLES, self.index_length as i32, GL::UNSIGNED_SHORT, 0)
  }
}