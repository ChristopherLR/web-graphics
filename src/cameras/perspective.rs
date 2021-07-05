use crate::math::matrix::*;
use std::f32::consts::PI;
use crate::scene_objects::SceneObject;
use crate::input::InputState;
use crate::log;
use glm::perspective;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct PerspectiveCamera {
  pub matrices: Matrices,
  pub fovy: f32,
  pub aspect: f32,
  pub near: f32,
  pub far: f32,
  pub perspective: Matrix,
  pub rotation_speed: f32,
  pub movement_speed: f32,
}

impl PerspectiveCamera {
  pub fn new(height: f32, fovy: f32, aspect: f32, near: f32, far: f32) -> PerspectiveCamera {
    let mut matrices = Matrices::new();
    matrices.translate(0.0, height, 0.0);
    let perspective = Matrix::get_perspective_matrix(aspect, fovy, far, near);

    PerspectiveCamera {
      matrices: matrices,
      fovy: fovy,
      aspect: aspect,
      near: near,
      far: far,
      perspective: perspective,
      rotation_speed: PI/2.0,
      movement_speed: 3.0,
    }
  }

  pub fn get_view_matrix(&self) -> Matrix {
    self.matrices.model_matrix.invert().unwrap()
  }

  pub fn get_perspective_matrix(&self) -> Matrix {
    self.perspective.clone()
  }

  pub fn get_view_direction(&self) -> [f32; 4] {
    let mut view_dir: [f32; 4] = [0.0; 4];
    // view_dir[0] = self.matrices.model_matrix.0[2];
    // view_dir[1] = self.matrices.model_matrix.0[6];
    // view_dir[2] = self.matrices.model_matrix.0[10];
    // view_dir[3] = self.matrices.model_matrix.0[14];
    view_dir
  }

}

impl SceneObject for PerspectiveCamera {
  fn get_model_matrix(&self) -> &Matrix {
    &self.matrices.model_matrix
  }

  fn get_mut_model_matrix(&mut self) -> &Matrix {
    &self.matrices.model_matrix
  }

  fn calc_model_matrix(&mut self, parent_matrix: Option<&Matrix>){
    self.matrices.calc_model_matrix(parent_matrix, false);
  }

  fn update_self(&mut self, dt: f32, input: &InputState) {
    match input.get_key_pressed() {
      (true, x) => { 
        console_log!("{}", x);
        let mut c_pos = self.matrices.get_position();
        match x {
          87 => { 
            c_pos[2]-=0.1;
            &self.matrices.set_position_arr(c_pos); 
          },
          79 => { &self.matrices.translate(0.0, 0.1, 0.0); },
          83 => { 
            c_pos[2]+=0.1;
            &self.matrices.set_position_arr(c_pos);
          },
          76 => { &self.matrices.translate(0.0, -0.1, 0.0); },
          38 => { &self.matrices.rotate_x(-PI/60.0); },
          40 => { &self.matrices.rotate_x(PI/60.0); },
          37 => { &self.matrices.rotate_y(-PI/60.0); },
          39 => { &self.matrices.rotate_y(PI/60.0); },
          33 => { self.fovy += 0.01; },
          34 => { self.fovy -= 0.01; },
          _ => {}
        }
      },
      (false, _) => {}

    };

    // let perspective = Matrix::get_perspective_matrix(self.fovy, self.far, self.near);
    // self.perspective = perspective;
    // self.matrices.rotate_x(0.01);
    // self.matrices.rotate_y(0.01);
    // self.matrices.rotate_z(0.01);
    // self.matrices.rotate_x(0.01);
    // log(&format!("{:?}", self.matrices.get_rotation()))
    // self.matrices.rotate_z(0.01);
  }
}