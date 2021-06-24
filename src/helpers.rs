use web_sys::*;
use std::fmt::Write;
use web_sys::WebGlRenderingContext as GL;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


pub fn link_program(gl: &GL, vert_src: &str, frag_src: &str) -> Result<WebGlProgram, String> {
  let program = gl.create_program().ok_or_else(|| String::from("Error creating program")).unwrap();
  let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_src).unwrap();
  let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_src).unwrap();

  gl.attach_shader(&program, &vert_shader);
  gl.attach_shader(&program, &frag_shader);
  gl.link_program(&program);

  if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false){
    Ok(program)
  } else {
    Err(
      gl.get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object"))
    )
  }
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
  let shader = gl.create_shader(shader_type).ok_or_else(|| String::from("Error creating shader"))?;
  gl.shader_source(&shader, source);
  gl.compile_shader(&shader);

  if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
    Ok(shader)
  } else {
    Err(
      gl.get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unable to get shader info log"))
    )
  }
}

pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
  let mut matrix = [0.0; 16];
  matrix[0] = 1.0;
  matrix[5] = 1.0;
  matrix[10] = 1.0;
  matrix[15] = 1.0;

  matrix[3] = tx;
  matrix[7] = ty;
  matrix[11] = tz;

  matrix
}

pub fn scale_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
  let mut matrix = [0.0; 16];
  matrix[0] = sx;
  matrix[5] = sy;
  matrix[10] = sz;
  matrix[15] = 1.0;

  matrix
}

pub fn matrix_mul(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
  let mut matrix = [0.0; 16];

  matrix[0] = a[0]*b[0] + a[4]*b[1] + a[8]* b[2] + a[12]*b[3];
  matrix[1] = a[1]*b[0] + a[5]*b[1] + a[9]* b[2] + a[13]*b[3];
  matrix[2] = a[2]*b[0] + a[6]*b[1] + a[10]*b[2] + a[14]*b[3];
  matrix[3] = a[3]*b[0] + a[7]*b[1] + a[11]*b[2] + a[15]*b[3];

  matrix[4] = a[0]*b[4] + a[4]*b[5] + a[8]* b[6] + a[12]*b[7];
  matrix[5] = a[1]*b[4] + a[5]*b[5] + a[9]* b[6] + a[13]*b[7];
  matrix[6] = a[2]*b[4] + a[6]*b[5] + a[10]*b[6] + a[14]*b[7];
  matrix[7] = a[3]*b[4] + a[7]*b[5] + a[11]*b[6] + a[15]*b[7];

  matrix[8]  = a[0]*b[8] + a[4]*b[9] + a[8]* b[10] + a[12]*b[11];
  matrix[9]  = a[1]*b[8] + a[5]*b[9] + a[9]* b[10] + a[13]*b[11];
  matrix[10] = a[2]*b[8] + a[6]*b[9] + a[10]*b[10] + a[14]*b[11];
  matrix[11] = a[3]*b[8] + a[7]*b[9] + a[11]*b[10] + a[15]*b[11];

  matrix[12] = a[0]*b[12] + a[4]*b[13] + a[8]* b[14] + a[12]*b[15];
  matrix[13] = a[1]*b[12] + a[5]*b[13] + a[9]* b[14] + a[13]*b[15];
  matrix[14] = a[2]*b[12] + a[6]*b[13] + a[10]*b[14] + a[14]*b[15];
  matrix[15] = a[3]*b[12] + a[7]*b[13] + a[11]*b[14] + a[15]*b[15];

  matrix
}

pub fn print_matrix(a: [f32; 16]) {
  let mut s = String::new();
  write!(s, "{} {} {} {}\n", a[0], a[1], a[2], a[3]).unwrap();
  write!(s, "{} {} {} {}\n", a[4], a[5], a[6], a[7]).unwrap();
  write!(s, "{} {} {} {}\n", a[8], a[9], a[10], a[11]).unwrap();
  write!(s, "{} {} {} {}\n", a[12], a[13], a[14], a[15]).unwrap();
  log(&s);
}