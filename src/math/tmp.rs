use std::fmt::Write;
use crate::log;
use std::ops::Mul;
use std::sync::{ Arc, Mutex };
use std::f32::consts::PI;
use na::*;
use glm::*;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub type Matrix = SMatrix<f32, 4, 4>;

pub struct Matrices {
  pub model_matrix: Matrix,
  position: [f32; 3],
  angle: [f32; 3],
  scale: [f32; 3],
}

impl Matrices {
  pub fn new() -> Self {
    Matrices {
      model_matrix: Matrix::identity(),
      position: [0.0, 0.0, 0.0],
      angle: [0.0, 0.0, 0.0],
      scale: [1.0, 1.0, 1.0],
    }
  }
  pub fn calc_model_matrix(&mut self, parent: Option<&Matrix>) {
    // match parent {
    //   Some(p) => { self.model_matrix.from(Some(p));},
    //   None => { self.model_matrix.identity();}
    // }
    let mat = Matrix::identity();
    let angle: TVec3<f32> = TVec3::new(self.position[0], self.position[1], self.position[2]);
    let scale: TVec3<f32> = TVec3::new(self.scale[0], self.scale[1], self.scale[2]);
    let mat = translate(&mat, &angle);
    let mat = rotate_y(&mat, self.angle[1]);
    let mat = rotate_x(&mat, self.angle[0]);
    let mat = rotate_z(&mat, self.angle[2]);
    let mat = glm::scale(&mat, &scale);
    self.model_matrix = mat;
    // self.model_matrix.translate(self.position);
    // self.model_matrix.rotate_y(self.angle[1]);
    // self.model_matrix.rotate_x(self.angle[0]);
    // self.model_matrix.rotate_z(self.angle[2]);
    // self.model_matrix.scale_arr(self.scale);
  }
  pub fn get_position(&self) -> [f32; 3] {
    self.position
  }
  pub fn set_position(&mut self, tx: f32, ty: f32, tz: f32){
    self.position = [tx, ty, tz];
  }
  pub fn set_position_arr(&mut self, pos: [f32; 3]){
    self.position = pos; 
  }
  pub fn translate(&mut self, tx: f32, ty: f32, tz: f32){
    self.position[0] += tx;
    self.position[1] += ty;
    self.position[2] += tz;
  }
  pub fn get_rotation(&self) -> [f32; 3] {
      self.angle
  }
  pub fn set_rotation(&mut self, rx: f32, ry: f32, rz: f32){
    self.angle = [rx, ry, rz];
  }
  pub fn set_rotation_arr(&mut self, rotation: [f32; 3]){
    self.set_rotation(rotation[0], rotation[1], rotation[2]);
  }
  pub fn rotate_x(&mut self, radians: f32){
    self.angle[0] += radians; 
  }
  pub fn rotate_y(&mut self, radians: f32){
    self.angle[1] += radians; 
  }
  pub fn rotate_z(&mut self, radians: f32){
    self.angle[2] += radians; 
  }
  pub fn get_scale(&self) -> [f32; 3] {
    self.scale
  }
  pub fn scale(&mut self, sx: f32, sy: f32, sz: f32){
    self.scale[0] *= sx;
    self.scale[1] *= sy;
    self.scale[2] *= sz;
  }
  pub fn scale_arr(&mut self, scale: [f32; 3]){
    self.scale(scale[0], scale[1], scale[2]);
  }
  pub fn set_scale(&mut self, sx: f32, sy: f32, sz: f32){
    self.scale = [sx, sy, sz];
  }
  pub fn set_scale_arr(&mut self, scale: [f32; 3]){
    self.scale = scale;
  }

}

pub fn print_matrix(a: &[f32]) {
  let mut s = String::new();
  write!(s, "{} {} {} {}\n", a[0],  a[1],  a[2],  a[3]).unwrap();
  write!(s, "{} {} {} {}\n", a[4],  a[5],  a[6],  a[7]).unwrap();
  write!(s, "{} {} {} {}\n", a[8],  a[9],  a[10], a[11]).unwrap();
  write!(s, "{} {} {} {}\n", a[12], a[13], a[14], a[15]).unwrap();
  log(&s);
}