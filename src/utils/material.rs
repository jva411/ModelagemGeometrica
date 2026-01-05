use glam::{Mat4, Vec3, Vec4};

use crate::opengl::program::Program;

#[derive(Copy, Clone)]
pub struct Material {
  pub ambient: Vec3,
  pub diffuse: Vec3,
  pub specular: Vec3,
  pub shininess: f32,
}

pub const BLANK: Material = Material {
  ambient: Vec3::ONE,
  diffuse: Vec3::ONE,
  specular: Vec3::new(0.4, 0.4, 0.4),
  shininess: 1.0,
};

#[allow(dead_code)]
impl Material {
  pub fn new(ambient: Vec3, diffuse: Vec3, specular: Vec3, shininess: f32) -> Self {
    Material {
      ambient,
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

  pub fn to_gpu_mat4(&self) -> Mat4 {
    Mat4::from_cols(
      self.ambient.extend(self.shininess),
      self.diffuse.extend(0.0),
      self.specular.extend(0.0),
      Vec4::ZERO,
    )
  }
}

impl Default for Material {
  fn default() -> Self {
    BLANK
  }
}
