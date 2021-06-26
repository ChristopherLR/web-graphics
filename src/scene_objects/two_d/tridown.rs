use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use crate::helpers::*;
use crate::shaders::{ color_2d_frag, color_2d_vert };
use crate::math::matrix::*;
use crate::scene_objects::SceneObject;

pub struct TriDown {
  program: WebGlProgram,
  vertex_length: usize,
  vertex_buffer: WebGlBuffer, 
  matrices: Matrices,
  u_color: WebGlUniformLocation,
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
      matrices: Matrices::new(),
    }
  }
}

impl SceneObject for TriDown {
  fn draw_self(&self, gl: Option<&GL>){
    let gl = gl.unwrap();
    gl.use_program(Some(&self.program));
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);
    gl.uniform4f(Some(&self.u_color), 0.0, 0.5, 0.5, 1.0);

    let mut model_matrix = self.matrices.calc_model_matrix();
    let mut t_mat = Matrix::new();
    t_mat.translate(0.25, 0.25, 0.0);
    model_matrix = model_matrix * t_mat;
    model_matrix.print();

    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &model_matrix.0);
    gl.draw_arrays(GL::TRIANGLES, 0, (self.vertex_length / 2) as i32)
  }
}