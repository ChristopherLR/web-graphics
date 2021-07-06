use crate::math::matrix::*;
use crate::math::vector::*;
use std::f32::consts::PI;
use crate::scene_objects::SceneObject;
use crate::input::InputState;
use crate::log;
use crate::keycode::KeyCode;

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
      rotation_speed: (PI/2.0)/1000.0,
      movement_speed: 3.0/1000.0,
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
    self.matrices.calc_model_matrix(parent_matrix);
  }

  fn update_self(&mut self, dt: f32, input: &InputState) {
    let keys = input.get_keys_pressed();
    let movement = self.movement_speed * dt;
    let rotation = self.rotation_speed * dt;
    let position = self.matrices.get_position();
    let angle = self.matrices.get_rotation();
    let mut new_pos: Vector3<f32> = Vector3::new(0.0,0.0,0.0);
    let mut new_angle: Vector3<f32> = Vector3::new(0.0,0.0,0.0);

    for (_, key) in keys {
      match key {
        KeyCode::W => {
          new_pos.z -= movement;
          // self.matrices.translate(0.0, 0.0, -0.1)
        },
        KeyCode::S => {
          new_pos[2] += movement;
          // self.matrices.translate(0.0, 0.0, 0.1)
        },
        KeyCode::Q => {
          new_pos[0] -= movement;
          // self.matrices.translate(-0.1, 0.0, 0.0)
        },
        KeyCode::E => {
          new_pos[0] += movement;
          // self.matrices.translate(0.1, 0.0, 0.0)
        },
        KeyCode::O => {
          new_pos[1] += movement;
          // self.matrices.translate(0.0, 0.1, 0.0)
        },
        KeyCode::L => {
          new_pos[1] -= movement;
          // self.matrices.translate(0.0, -0.1, 0.0)
        },
        KeyCode::A => {
          new_angle[1] += rotation;
          // self.matrices.rotate_y(PI/60.0)
        },
        KeyCode::D => {
          new_angle[1] -= rotation;
          // self.matrices.rotate_y(-PI/60.0)
        },
        KeyCode::Up => {
          new_angle[0] += rotation;
          // self.matrices.rotate_x(PI/60.0)
        },
        KeyCode::Down => {
          new_angle[0] -= rotation;
          // self.matrices.rotate_x(-PI/60.0)
        },
        _ => {}
      }

      new_pos.rotate_x(angle[0]);
      new_pos.rotate_y(angle[1]);
      new_pos.rotate_z(angle[2]);
      self.matrices.translate(new_pos.x, new_pos.y, new_pos.z);
      self.matrices.rotate_x(new_angle.x);
      self.matrices.rotate_y(new_angle.y);
      self.matrices.rotate_z(new_angle.z);
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