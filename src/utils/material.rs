use glam::Vec3;

use crate::opengl::program::Program;

#[derive(Copy, Clone)]
pub struct Material {
  pub diffuse: Vec3,
  pub specular: Vec3,
  pub shininess: f32,

  pub rgb_diffuse: [f32; 3],
  pub rgb_specular: [f32; 3],
}

pub const BLANK: Material = Material {
  diffuse: Vec3::ONE,
  specular: Vec3::new(0.4, 0.4, 0.4),
  shininess: 1.0,

  rgb_diffuse: Vec3::ONE.to_array(),
  rgb_specular: [0.4, 0.4, 0.4],
};

#[allow(dead_code)]
impl Material {
  pub fn new(diffuse: Vec3, specular: Vec3, shininess: f32) -> Self {
    Material {
      diffuse,
      specular,
      shininess,

      rgb_diffuse: diffuse.to_array(),
      rgb_specular: specular.to_array(),
    }
  }

  pub fn send_to_program(&self, program: &Program) {
    let diffuse = Vec3::from_array(self.rgb_diffuse);
    let specular = Vec3::from_array(self.rgb_specular);

    program.set_uniform_vec3f("material.diffuse", diffuse).unwrap();
    program.set_uniform_vec3f("material.specular", specular).unwrap();
    program.set_uniform1f("material.shininess", self.shininess).unwrap();
  }
}

impl Default for Material {
  fn default() -> Self {
    BLANK
  }
}
