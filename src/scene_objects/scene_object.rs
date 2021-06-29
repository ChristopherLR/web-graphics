use web_sys::WebGl2RenderingContext as GL;
use crate::math::matrix::Matrix;
use crate::log;

pub trait SceneObject {
  fn draw_self(&mut self, gl: Option<&GL>){}
  fn get_model_matrix(&self) -> &Matrix;
  fn get_mut_model_matrix(&mut self) -> &Matrix;
  fn calc_model_matrix(&mut self, parent_matrix: Option<&Matrix>);

  fn draw(&mut self, gl: Option<&GL>, parent_matrix: Option<&Matrix>) {
    self.calc_model_matrix(parent_matrix);
    self.draw_self(gl);
    let p_matrix = self.get_model_matrix().clone();
    match self.mut_children() {
      Some(children) => {
        for i in 0..children.len(){
          let childs = children.get_mut(i);
          match childs {
            Some(child) =>{
              child.draw(gl, Some(&p_matrix));
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
  fn update_self(&mut self, dt: f32){
  }
  fn update(&mut self, dt: f32) {
    self.update_self(dt);
    match self.mut_children() {
      Some(children) => {
        for i in 0..children.len(){
          let childs = children.get_mut(i);
          match childs {
            Some(child) =>{
              child.update(dt);
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