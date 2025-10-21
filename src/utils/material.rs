use glam::Vec3;

use crate::opengl::program::Program;

#[derive(Copy, Clone)]
pub struct Material {
  pub diffuse: Vec3,
  pub specular: Vec3,
  pub shininess: f32,
}

pub const BLANK: Material = Material {
  diffuse: Vec3::ONE,
  specular: Vec3::new(0.4, 0.4, 0.4),
  shininess: 1.0,
};

#[allow(dead_code)]
impl Material {
  pub fn new(diffuse: Vec3, specular: Vec3, shininess: f32) -> Self {
    Material {
      diffuse,
      specular,
      shininess,
    }
  }

  pub fn send_to_program(&self, program: &Program) {
    program.set_uniform_vec3f("material.diffuse", self.diffuse).unwrap();
    program.set_uniform_vec3f("material.specular", self.specular).unwrap();
    program.set_uniform1f("material.shininess", self.shininess).unwrap();
  }
}

impl Default for Material {
  fn default() -> Self {
    BLANK
  }
}
