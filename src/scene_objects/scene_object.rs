use web_sys::WebGl2RenderingContext as GL;
use crate::log;

pub trait SceneObject {
  fn draw_self(&self, gl: Option<&GL>){}
  fn draw(&mut self, gl: Option<&GL>) {
    self.draw_self(gl);
    match self.mut_children() {
      Some(children) => {
        for i in 0..children.len(){
          let childs = children.get_mut(i);
          match childs {
            Some(child) =>{
              child.draw(gl);
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