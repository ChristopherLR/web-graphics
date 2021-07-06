use std::f32::consts::PI;
use std::ops::{ Mul, Add, Index, IndexMut };
use std::fmt::Display;
use crate::log;

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug)]
pub struct Vector3<S> {
  pub x: S,
  pub y: S,
  pub z: S,
}

impl <S> Vector3<S> {
  pub fn new(x: S, y: S, z: S) -> Vector3<S> {
    Vector3 { x, y, z }
  }
}

impl <S> Add<&Vector3<S>> for &Vector3<S>
where S: Add + Add<Output = S> + Copy
{
  type Output = Vector3<S>;

  fn add(self, other: &Vector3<S>) -> Vector3<S> {
    Vector3 { 
      x: self.x + other.x, 
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl<S> Index<u32> for Vector3<S> {
  type Output = S;
  fn index(&self, idx: u32) -> &S {
    match idx {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => { panic!("Index out of range"); }
    }
  }
}

impl<S> IndexMut<u32> for Vector3<S> {
  fn index_mut(&mut self, idx: u32) -> &mut S {
    match idx {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      _ => { panic!("Index out of range"); }
    }
  }
}

impl<S> Vector3<S> 
where S: Display
{
  pub fn print(&self){
    console_log!("x: {}, y: {}, z: {}", self.x, self.y, self.z);
  }
}

impl Vector3<f32> {
  pub fn rotate_y(&mut self, radians: f32) {
    let (rc, rs) = (radians.cos(), radians.sin());
    let x = self.x;
    let y = self.y;
    let z = self.z;
    self.x = x * rc + z * rs;
    self.y = y;
    self.z = -x * rs + z * rc;
  }

  pub fn rotate_z(&mut self, radians: f32) {
    let (rc, rs) = (radians.cos(), radians.sin());
    let x = self.x;
    let y = self.y;
    let z = self.z;
    self.x = x * rc - y * rs;
    self.y = x * rs + y * rc;
    self.z = z;
  }

  pub fn rotate_x(&mut self, radians: f32) {
    let (rc, rs) = (radians.cos(), radians.sin());
    let x = self.x;
    let y = self.y;
    let z = self.z;
    self.x = x;
    self.y = y * rc - z * rs;
    self.z = y * rs + z * rc;
  }
}