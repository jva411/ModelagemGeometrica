use core::f32;

use glam::Vec3;
use uuid::Uuid;

use crate::{mesh_implement_partial_Object, objects::{instanced::instanced_cube::SIZE_F32, mesh::mesh_object::MeshObject, object::Object, octree::octree_object::OctreeObject}, opengl::{ebo::EBO, program::Program, vao::VAO, vbo::VBO}, utils::{material::Material, transform::Transform}};

pub struct MeshCylinder {
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

  _vertices: Vec<Vec3>,
  indices: Vec<usize>,
}

impl MeshCylinder {
  pub fn new(name: String, radius: f32, height: f32, subdivisions: u32) -> Self {
    let vao = VAO::new();
    let vbo = VBO::new();
    let ebo = EBO::new();
    vao.bind();
    vbo.bind();
    ebo.bind();

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let circles_normals = vec![
      Vec3::Y,
      Vec3::NEG_Y,
    ];
    let circles_origins = vec![
      Vec3::new(0.0, height / 2.0, 0.0),
      Vec3::new(0.0, -height / 2.0, 0.0),
    ];

    for (&normal, &origin) in circles_normals.iter().zip(circles_origins.iter()) {
      vertices.push(origin);
      vertices.push(normal);
      let start_index = vertices.len() / 2;
      for i in 0..=subdivisions {
        let angle = (i as f32 / subdivisions as f32) * f32::consts::PI * 2.0;
        let x = radius * angle.cos() + origin.x;
        let z = radius * angle.sin() + origin.z;

        vertices.push(Vec3::new(x, origin.y, z));
        vertices.push(normal);

        if i > 0 {
          if normal.y > 0.0 {
            indices.push(start_index - 1);
            indices.push(start_index + i as usize);
            indices.push(start_index + (i-1) as usize);
          } else {
            indices.push(start_index - 1);
            indices.push(start_index + (i-1) as usize);
            indices.push(start_index + i as usize);
          }
        }
      }

      if normal.y > 0.0 {
        indices.push(start_index - 1);
        indices.push(start_index + subdivisions as usize);
        indices.push(start_index);
      } else {
        indices.push(start_index - 1);
        indices.push(start_index);
        indices.push(start_index + subdivisions as usize);
      }
    }

    let start_index = vertices.len() / 2;
    for i in 0..=subdivisions {
      let angle = (i as f32 / subdivisions as f32) * f32::consts::PI * 2.0;
      let x = radius * angle.cos();
      let z = radius * angle.sin();
      let normal = Vec3::new(x, 0.0, z).normalize();

      vertices.push(Vec3::new(x, height / 2.0, z));
      vertices.push(normal);
      vertices.push(Vec3::new(x, -height / 2.0, z));
      vertices.push(normal);

      if i > 0 {
        let top_left = start_index + (i - 1) as usize * 2;
        let bottom_left = start_index + (i - 1) as usize * 2 + 1;
        let top_right = start_index + i as usize * 2;
        let bottom_right = start_index + i as usize * 2 + 1;

        indices.push(bottom_left);
        indices.push(top_left);
        indices.push(bottom_right);

        indices.push(top_left);
        indices.push(top_right);
        indices.push(bottom_right);
      }
    }

    let top_left = start_index + (subdivisions - 1) as usize;
    let bottom_left = start_index + (subdivisions - 1) as usize;
    let top_right = start_index + subdivisions as usize;
    let bottom_right = start_index + subdivisions as usize;

    indices.push(bottom_left);
    indices.push(bottom_right);
    indices.push(top_left);

    indices.push(top_left);
    indices.push(bottom_right);
    indices.push(top_right);

    vao.add_attribute(0, 6 * SIZE_F32, 0);
    vao.add_attribute(1, 6 * SIZE_F32, 3 * SIZE_F32);

    let flat_data: Vec<f32> = vertices.iter().flat_map(|v| v.to_array()).collect();
    vbo.send_data(&flat_data);
    ebo.send_data(&indices.iter().map(|i| *i as u32).collect::<Vec<u32>>());

    MeshCylinder {
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
      _vertices: vertices,
      indices,
    }
  }

  pub fn clone(&self) -> Self {
    Self::new(self.name.clone(), self.radius, self.height, self.subdivisions)
  }

}

impl MeshObject for MeshCylinder {
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

impl Object for MeshCylinder {
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

