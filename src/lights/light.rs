use glam::Vec3;

use crate::{opengl::program::Program, utils::transform::Transform};

#[allow(dead_code)]
pub trait Light {
  fn get_transform(&self) -> &Transform;
  fn get_transform_mut(&mut self) -> &mut Transform;

  fn get_diffuse(&self) -> Vec3;
  fn get_ambient(&self) -> Vec3;
  fn get_specular(&self) -> Vec3;

  fn tick(&mut self);
  fn draw(&self, program: &Program);

  fn send_to_program(&self, program: &Program, index: usize) {
    program.set_uniform_vec3f(format!("lights[{}].position", index).as_str(), self.get_transform().translation).expect("Failed to set light position");
    program.set_uniform_vec3f(format!("lights[{}].diffuse", index).as_str(), self.get_diffuse()).expect("Failed to set light diffuse");
    program.set_uniform_vec3f(format!("lights[{}].specular", index).as_str(), self.get_specular()).expect("Failed to set light specular");
    program.set_uniform_vec3f(format!("lights[{}].ambient", index).as_str(), self.get_ambient()).expect("Failed to set light ambient");
  }
}
