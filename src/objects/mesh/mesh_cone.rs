use core::f32;
use glam::Vec3;
use uuid::Uuid;

use crate::{
  mesh_implement_partial_Object,
  objects::{
    instanced::instanced_cube::SIZE_F32,
    mesh::mesh_object::MeshObject,
    object::Object,
    octree::octree_object::OctreeObject
  },
  opengl::{ebo::EBO, program::Program, vao::VAO, vbo::VBO},
  utils::{material::Material, transform::Transform}
};

pub struct MeshCone {
  pub id: Uuid,
  pub name: String,

  pub radius: f32,
  pub height: f32,
  pub subdivisions: u32,

  pub transform: Transform,
  pub material: Material,

  pub vao: VAO,
  pub vbo: VBO,
  pub ebo: EBO,

  indices: Vec<usize>,
}

impl MeshCone {
  pub fn new(name: String, radius: f32, height: f32, subdivisions: u32) -> Self {
    let vao = VAO::new();
    let vbo = VBO::new();
    let ebo = EBO::new();
    vao.bind();
    vbo.bind();
    ebo.bind();

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let base_normal = Vec3::NEG_Y;
    let base_origin = Vec3::new(0.0, -height / 2.0, 0.0);

    vertices.push(base_origin);
    vertices.push(base_normal);

    let base_center_index = 0;

    for i in 0..=subdivisions {
      let angle = (i as f32 / subdivisions as f32) * f32::consts::PI * 2.0;
      let x = radius * angle.cos();
      let z = radius * angle.sin();

      vertices.push(Vec3::new(x, -height / 2.0, z));
      vertices.push(base_normal);

      if i > 0 {
        let current = base_center_index + i as usize + 1;
        let prev = base_center_index + i as usize;

        indices.push(base_center_index);
        indices.push(prev);
        indices.push(current);
      }
    }

    let side_start_index = vertices.len() / 2;

    for i in 0..=subdivisions {
      let angle = (i as f32 / subdivisions as f32) * f32::consts::PI * 2.0;
      let x = radius * angle.cos();
      let z = radius * angle.sin();

      let normal_x = height * angle.cos();
      let normal_y = radius;
      let normal_z = height * angle.sin();
      let side_normal = Vec3::new(normal_x, normal_y, normal_z).normalize();

      vertices.push(Vec3::new(0.0, height / 2.0, 0.0));
      vertices.push(side_normal);

      vertices.push(Vec3::new(x, -height / 2.0, z));
      vertices.push(side_normal);

      if i > 0 {
        let top_curr = side_start_index + (i as usize * 2);
        let bottom_curr = side_start_index + (i as usize * 2) + 1;
        let bottom_prev = side_start_index + ((i - 1) as usize * 2) + 1;

        indices.push(bottom_prev);
        indices.push(top_curr);
        indices.push(bottom_curr);
      }
    }

    vao.add_attribute(0, 6 * SIZE_F32, 0);
    vao.add_attribute(1, 6 * SIZE_F32, 3 * SIZE_F32);

    let flat_data: Vec<f32> = vertices.iter().flat_map(|v| v.to_array()).collect();
    vbo.send_data(&flat_data);
    ebo.send_data(&indices.iter().map(|i| *i as u32).collect::<Vec<u32>>());

    MeshCone {
      id: Uuid::new_v4(),
      name,
      radius,
      height,
      subdivisions,
      transform: Transform::default(),
      material: Material::default(),
      vao,
      vbo,
      ebo,
      indices,
    }
  }

  pub fn clone(&self) -> Self {
    Self::new(self.name.clone(), self.radius, self.height, self.subdivisions)
  }
}

impl MeshObject for MeshCone {
  fn clone_box(&self) -> Box<dyn MeshObject> {
    Box::new(self.clone())
  }

  fn csg_draw(&self, program: &Program, base_model: glam::Mat4) {
    self.vao.bind();
    self.vbo.bind();
    self.ebo.bind();

    let final_model = base_model * self.transform.build_model();
    program.set_uniform_matrix4f("model", final_model).unwrap();
    self.material.send_to_program(&program);

    unsafe {
      gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, 0 as *const _);
    }
  }
}

impl Object for MeshCone {
  mesh_implement_partial_Object!();

  fn draw(&self, program: &Program, base_transform: Option<Transform>) {
    self.vao.bind();
    self.vbo.bind();
    self.ebo.bind();

    let model_transform = match base_transform {
      Some(t) => &self.transform.concat(&t),
      None => &self.transform,
    };
    model_transform.send_to_program(&program);
    self.material.send_to_program(&program);

    unsafe {
      gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, 0 as *const _);
    }
  }
}
