use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use wasm_bindgen::JsCast;
use js_sys::WebAssembly;
use js_sys::Object;
use js_sys::{ Float32Array, Float64Array, Uint16Array };
use std::mem::size_of;


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

pub fn size_of_type<T, const N: usize>(_: &[T; N]) -> usize {
  size_of::<T>()
}

pub fn get_ptr<T, const N: usize>(item: &[T; N]) -> u32 {
  item.as_ptr() as u32 / (size_of_type(item) as u32)
}

pub fn create_buffer<T, const N: usize>(gl: &GL, array: &[T; N], array_type: u32) -> WebGlBuffer {
    let buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    let memory = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
    gl.bind_buffer(array_type, Some(&buffer));
    let location = get_ptr(&array);
    let data_array: Object = match size_of_type(array) {
      2 => Object::from(Uint16Array::new(&memory).subarray(location, location + array.len() as u32)),
      4 => Object::from(Float32Array::new(&memory).subarray(location, location + array.len() as u32)),
      8 => Object::from(Float64Array::new(&memory).subarray(location, location + array.len() as u32)),
      _ => panic!("Unimplemented"),
    };
    gl.buffer_data_with_array_buffer_view(array_type, &data_array, GL::STATIC_DRAW);
    buffer
}