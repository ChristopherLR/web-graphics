use super::scene_object::SceneObject;
use crate::math::matrix::{ Matrix, Matrices};
use web_sys::WebGl2RenderingContext as GL;

pub struct Pivot {
  children: Vec<Box<dyn SceneObject>>,
  pub matrices: Matrices,
}

impl Pivot {
  pub fn new() -> Self {
    Self {
      children: Vec::new(),
      matrices: Matrices::new(),
    }
  }

  pub fn add_child(&mut self, child: Box<dyn SceneObject>) {
    self.children.push(child);
  }
}

impl SceneObject for Pivot {
  fn children(&self) -> Option<&Vec<Box<dyn SceneObject>>> {
    Some(&self.children)
  }

  fn get_model_matrix(&self) -> &Matrix {
    &self.matrices.model_matrix
  }
  fn get_mut_model_matrix(&mut self) -> &Matrix {
    &self.matrices.model_matrix
  }

  fn calc_model_matrix(&mut self, parent_matrix: Option<&Matrix>){
    self.matrices.calc_model_matrix(parent_matrix)
  }

  fn update_self(&mut self, dt: f32){
  }

  fn mut_children(&mut self) -> Option<&mut Vec<Box<dyn SceneObject>>> {
    Some(&mut self.children)
  }
  fn name(&self) -> &str {
    "Pivot"
  }
}