#![allow(dead_code)]
use web_sys::WebGl2RenderingContext as GL;
use crate::math::matrix::Matrix;
use crate::input::InputState;
use crate::log;
use crate::cameras::PerspectiveCamera;

pub trait SceneObject {
  fn draw_self(&mut self, gl: Option<&GL>, camera: &PerspectiveCamera){}
  fn get_model_matrix(&self) -> &Matrix;
  fn get_mut_model_matrix(&mut self) -> &Matrix;
  fn calc_model_matrix(&mut self, parent_matrix: Option<&Matrix>);

  fn draw(&mut self, gl: Option<&GL>, parent_matrix: Option<&Matrix>, camera: &PerspectiveCamera) {
    self.calc_model_matrix(parent_matrix);
    self.draw_self(gl, camera);
    let p_matrix = self.get_model_matrix().clone();
    match self.mut_children() {
      Some(children) => {
        for i in 0..children.len(){
          let childs = children.get_mut(i);
          match childs {
            Some(child) =>{
              child.draw(gl, Some(&p_matrix), camera);
            },
            None => (),
          }
        }
      },
      None => ()
    }
  }
  // fn draw(&self, gl: Option<&GL>){
  //   self.draw_self(gl);
  //   match self.children() {
  //     Some(children) => {
  //       for child in children.iter() {
  //         child.draw(gl);
  //       }
  //     },
  //     None => ()
  //   }
  // }
  fn name(&self) -> &str {
    "Unimplemented"
  }
  fn mut_children(&mut self) -> Option<&mut Vec<Box<dyn SceneObject>>> {
    None
  }
  fn update_self(&mut self, dt: f32, input: &InputState){
  }
  fn update(&mut self, dt: f32, input: &InputState) {
    self.update_self(dt, input);
    match self.mut_children() {
      Some(children) => {
        for i in 0..children.len(){
          let childs = children.get_mut(i);
          match childs {
            Some(child) =>{
              child.update(dt, input);
            },
            None => (),
          }
        }
      },
      None => ()
    }
  }

  fn children(&self) -> Option<&Vec<Box<dyn SceneObject>>> {
    None
  }
}