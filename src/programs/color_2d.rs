use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use crate::helpers::*;
use crate::math::matrix::*;
use super::super::shaders::{ color_2d_frag, color_2d_vert };
use super::super::scene_objects::SceneObject;

pub struct Color2D {
  program: WebGlProgram,
  vertex_length: usize,
  vertex_buffer: WebGlBuffer, 
  u_color: WebGlUniformLocation,
  u_transform: WebGlUniformLocation,
}

impl Color2D {
  pub fn new(gl: &GL) -> Self {
    let program = link_program(gl, color_2d_vert::SHADER, color_2d_frag::SHADER).unwrap();

    let vertices: [f32; 6] = [
      -1., -1.,
      1., -1.,
      0., 0.,
    ];

    let memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
    let vertices_location = vertices.as_ptr() as u32 / 4;
    let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(vertices_location, vertices_location + vertices.len() as u32);
    let buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

    Self {
      u_color: gl.get_uniform_location(&program, "u_color").unwrap(),
      u_transform: gl.get_uniform_location(&program, "u_transform").unwrap(),
      vertex_length: vertices.len(),
      vertex_buffer: buffer,
      program: program,
    }
  }
}

impl SceneObject for Color2D {
  fn name(&self) -> &str {
    "C2D"
  }
  fn draw_self(&self, gl: Option<&GL>){
    let gl = gl.unwrap();
    gl.use_program(Some(&self.program));
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);
    gl.uniform4f(Some(&self.u_color), 0.0, 0.5, 0.5, 1.0);

    let t_matrix = translation_matrix(0.0,0.0,0.0);
    let s_matrix = scale_matrix(1.0,1.0,1.0);

    let model_matrix = matrix_mul(t_matrix, s_matrix);
    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &model_matrix);
    gl.draw_arrays(GL::TRIANGLES, 0, (self.vertex_length / 2) as i32)
  }
}