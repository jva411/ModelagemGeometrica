use glam::{Mat4, Vec4};

use crate::{objects::{instanced::instanced_object::InstacedObject, object::Object}, opengl::{ebo::EBO, program::Program, vao::VAO, vbo::VBO}, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct InstancedCube {
  pub material: Material,
  pub transform: Transform,
  pub instances_transforms: Vec<Transform>,

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
      instances_transforms: Vec::new(),
      transform: Transform::new(),
      material: material.unwrap_or_default(),
      vao,
      vbo,
      ebo,
      instance_vbo,
    };
  }
}

impl InstacedObject for InstancedCube {
  fn get_instances_count(&self) -> usize { self.instances_transforms.len() }
  fn get_instances_transforms(&self) -> &Vec<Transform> { &self.instances_transforms }
  fn get_instances_transforms_mut(&mut self) -> &mut Vec<Transform> { &mut self.instances_transforms }

  fn add_instance(&mut self, transform: Transform) { self.instances_transforms.push(transform); }

  fn setup_instances(&mut self) {
    self.vao.bind();
    self.instance_vbo.bind();

    let models: Vec<Mat4> = self.instances_transforms.iter().map(|t| t.build_model()).collect();

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

impl Object for InstancedCube {
  fn get_transform(&self) -> &Transform { &self.transform }
  fn get_transform_mut(&mut self) -> &mut Transform { &mut self.transform }
  fn get_material(&self) -> &Material { &self.material }

  fn tick(&mut self) {}

  fn draw(&self, program: &Program) {
    self.vao.bind();
    self.vbo.bind();
    self.ebo.bind();

    self.material.send_to_program(program);

    unsafe {
      program.set_uniform_matrix4f("baseModel", self.transform.build_model()).expect("Failed to set baseModel uniform");
      gl::DrawElementsInstanced(gl::TRIANGLES, INDICES.len() as i32, gl::UNSIGNED_INT, 0 as *const _, self.instances_transforms.len() as i32);
    }
  }
}
