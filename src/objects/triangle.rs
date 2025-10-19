use crate::{objects::object::Object, opengl::{program::Program, vao::VAO, vbo::VBO}, utils::{material::{Material, BLANK}, transform::Transform}};

#[allow(dead_code)]
pub struct Triangle {
  pub transform: Transform,
  pub material: Material,

  pub vao: VAO,
  pub vbo: VBO,
}

const VERTICES: [f32; 18] = [
//   X     Y    Z     Nx    Ny    Nz
  -0.5, -0.5, 0.0,   0.0,  0.0, -1.0,
   0.5, -0.5, 0.0,   0.0,  0.0, -1.0,
   0.0,  0.5, 0.0,   0.0,  0.0, -1.0,
];

impl Triangle {
  pub fn new(transform: Option<Transform>, material: Option<Material>) -> Self {
    let transform = transform.unwrap_or(Transform::new());
    let material = material.unwrap_or(BLANK.clone());

    let vao = VAO::new();
    let vbo = VBO::new();
    vao.bind();
    vbo.bind();

    vao.add_attribute(0, 6 * 4, 0);
    vao.add_attribute(1, 6 * 4, 3 * 4);
    vbo.send_data(&VERTICES);

    return Triangle {
      transform,
      material,
      vao,
      vbo,
    };
  }
}

impl Object for Triangle {
  fn get_transform(&self) -> &Transform { return &self.transform; }
  fn get_transform_mut(&mut self) -> &mut Transform { return &mut self.transform; }
  fn get_material(&self) -> &Material { return &self.material; }

  fn draw(&self, program: &Program) {
    self.vao.bind();
    self.vbo.bind();

    self.transform.send_to_program(&program);
    self.material.send_to_program(&program);

    unsafe {
      gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
  }

  fn tick(&mut self) {
    // self.transform.translate3f(0.0, 0.0, -0.01);
  }
}
