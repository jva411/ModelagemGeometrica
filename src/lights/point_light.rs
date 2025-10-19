use glam::Vec3;

use crate::{lights::light::Light, opengl::program::Program, utils::transform::Transform};

#[allow(dead_code)]
pub struct PointLight {
  pub transform: Transform,

  pub diffuse: Vec3,
  pub specular: Vec3,
  pub ambient: Vec3,
}

#[allow(dead_code)]
impl PointLight {
  pub fn new(diffuse: Vec3, specular: Vec3, ambient: Vec3) -> PointLight {
    PointLight {
      transform: Transform::new(),
      diffuse,
      specular,
      ambient,
    }
  }
}

#[allow(dead_code)]
impl Light for PointLight {
  fn get_transform(&self) -> &Transform { &self.transform }
  fn get_transform_mut(&mut self) -> &mut Transform { &mut self.transform }

  fn get_diffuse(&self) -> Vec3 { self.diffuse }
  fn get_specular(&self) -> Vec3 { self.specular }
  fn get_ambient(&self) -> Vec3 { self.ambient }

  fn tick(&mut self) { }
  fn draw(&self, _program: &Program) { }
}
