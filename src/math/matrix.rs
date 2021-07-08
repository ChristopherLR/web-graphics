use std::fmt::Write;
use crate::log;
use std::ops::Mul;
use std::sync::{ Arc, Mutex };
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Matrix(pub [f32; 16]);

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


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
  pub fn clone(&self) -> Matrix {
    let mut matrix = [0.0; 16];

    matrix[0]  = self.0[0];
    matrix[1]  = self.0[1];
    matrix[2]  = self.0[2];
    matrix[3]  = self.0[3];
  
    matrix[4]  = self.0[4];
    matrix[5]  = self.0[5];
    matrix[6]  = self.0[6];
    matrix[7]  = self.0[7];
  
    matrix[8]  = self.0[8];
    matrix[9]  = self.0[9];
    matrix[10] = self.0[10];
    matrix[11] = self.0[11];
  
    matrix[12] = self.0[12];
    matrix[13] = self.0[13];
    matrix[14] = self.0[14];
    matrix[15] = self.0[15];
  
    Matrix(matrix)
  }
  pub fn translate_arr(&mut self, t: [f32; 3]){
    self.translate(t[0], t[1], t[2])
  }
  pub fn translate(&mut self, x: f32, y: f32, z: f32){
    let tmp_0 = self.0[0];
    let tmp_1 = self.0[1];
    let tmp_2 = self.0[2];
    let tmp_3 = self.0[3];
    self.0[3] = tmp_0 * x + tmp_1 * y + tmp_2 * z + tmp_3;

    let tmp_0 = self.0[4];
    let tmp_1 = self.0[5];
    let tmp_2 = self.0[6];
    let tmp_3 = self.0[7];
    self.0[7] = tmp_0 * x + tmp_1 * y + tmp_2 * z + tmp_3;

    let tmp_0 = self.0[8];
    let tmp_1 = self.0[9];
    let tmp_2 = self.0[10];
    let tmp_3 = self.0[11];
    self.0[11] = tmp_0 * x + tmp_1 * y + tmp_2 * z + tmp_3;

    let tmp_0 = self.0[12];
    let tmp_1 = self.0[13];
    let tmp_2 = self.0[14];
    let tmp_3 = self.0[15];
    self.0[15] = tmp_0 * x + tmp_1 * y + tmp_2 * z + tmp_3;
  }
  pub fn set_translation(&mut self, tx: f32, ty: f32, tz: f32){
    self.ident();
    self.0[3] = tx;
    self.0[7] = ty;
    self.0[11] = tz;
  }
  pub fn set_translation_arr(&mut self, pos: [f32; 3]){
    self.set_translation(pos[0], pos[1], pos[2])
  }

  pub fn rotate_x(&mut self, radians: f32){
    let (rxc, rxs) = (radians.cos(), radians.sin());

    let tmp_1 = self.0[1];
    let tmp_2 = self.0[2];
    self.0[1] = tmp_1*rxc + tmp_2*rxs;
    self.0[2] = tmp_2*rxc - tmp_1*rxs;

    let tmp_1 = self.0[5];
    let tmp_2 = self.0[6];
    self.0[5] = tmp_1*rxc + tmp_2*rxs;
    self.0[6] = tmp_2*rxc - tmp_1*rxs;

    let tmp_1 = self.0[9];
    let tmp_2 = self.0[10];
    self.0[9] = tmp_1*rxc + tmp_2*rxs;
    self.0[10]= tmp_2*rxc - tmp_1*rxs;

    let tmp_1 = self.0[13];
    let tmp_2 = self.0[14];
    self.0[13]= tmp_1*rxc + tmp_2*rxs;
    self.0[14]= tmp_2*rxc - tmp_1*rxs;
  }
  pub fn rotate_y(&mut self, radians: f32){
    let (rxc, rxs) = (radians.cos(), radians.sin());

    let tmp_0 = self.0[0];
    let tmp_2 = self.0[2];
    self.0[0] = tmp_0*rxc - tmp_2*rxs;
    self.0[2] = tmp_0*rxs + tmp_2*rxc;

    let tmp_0 = self.0[4];
    let tmp_2 = self.0[6];
    self.0[4] = tmp_0*rxc - tmp_2*rxs;
    self.0[6] = tmp_0*rxs + tmp_2*rxc;

    let tmp_0 = self.0[8];
    let tmp_2 = self.0[10];
    self.0[8]  = tmp_0*rxc - tmp_2*rxs;
    self.0[10] = tmp_0*rxs + tmp_2*rxc;

    let tmp_0 = self.0[12];
    let tmp_2 = self.0[14];
    self.0[12] = tmp_0*rxc - tmp_2*rxs;
    self.0[14] = tmp_0*rxs + tmp_2*rxc;

  }
  pub fn rotate_z(&mut self, radians: f32){
    let (rxc, rxs) = (radians.cos(), radians.sin());

    let tmp_0 = self.0[0];
    let tmp_1 = self.0[1];
    self.0[0] = tmp_0*rxc + tmp_1*rxs;
    self.0[1] = tmp_1*rxc - tmp_0*rxs;

    let tmp_0 = self.0[4];
    let tmp_1 = self.0[5];
    self.0[4] = tmp_0*rxc + tmp_1*rxs;
    self.0[5] = tmp_1*rxc - tmp_0*rxs;

    let tmp_0 = self.0[8];
    let tmp_1 = self.0[9];
    self.0[8] = tmp_0*rxc + tmp_1*rxs;
    self.0[9] = tmp_1*rxc - tmp_0*rxs;

    let tmp_0 = self.0[12];
    let tmp_1 = self.0[13];
    self.0[12] = tmp_0*rxc + tmp_1*rxs;
    self.0[13] = tmp_1*rxc - tmp_0*rxs;

  }
  pub fn set_rotation(&mut self, rx: f32, ry: f32, rz: f32){
    let (rxc, rxs) = (rx.cos(), rx.sin());
    let (ryc, rys) = (ry.cos(), ry.sin());
    let (rzc, rzs) = (rz.cos(), rz.sin());

    self.0[0] =  ryc*rzc;
    self.0[1] = -ryc*rzs;
    self.0[2] =  rys;
    self.0[3] =  0.0;

    self.0[4] =  rxc*rys*rzc + rxc*rzs;
    self.0[5] =  rxc*rzc - rxs*rys*rzs;
    self.0[6] =  -rxs*ryc;
    self.0[7] =  0.0;

    self.0[8] =  rxs*rzs - rxc*rys*rzc;
    self.0[9] =  rxc*rys*rzs + rxs*rzc;
    self.0[10]=  rxc*ryc;
    self.0[11]=  0.0;

    self.0[12]= 0.0; 
    self.0[13]= 0.0; 
    self.0[14]= 0.0; 
    self.0[15]= 1.0;
  }
  pub fn scale(&mut self, sx: f32, sy: f32, sz: f32){
    self.0[0] = self.0[0] * sx;
    self.0[1] = self.0[1] * sy;
    self.0[2] = self.0[2] * sz;
    self.0[3] = self.0[3];

    self.0[4] = self.0[4] * sx;
    self.0[5] = self.0[5] * sy;
    self.0[6] = self.0[6] * sz;
    self.0[7] = self.0[7];

    self.0[8] = self.0[8] * sx;
    self.0[9] = self.0[9] * sy;
    self.0[10]= self.0[10] * sz;
    self.0[11]= self.0[11];

    self.0[12]= self.0[12] * sx;
    self.0[13]= self.0[13] * sy;
    self.0[14]= self.0[14] * sz;
    self.0[15]= self.0[15];
  }
  pub fn scale_arr(&mut self, s: [f32; 3]){
    self.scale(s[0], s[1], s[2])
  }
  pub fn set_scale(&mut self, sx: f32, sy: f32, sz: f32){
    self.ident();
    self.scale(sx, sy, sz);
  }
  pub fn set_scale_arr(&mut self, scale: [f32; 3]){
    self.ident();
    self.scale(scale[0], scale[1], scale[2]);
  }
  pub fn print(&self){
    let mut s = String::new();
    write!(s, "{} {} {} {}\n", self.0[0],  self.0[1],  self.0[2],  self.0[3]).unwrap();
    write!(s, "{} {} {} {}\n", self.0[4],  self.0[5],  self.0[6],  self.0[7]).unwrap();
    write!(s, "{} {} {} {}\n", self.0[8],  self.0[9],  self.0[10], self.0[11]).unwrap();
    write!(s, "{} {} {} {}\n", self.0[12], self.0[13], self.0[14], self.0[15]).unwrap();
    log(&s);
  }
  pub fn print_std(&self, precision: usize) -> String {
    let mut s = String::new();
    write!(s, "{:.4$} {:.4$} {:.4$} {:.4$}\n", self.0[0],  self.0[1],  self.0[2],  self.0[3], precision).unwrap();
    write!(s, "{:.4$} {:.4$} {:.4$} {:.4$}\n", self.0[4],  self.0[5],  self.0[6],  self.0[7], precision).unwrap();
    write!(s, "{:.4$} {:.4$} {:.4$} {:.4$}\n", self.0[8],  self.0[9],  self.0[10], self.0[11], precision).unwrap();
    write!(s, "{:.4$} {:.4$} {:.4$} {:.4$}\n", self.0[12], self.0[13], self.0[14], self.0[15], precision).unwrap();
    s
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
  pub fn set(&mut self, rhs: Option<&Matrix>){
    let other = rhs.unwrap();
    self.0[0] = other.0[0];
    self.0[1] = other.0[1];
    self.0[2] = other.0[2];
    self.0[3] = other.0[3];

    self.0[4] = other.0[4];
    self.0[5] = other.0[5];
    self.0[6] = other.0[6];
    self.0[7] = other.0[7];

    self.0[8] = other.0[8];
    self.0[9] = other.0[9];
    self.0[10]= other.0[10];
    self.0[11]= other.0[11];

    self.0[12]= other.0[12];
    self.0[13]= other.0[13];
    self.0[14]= other.0[14];
    self.0[15]= other.0[15];
  }
  pub fn set_arr(&mut self, rhs: [f32; 16]) {
    self.0[0] = rhs[0];
    self.0[1] = rhs[1];
    self.0[2] = rhs[2];
    self.0[3] = rhs[3];

    self.0[4] = rhs[4];
    self.0[5] = rhs[5];
    self.0[6] = rhs[6];
    self.0[7] = rhs[7];

    self.0[8] = rhs[8];
    self.0[9] = rhs[9];
    self.0[10]= rhs[10];
    self.0[11]= rhs[11];

    self.0[12]= rhs[12];
    self.0[13]= rhs[13];
    self.0[14]= rhs[14];
    self.0[15]= rhs[15];
  }
  // From MESA Library
  pub fn invert(&self) -> Result<Matrix, &'static str> {
    let mut inv: [f32; 16] = [0.0; 16];
    inv[0] = self.0[5]  * self.0[10] * self.0[15] - 
             self.0[5]  * self.0[11] * self.0[14] - 
             self.0[9]  * self.0[6]  * self.0[15] + 
             self.0[9]  * self.0[7]  * self.0[14] +
             self.0[13] * self.0[6]  * self.0[11] - 
             self.0[13] * self.0[7]  * self.0[10];

    inv[4] =  -self.0[4]  * self.0[10] * self.0[15] + 
              self.0[4]  * self.0[11] * self.0[14] + 
              self.0[8]  * self.0[6]  * self.0[15] - 
              self.0[8]  * self.0[7]  * self.0[14] - 
              self.0[12] * self.0[6]  * self.0[11] + 
              self.0[12] * self.0[7]  * self.0[10];

    inv[8] = self.0[4]  * self.0[9] * self.0[15] - 
             self.0[4] * self.0[11] * self.0[13] - 
             self.0[8]  * self.0[5] * self.0[15] + 
             self.0[8]  * self.0[7] * self.0[13] + 
             self.0[12] * self.0[5] * self.0[11] - 
             self.0[12] * self.0[7] * self.0[9];

    inv[12] = -self.0[4]  * self.0[9]  * self.0[14] + 
               self.0[4]  * self.0[10] * self.0[13] +
               self.0[8]  * self.0[5]  * self.0[14] - 
               self.0[8]  * self.0[6]  * self.0[13] - 
               self.0[12] * self.0[5]  * self.0[10] + 
               self.0[12] * self.0[6]  * self.0[9];

    inv[1] = -self.0[1]  * self.0[10] * self.0[15] + 
              self.0[1]  * self.0[11] * self.0[14] + 
              self.0[9]  * self.0[2]  * self.0[15] - 
              self.0[9]  * self.0[3]  * self.0[14] - 
              self.0[13] * self.0[2]  * self.0[11] + 
              self.0[13] * self.0[3]  * self.0[10];

    inv[5] = self.0[0]  * self.0[10] * self.0[15] - 
             self.0[0]  * self.0[11] * self.0[14] - 
             self.0[8]  * self.0[2] * self.0[15] + 
             self.0[8]  * self.0[3] * self.0[14] + 
             self.0[12] * self.0[2] * self.0[11] - 
             self.0[12] * self.0[3] * self.0[10];

    inv[9] = -self.0[0]  * self.0[9] * self.0[15] + 
              self.0[0]  * self.0[11] * self.0[13] + 
              self.0[8]  * self.0[1] * self.0[15] - 
              self.0[8]  * self.0[3] * self.0[13] - 
              self.0[12] * self.0[1] * self.0[11] + 
              self.0[12] * self.0[3] * self.0[9];

    inv[13] = self.0[0]  * self.0[9] * self.0[14] - 
              self.0[0]  * self.0[10] * self.0[13] - 
              self.0[8]  * self.0[1] * self.0[14] + 
              self.0[8]  * self.0[2] * self.0[13] + 
              self.0[12] * self.0[1] * self.0[10] - 
              self.0[12] * self.0[2] * self.0[9];

    inv[2] = self.0[1]  * self.0[6] * self.0[15] - 
              self.0[1]  * self.0[7] * self.0[14] - 
              self.0[5]  * self.0[2] * self.0[15] + 
              self.0[5]  * self.0[3] * self.0[14] + 
              self.0[13] * self.0[2] * self.0[7] - 
              self.0[13] * self.0[3] * self.0[6];

    inv[6] = -self.0[0]  * self.0[6] * self.0[15] + 
              self.0[0]  * self.0[7] * self.0[14] + 
              self.0[4]  * self.0[2] * self.0[15] - 
              self.0[4]  * self.0[3] * self.0[14] - 
              self.0[12] * self.0[2] * self.0[7] + 
              self.0[12] * self.0[3] * self.0[6];

    inv[10] = self.0[0]  * self.0[5] * self.0[15] - 
              self.0[0]  * self.0[7] * self.0[13] - 
              self.0[4]  * self.0[1] * self.0[15] + 
              self.0[4]  * self.0[3] * self.0[13] + 
              self.0[12] * self.0[1] * self.0[7] - 
              self.0[12] * self.0[3] * self.0[5];

    inv[14] = -self.0[0]  * self.0[5] * self.0[14] + 
               self.0[0]  * self.0[6] * self.0[13] + 
               self.0[4]  * self.0[1] * self.0[14] - 
               self.0[4]  * self.0[2] * self.0[13] - 
               self.0[12] * self.0[1] * self.0[6] + 
               self.0[12] * self.0[2] * self.0[5];

    inv[3] = -self.0[1] * self.0[6] * self.0[11] + 
              self.0[1] * self.0[7] * self.0[10] + 
              self.0[5] * self.0[2] * self.0[11] - 
              self.0[5] * self.0[3] * self.0[10] - 
              self.0[9] * self.0[2] * self.0[7] + 
              self.0[9] * self.0[3] * self.0[6];

    inv[7] = self.0[0] * self.0[6] * self.0[11] - 
              self.0[0] * self.0[7] * self.0[10] - 
              self.0[4] * self.0[2] * self.0[11] + 
              self.0[4] * self.0[3] * self.0[10] + 
              self.0[8] * self.0[2] * self.0[7] - 
              self.0[8] * self.0[3] * self.0[6];

    inv[11] = -self.0[0] * self.0[5] * self.0[11] + 
               self.0[0] * self.0[7] * self.0[9] + 
               self.0[4] * self.0[1] * self.0[11] - 
               self.0[4] * self.0[3] * self.0[9] - 
               self.0[8] * self.0[1] * self.0[7] + 
               self.0[8] * self.0[3] * self.0[5];

    inv[15] = self.0[0] * self.0[5] * self.0[10] - 
              self.0[0] * self.0[6] * self.0[9] - 
              self.0[4] * self.0[1] * self.0[10] + 
              self.0[4] * self.0[2] * self.0[9] + 
              self.0[8] * self.0[1] * self.0[6] - 
              self.0[8] * self.0[2] * self.0[5];

    let mut det = self.0[0] * inv[0] + self.0[1] * inv[4] + self.0[2] * inv[8] + self.0[3] * inv[12];
    
    if det == 0.0 {
      return Err("MATRIX::DET::ZERO")
    }

    det = 1.0 / det;

    for i in 0..inv.len() {
      inv[i] = inv[i] * det;
    }

    Ok(Matrix(inv))
  }

  pub fn get_perspective_matrix(aspect: f32, fovy: f32, far: f32, near: f32) -> Matrix {

    let mut perspective: [f32; 16] = [0.0; 16];
    let sar = 1.0/(aspect*(fovy/2.0).tan());
    let s = 1.0/((fovy/2.0).tan());
    let range = far - near;
    let f = (-near-far)/range;
    let f2 = (2.0*far*near)/range;
    perspective[0] = sar;
    perspective[5] = s;
    perspective[10] = -f;
    perspective[11] = -1.0;
    perspective[14] = f2;


    Matrix(perspective)
  }

  pub fn transpose(&mut self){
    let mut matrix = [0.0; 16];

    matrix[0] = self.0[0];
    matrix[1] = self.0[4];
    matrix[2] = self.0[8];
    matrix[3] = self.0[12];

    matrix[4] = self.0[1];
    matrix[5] = self.0[5];
    matrix[6] = self.0[9];
    matrix[7] = self.0[13];

    matrix[8] = self.0[2];
    matrix[9] = self.0[6];
    matrix[10] = self.0[10];
    matrix[11] = self.0[14];

    matrix[12] = self.0[3];
    matrix[13] = self.0[7];
    matrix[14] = self.0[11];
    matrix[15] = self.0[15];

    self.0 = matrix;
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
  position: [f32; 3],
  angle: [f32; 3],
  scale: [f32; 3],
}

impl Matrices {
  pub fn new() -> Self {
    Matrices {
      model_matrix: Matrix::new(),
      position: [0.0, 0.0, 0.0],
      angle: [0.0, 0.0, 0.0],
      scale: [1.0, 1.0, 1.0],
    }
  }
  pub fn calc_model_matrix(&mut self, parent: Option<&Matrix>) {
    match parent {
      Some(p) => { self.model_matrix.set(Some(p));},
      None => { self.model_matrix.ident();}
    }
    self.model_matrix.translate_arr(self.position);
    // self.model_matrix.print();
    self.model_matrix.rotate_y(self.angle[1]);
    
    self.model_matrix.rotate_x(self.angle[0]);
    // self.model_matrix.print();

    self.model_matrix.rotate_z(self.angle[2]);
    if self.angle[0] > 0.0 {
      self.model_matrix.print();
    }
    // self.model_matrix.print();
    self.model_matrix.scale_arr(self.scale);
    // self.model_matrix.print();
    self.model_matrix.transpose();
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
  write!(s, "{} {} {} {}\n", a[0],  a[1],  a[2],  a[3]).unwrap();
  write!(s, "{} {} {} {}\n", a[4],  a[5],  a[6],  a[7]).unwrap();
  write!(s, "{} {} {} {}\n", a[8],  a[9],  a[10], a[11]).unwrap();
  write!(s, "{} {} {} {}\n", a[12], a[13], a[14], a[15]).unwrap();
  log(&s);
}