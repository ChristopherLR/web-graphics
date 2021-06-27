use std::fmt::Write;
use crate::log;
use std::ops::Mul;
use std::sync::{ Arc, Mutex };

pub struct Matrix(pub [f32; 16]);

impl Matrix {
  pub fn new() -> Self {
    let mut matrix = [0.0; 16];
    matrix[0] = 1.0;
    matrix[5] = 1.0;
    matrix[10] = 1.0;
    matrix[15] = 1.0;

    Matrix(matrix)
  }
  pub fn ident(&mut self) {
    self.0[0] = 1.0;
    self.0[1] = 0.0;
    self.0[2] = 0.0;
    self.0[3] = 0.0;

    self.0[4] = 0.0;
    self.0[5] = 1.0;
    self.0[6] = 0.0;
    self.0[7] = 0.0;

    self.0[8] = 0.0;
    self.0[9] = 0.0;
    self.0[10] = 1.0;
    self.0[11] = 0.0;

    self.0[12] = 0.0;
    self.0[13] = 0.0;
    self.0[14] = 0.0;
    self.0[15] = 1.0;
  }
  pub fn translate(&mut self, tx: f32, ty: f32, tz: f32) {
    self.0[3] += tx;
    self.0[7] += ty;
    self.0[11] += tz;
  }
  pub fn scale(&mut self, sx: f32, sy: f32, sz: f32) {
    self.0[0] *= sx;
    self.0[5] *= sy;
    self.0[10] *= sz;
  }
  pub fn print(&self){
    let mut s = String::new();
    write!(s, "{} {} {} {}\n", self.0[0],  self.0[1],  self.0[2],  self.0[3]).unwrap();
    write!(s, "{} {} {} {}\n", self.0[4],  self.0[5],  self.0[6],  self.0[7]).unwrap();
    write!(s, "{} {} {} {}\n", self.0[8],  self.0[9],  self.0[10], self.0[11]).unwrap();
    write!(s, "{} {} {} {}\n", self.0[12], self.0[13], self.0[14], self.0[15]).unwrap();
    log(&s);
  }
  pub fn mat_mul(&self, rhs: &Matrix) -> Self {
    let mut matrix = [0.0; 16];

    matrix[0]  = self.0[0]*rhs.0[0] + self.0[4]*rhs.0[1] + self.0[8]* rhs.0[2] + self.0[12]*rhs.0[3];
    matrix[1]  = self.0[1]*rhs.0[0] + self.0[5]*rhs.0[1] + self.0[9]* rhs.0[2] + self.0[13]*rhs.0[3];
    matrix[2]  = self.0[2]*rhs.0[0] + self.0[6]*rhs.0[1] + self.0[10]*rhs.0[2] + self.0[14]*rhs.0[3];
    matrix[3]  = self.0[3]*rhs.0[0] + self.0[7]*rhs.0[1] + self.0[11]*rhs.0[2] + self.0[15]*rhs.0[3];
  
    matrix[4]  = self.0[0]*rhs.0[4] + self.0[4]*rhs.0[5] + self.0[8]* rhs.0[6] + self.0[12]*rhs.0[7];
    matrix[5]  = self.0[1]*rhs.0[4] + self.0[5]*rhs.0[5] + self.0[9]* rhs.0[6] + self.0[13]*rhs.0[7];
    matrix[6]  = self.0[2]*rhs.0[4] + self.0[6]*rhs.0[5] + self.0[10]*rhs.0[6] + self.0[14]*rhs.0[7];
    matrix[7]  = self.0[3]*rhs.0[4] + self.0[7]*rhs.0[5] + self.0[11]*rhs.0[6] + self.0[15]*rhs.0[7];
  
    matrix[8]  = self.0[0]*rhs.0[8] + self.0[4]*rhs.0[9] + self.0[8]* rhs.0[10] + self.0[12]*rhs.0[11];
    matrix[9]  = self.0[1]*rhs.0[8] + self.0[5]*rhs.0[9] + self.0[9]* rhs.0[10] + self.0[13]*rhs.0[11];
    matrix[10] = self.0[2]*rhs.0[8] + self.0[6]*rhs.0[9] + self.0[10]*rhs.0[10] + self.0[14]*rhs.0[11];
    matrix[11] = self.0[3]*rhs.0[8] + self.0[7]*rhs.0[9] + self.0[11]*rhs.0[10] + self.0[15]*rhs.0[11];
  
    matrix[12] = self.0[0]*rhs.0[12] + self.0[4]*rhs.0[13] + self.0[8]* rhs.0[14] + self.0[12]*rhs.0[15];
    matrix[13] = self.0[1]*rhs.0[12] + self.0[5]*rhs.0[13] + self.0[9]* rhs.0[14] + self.0[13]*rhs.0[15];
    matrix[14] = self.0[2]*rhs.0[12] + self.0[6]*rhs.0[13] + self.0[10]*rhs.0[14] + self.0[14]*rhs.0[15];
    matrix[15] = self.0[3]*rhs.0[12] + self.0[7]*rhs.0[13] + self.0[11]*rhs.0[14] + self.0[15]*rhs.0[15];
  
    Matrix(matrix)
  }
}

impl Mul for Matrix {
  type Output = Matrix;

  fn mul(self, rhs: Matrix) -> Matrix {
    let mut matrix = [0.0; 16];

    matrix[0]  = self.0[0]*rhs.0[0] + self.0[4]*rhs.0[1] + self.0[8]* rhs.0[2] + self.0[12]*rhs.0[3];
    matrix[1]  = self.0[1]*rhs.0[0] + self.0[5]*rhs.0[1] + self.0[9]* rhs.0[2] + self.0[13]*rhs.0[3];
    matrix[2]  = self.0[2]*rhs.0[0] + self.0[6]*rhs.0[1] + self.0[10]*rhs.0[2] + self.0[14]*rhs.0[3];
    matrix[3]  = self.0[3]*rhs.0[0] + self.0[7]*rhs.0[1] + self.0[11]*rhs.0[2] + self.0[15]*rhs.0[3];
  
    matrix[4]  = self.0[0]*rhs.0[4] + self.0[4]*rhs.0[5] + self.0[8]* rhs.0[6] + self.0[12]*rhs.0[7];
    matrix[5]  = self.0[1]*rhs.0[4] + self.0[5]*rhs.0[5] + self.0[9]* rhs.0[6] + self.0[13]*rhs.0[7];
    matrix[6]  = self.0[2]*rhs.0[4] + self.0[6]*rhs.0[5] + self.0[10]*rhs.0[6] + self.0[14]*rhs.0[7];
    matrix[7]  = self.0[3]*rhs.0[4] + self.0[7]*rhs.0[5] + self.0[11]*rhs.0[6] + self.0[15]*rhs.0[7];
  
    matrix[8]  = self.0[0]*rhs.0[8] + self.0[4]*rhs.0[9] + self.0[8]* rhs.0[10] + self.0[12]*rhs.0[11];
    matrix[9]  = self.0[1]*rhs.0[8] + self.0[5]*rhs.0[9] + self.0[9]* rhs.0[10] + self.0[13]*rhs.0[11];
    matrix[10] = self.0[2]*rhs.0[8] + self.0[6]*rhs.0[9] + self.0[10]*rhs.0[10] + self.0[14]*rhs.0[11];
    matrix[11] = self.0[3]*rhs.0[8] + self.0[7]*rhs.0[9] + self.0[11]*rhs.0[10] + self.0[15]*rhs.0[11];
  
    matrix[12] = self.0[0]*rhs.0[12] + self.0[4]*rhs.0[13] + self.0[8]* rhs.0[14] + self.0[12]*rhs.0[15];
    matrix[13] = self.0[1]*rhs.0[12] + self.0[5]*rhs.0[13] + self.0[9]* rhs.0[14] + self.0[13]*rhs.0[15];
    matrix[14] = self.0[2]*rhs.0[12] + self.0[6]*rhs.0[13] + self.0[10]*rhs.0[14] + self.0[14]*rhs.0[15];
    matrix[15] = self.0[3]*rhs.0[12] + self.0[7]*rhs.0[13] + self.0[11]*rhs.0[14] + self.0[15]*rhs.0[15];
  
    Matrix(matrix)
  }
}

pub struct Matrices {
  pub model_matrix: Matrix,
  pub translation_matrix: Matrix,
  pub rotation_matrix: Matrix,
  pub scale_matrix: Matrix,
  pub position: [f32; 3],
  pub angle: [f32; 3],
  pub scale: [f32; 3],
}

impl Matrices {
  pub fn new() -> Self {
    Matrices {
      model_matrix: Matrix::new(),
      translation_matrix: Matrix::new(),
      scale_matrix: Matrix::new(),
      rotation_matrix: Matrix::new(),
      position: [0.0, 0.0, 0.0],
      angle: [0.0, 0.0, 0.0],
      scale: [1.0, 1.0, 1.0],
    }
  }
  pub fn calc_model_matrix(&self) -> Matrix {
    let m_mat = &self.model_matrix;
    let t_mat = &self.translation_matrix;
    let r_mat = &self.rotation_matrix;
    let s_mat = &self.scale_matrix;
    let t = m_mat.mat_mul(t_mat);
    let r = t.mat_mul(r_mat);
    let s = r.mat_mul(s_mat);
    Matrix(s.0)
  }
  pub fn set_position(&mut self, tx: f32, ty: f32, tz: f32){
    self.position = [tx, ty, tz];

    self.translation_matrix.0[3] = tx;
    self.translation_matrix.0[7] = ty;
    self.translation_matrix.0[11] = tz;
  }
  pub fn set_position_arr(&mut self, pos: [f32; 3]){
    self.position = pos; 

    self.translation_matrix.0[3] = pos[0];
    self.translation_matrix.0[7] = pos[1];
    self.translation_matrix.0[11] = pos[1];
  }
  pub fn set_rotation(&mut self, rx: f32, ry: f32, rz: f32){
    self.angle = [rx, ry, rz];

    self.rotation_matrix.0[0] =  ry.cos()*rz.cos();
    self.rotation_matrix.0[1] = -ry.cos()*rz.sin();
    self.rotation_matrix.0[2] =  ry.sin();
    self.rotation_matrix.0[3] =  0.0;

    self.rotation_matrix.0[4] =  rx.sin()*ry.sin()*rz.cos() + rx.cos()*rz.sin();
    self.rotation_matrix.0[5] =  rx.cos()*rz.cos() - rx.sin()*ry.sin()*rz.sin();
    self.rotation_matrix.0[6] =  -rx.sin()*ry.cos();
    self.rotation_matrix.0[7] =  0.0;

    self.rotation_matrix.0[8] =  rx.sin()*rz.sin() -rx.cos()*ry.sin()*rz.cos();
    self.rotation_matrix.0[9] =  rx.cos()*ry.sin()*rz.sin() + rx.sin()*rz.cos();
    self.rotation_matrix.0[10]=  rx.cos()*ry.cos();
    self.rotation_matrix.0[11]=  0.0;

    self.rotation_matrix.0[12]= 0.0; 
    self.rotation_matrix.0[13]= 0.0; 
    self.rotation_matrix.0[14]= 0.0; 
    self.rotation_matrix.0[15]= 1.0;
  }
  pub fn set_rotation_arr(&mut self, rotation: [f32; 3]){
    self.set_rotation(rotation[0], rotation[1], rotation[2]);
  }
  pub fn rotate_x(&mut self, radians: f32){
    self.angle[0] += radians; 

    let mut new_rot = [0.0; 16];

    new_rot[0] = 1.0;
    new_rot[1] = 0.0;
    new_rot[2] = 0.0;
    new_rot[3] = 0.0;

    new_rot[4] =  0.0;
    new_rot[5] =  radians.cos();
    new_rot[6] = -radians.sin();
    new_rot[7] =  0.0;

    new_rot[8] = 0.0;
    new_rot[9] = radians.sin();
    new_rot[10]= radians.cos();
    new_rot[11]= 0.0;

    new_rot[12]= 0.0; 
    new_rot[13]= 0.0; 
    new_rot[14]= 0.0; 
    new_rot[15]= 1.0;

    let new_rot = matrix_mul(self.rotation_matrix.0, new_rot);
    self.rotation_matrix.0 = new_rot;
  }
  pub fn rotate_y(&mut self, radians: f32){
    self.angle[1] += radians; 
    let mut new_rot = [0.0; 16];

    new_rot[0] = radians.cos();
    new_rot[1] = 0.0;
    new_rot[2] = radians.sin();
    new_rot[3] = 0.0;

    new_rot[4] = 0.0;
    new_rot[5] = 1.0;
    new_rot[6] = 0.0;
    new_rot[7] = 0.0;

    new_rot[8] = -radians.sin();
    new_rot[9] = 0.0;
    new_rot[10]= radians.cos();
    new_rot[11]= 0.0;

    new_rot[12]= 0.0; 
    new_rot[13]= 0.0; 
    new_rot[14]= 0.0; 
    new_rot[15]= 1.0;

    let new_rot = matrix_mul(self.rotation_matrix.0, new_rot);
    self.rotation_matrix.0 = new_rot;
  }
  pub fn rotate_z(&mut self, radians: f32){
    self.angle[2] += radians; 
    let mut new_rot = [0.0; 16];

    new_rot[0] =  radians.cos();
    new_rot[1] = -radians.sin();
    new_rot[2] = 0.0;
    new_rot[3] = 0.0;

    new_rot[4] = radians.sin();
    new_rot[5] = radians.cos();
    new_rot[6] = 0.0;
    new_rot[7] = 0.0;

    new_rot[8] = 0.0;
    new_rot[9] = 0.0;
    new_rot[10]= 1.0;
    new_rot[11]= 0.0;

    new_rot[12]= 0.0; 
    new_rot[13]= 0.0; 
    new_rot[14]= 0.0; 
    new_rot[15]= 1.0;

    let new_rot = matrix_mul(self.rotation_matrix.0, new_rot);
    self.rotation_matrix.0 = new_rot;
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