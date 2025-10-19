use glam::{Mat4, Quat, Vec3};

use crate::opengl::program::Program;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
  pub translation: Vec3,
  pub rotation: Quat,
  pub scale: Vec3,
}

#[allow(dead_code)]
impl Transform {
  pub fn new() -> Self {
    Transform {
      translation: Vec3::ZERO,
      rotation: Quat::IDENTITY,
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

  pub fn rotate3f(&mut self, x: f32, y: f32, z: f32) {
    self.rotation = Quat::from_rotation_x(x) * self.rotation;
    self.rotation = Quat::from_rotation_y(y) * self.rotation;
    self.rotation = Quat::from_rotation_z(z) * self.rotation;
  }

  pub fn rotate_x(&mut self, x: f32) {
    self.rotation = Quat::from_rotation_x(x) * self.rotation;
  }
  pub fn rotate_y(&mut self, y: f32) {
    self.rotation = Quat::from_rotation_y(y) * self.rotation;
  }
  pub fn rotate_z(&mut self, z: f32) {
    self.rotation = Quat::from_rotation_z(z) * self.rotation;
  }

  pub fn build_model(&self) -> Mat4 {
    Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
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
