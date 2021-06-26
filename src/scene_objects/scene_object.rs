use web_sys::WebGl2RenderingContext as GL;

pub trait SceneObject {
  fn draw_self(&self, gl: Option<&GL>){}
  fn draw(&self, gl: Option<&GL>){
    self.draw_self(gl);
    match self.children() {
      Some(children) => {
        for child in children.iter() {
          child.draw(gl);
        }
      },
      None => ()
    }
  }

  fn children(&self) -> Option<&Vec<Box<dyn SceneObject>>> {
    None
  }
}