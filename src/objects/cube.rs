use uuid::Uuid;

use crate::{objects::object::Object, opengl::{ebo::EBO, program::Program, vao::VAO, vbo::VBO}, utils::{material::{Material, BLANK}, transform::Transform}};

#[allow(dead_code)]
pub struct Cube {
  pub id: Uuid,
  pub name: String,

  pub transform: Transform,
  pub material: Material,

  pub vao: VAO,
  pub vbo: VBO,
  pub ebo: EBO,
}

const VERTICES: [f32; (3 + 3) * 24] = [
  // X     Y    Z     Nx    Ny    Nz
  -0.5, -0.5, -0.5,     0.0,  0.0, -1.0,  // 0  Back
   0.5, -0.5, -0.5,     0.0,  0.0, -1.0,  // 1
   0.5,  0.5, -0.5,     0.0,  0.0, -1.0,  // 2
  -0.5,  0.5, -0.5,     0.0,  0.0, -1.0,  // 3

  -0.5, -0.5,  0.5,     0.0,  0.0,  1.0,  // 4  Front
  -0.5,  0.5,  0.5,     0.0,  0.0,  1.0,  // 5
   0.5,  0.5,  0.5,     0.0,  0.0,  1.0,  // 6
   0.5, -0.5,  0.5,     0.0,  0.0,  1.0,  // 7

  -0.5, -0.5, -0.5,    -1.0,  0.0,  0.0,  // 8  Left
  -0.5,  0.5, -0.5,    -1.0,  0.0,  0.0,  // 9
  -0.5,  0.5,  0.5,    -1.0,  0.0,  0.0,  // 10
  -0.5, -0.5,  0.5,    -1.0,  0.0,  0.0,  // 11

   0.5, -0.5, -0.5,     1.0,  0.0,  0.0,  // 12  Right
   0.5, -0.5,  0.5,     1.0,  0.0,  0.0,  // 13
   0.5,  0.5,  0.5,     1.0,  0.0,  0.0,  // 14
   0.5,  0.5, -0.5,     1.0,  0.0,  0.0,  // 15

   0.5,  0.5,  0.5,     0.0,  1.0,  0.0,  // 16  Top
  -0.5,  0.5,  0.5,     0.0,  1.0,  0.0,  // 17
  -0.5,  0.5, -0.5,     0.0,  1.0,  0.0,  // 18
   0.5,  0.5, -0.5,     0.0,  1.0,  0.0,  // 19

   0.5, -0.5,  0.5,     0.0, -1.0,  0.0,  // 20  Bottom
   0.5, -0.5, -0.5,     0.0, -1.0,  0.0,  // 21
  -0.5, -0.5, -0.5,     0.0, -1.0,  0.0,  // 22
  -0.5, -0.5,  0.5,     0.0, -1.0,  0.0,  // 23
];

const INDICES: [u32; 3 * 2 * 6] = [
   0,  2,  1,   0,  3,  2,  // Back
   4,  6,  5,   4,  7,  6,  // Front
   8, 10,  9,   8, 11, 10,  // Left
  12, 14, 13,  12, 15, 14,  // Right
  16, 18, 17,  16, 19, 18,  // Top
  20, 22, 21,  20, 23, 22,  // Bottom
];

const SIZE_F32: u32 = size_of::<f32>() as u32;
const STRIDE: u32 = (3+3) * SIZE_F32;
const SKIPS: [u32; 2] = [0, 3 * SIZE_F32];

#[allow(dead_code)]
impl Cube {
  pub fn new(transform: Option<Transform>, material: Option<Material>) -> Self {
    let transform = transform.unwrap_or(Transform::new());
    let material = material.unwrap_or(BLANK);

    let vao = VAO::new();
    let vbo = VBO::new();
    let ebo = EBO::new();
    vao.bind();
    vbo.bind();
    ebo.bind();

    for i in 0..SKIPS.len() {
      vao.add_attribute(i as u32, STRIDE, SKIPS[i]);
    }

    vbo.send_data(&VERTICES);
    ebo.send_data(&INDICES);

    return Self {
      id: Uuid::new_v4(),
      name: "Cube".to_string(),
      transform,
      material,
      vao,
      vbo,
      ebo,
    };
  }
}

impl Object for Cube {
  fn get_id(&self) -> Uuid { self.id }
  fn get_name(&self) -> String { self.name.clone() }
  fn get_name_mut(&mut self) -> &mut String { &mut self.name }

  fn get_transform(&self) -> &Transform { &self.transform }
  fn get_transform_mut(&mut self) -> &mut Transform { &mut self.transform }
  fn get_material(&self) -> &Material { &self.material }

  fn tick(&mut self) {}

  fn draw(&self, program: &Program) {
    self.vao.bind();
    self.vbo.bind();
    self.ebo.bind();

    self.transform.send_to_program(&program);
    self.material.send_to_program(&program);

    unsafe {
      gl::DrawElements(gl::TRIANGLES, INDICES.len() as i32, gl::UNSIGNED_INT, 0 as *const _);
    }
  }
}
