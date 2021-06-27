use super::scene_object::SceneObject;

pub struct Pivot {
  children: Vec<Box<dyn SceneObject>>,
}

impl Pivot {
  pub fn new() -> Self {
    Self {
      children: Vec::new(),
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
  fn mut_children(&mut self) -> Option<&mut Vec<Box<dyn SceneObject>>> {
    Some(&mut self.children)
  }
  fn name(&self) -> &str {
    "Pivot"
  }
}