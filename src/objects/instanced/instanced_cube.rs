use glam::{Mat4, Vec4};

use crate::{objects::instanced::instanced_object::InstacedObject, opengl::{ebo::EBO, renderer::{self, Renderer}, vao::VAO, vbo::VBO}, utils::{material::{Material, BLANK}, transform::{self, Transform}}};

pub struct InstancedCube {
  pub transforms: Vec<Transform>,
  pub material: Material,

  pub vao: VAO,
  pub vbo: VBO,
  pub ebo: EBO,
  pub instance_vbo: VBO,
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

impl InstancedCube {
  pub fn new(material: Option<Material>) -> Self {
    let transforms = Vec::new();
    let material = material.unwrap_or(BLANK);

    let vao = VAO::new();
    let vbo = VBO::new();
    let ebo = EBO::new();
    let instance_vbo = VBO::new();
    vao.bind();
    vbo.bind();
    ebo.bind();

    for i in 0..SKIPS.len() {
      vao.add_attribute(i as u32, STRIDE, SKIPS[i]);
    }

    vbo.send_data(&VERTICES);
    ebo.send_data(&INDICES);

    return Self {
      transforms,
      material,
      vao,
      vbo,
      ebo,
      instance_vbo,
    };
  }

  pub fn setup_instances(&self) {
    self.vao.bind();
    self.instance_vbo.bind();

    let models: Vec<Mat4> = self.transforms.iter().map(|t| t.build_model()).collect();

    unsafe {
      gl::BufferData(
        gl::ARRAY_BUFFER,
        size_of_val(models.as_slice()) as isize,
        models.as_ptr().cast(),
        gl::STATIC_DRAW
      );
    }

    let mat4_size = size_of::<Mat4>() as i32;
    unsafe {
      for i in 0..4 {
        let location = 2 + i;
        gl::EnableVertexAttribArray(location);
        gl::VertexAttribPointer(
          location,
          4,
          gl::FLOAT,
          gl::FALSE,
          mat4_size,
          (i * size_of::<Vec4>() as u32) as *const _
        );
        gl::VertexAttribDivisor(location, 1);
      }
    }
  }
}

impl InstacedObject for InstancedCube {
  fn get_transform(&self) -> &Vec<Transform> { return &self.transforms; }
  fn get_transform_mut(&mut self) -> &mut Vec<Transform> { return &mut self.transforms; }
  fn get_material(&mut self) -> &mut Material { return &mut self.material; }

  fn add_instance(&mut self, transform: Transform) { self.transforms.push(transform); }

  fn tick(&mut self) {}

  fn draw(&self, renderer: &Renderer) {
    self.vao.bind();
    self.vbo.bind();
    self.ebo.bind();

    self.material.send_to_program(&renderer.instanced_program);

    unsafe {
      gl::DrawElementsInstanced(gl::TRIANGLES, INDICES.len() as i32, gl::UNSIGNED_INT, 0 as *const _, self.transforms.len() as i32);
    }
  }
}
