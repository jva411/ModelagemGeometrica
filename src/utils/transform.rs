use glam::{Mat4, Quat, Vec3};

use crate::opengl::program::Program;

#[derive(Debug, Copy, Clone)]
pub struct Rotation {
  pub yaw: f32,
  pub pitch: f32,
  pub roll: f32,
}

impl Rotation {
  pub fn new() -> Self {
    Rotation {
      yaw: 0.0,
      pitch: 0.0,
      roll: 0.0,
    }
  }

  pub fn to_quat(&self) -> Quat {
    Quat::from_axis_angle(Vec3::Y, self.yaw.to_radians())
      * Quat::from_axis_angle(Vec3::X, self.pitch.to_radians())
      * Quat::from_axis_angle(Vec3::Z, self.roll.to_radians())
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Transform {
  pub translation: Vec3,
  pub rotation: Rotation,
  pub scale: Vec3,
}

#[allow(dead_code)]
impl Transform {
  pub fn new() -> Self {
    Transform {
      translation: Vec3::ZERO,
      rotation: Rotation::new(),
      scale: Vec3::ONE,
    }
  }

  pub fn translate3f(&mut self, x: f32, y: f32, z: f32) {
    self.translation += Vec3::new(x, y, z);
  }
  pub fn translatev3f(&mut self, v: Vec3) {
    self.translation += Vec3::new(v.x, v.y, v.z);
  }

  pub fn scale3f(&mut self, x: f32, y: f32, z: f32) {
    self.scale *= Vec3::new(x, y, z);
  }
  pub fn scalev3f(&mut self, v: Vec3) {
    self.scale *= Vec3::new(v.x, v.y, v.z);
  }

  pub fn add_yaw(&mut self, radians: f32) {
    self.rotation.yaw += radians;
  }
  pub fn add_pitch(&mut self, radians: f32) {
    self.rotation.pitch += radians;
  }
  pub fn add_roll(&mut self, radians: f32) {
    self.rotation.roll += radians;
  }

  pub fn build_model(&self) -> Mat4 {
    Mat4::from_scale_rotation_translation(self.scale, self.rotation.to_quat(), self.translation)
  }

  pub fn send_to_program(&self, program: &Program) {
    program.set_uniform_matrix4f("model", self.build_model()).unwrap();
  }

  pub fn send_to_program_instanced(&self, program: &Program, index: i32) {
    program.set_uniform_matrix4f(&format!("models[{}]", index), self.build_model()).unwrap();
  }
}

impl Default for Transform {
  fn default() -> Self { Self::new() }
}
