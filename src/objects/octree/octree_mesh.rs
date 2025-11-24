use std::path::PathBuf;

use glam::Vec3;
use uuid::Uuid;

use crate::{octree_derive_Object, impl_partial_OctreeObject, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{AABB, OctreeNode, OctreeNodeType, OctreeObject}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeMesh {
  pub vertices: Vec<Vec3>,
  pub indices: Vec<usize>,
  pub max_depth: u32,
  pub spacing: f32,
  pub volume: f32,
  pub root: Option<OctreeNode>,
  pub instanced_cube: InstancedCube,
  pub transform: Transform,
}

#[allow(dead_code)]
impl OctreeMesh {
  pub fn new(
    name: String,
    obj_path: PathBuf,
    max_depth: u32,
    spacing: f32,
    material: Option<Material>,
  ) -> Self {
    let (models, _) = tobj::load_obj(
      obj_path,
      &tobj::LoadOptions {
        single_index: true,
        triangulate: true,
        ..Default::default()
      },
    )
    .expect("Failed to load obj file");

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for model in models {
      let mesh = &model.mesh;
      let vertex_offset = vertices.len();

      vertices.extend(
        mesh
          .positions
          .chunks_exact(3)
          .map(|p| Vec3::new(p[0], p[1], p[2])),
      );

      indices.extend(mesh.indices.iter().map(|i| *i as usize + vertex_offset));
    }

    let mut object = OctreeMesh {
      vertices,
      indices,
      max_depth,
      spacing,
      volume: 0.0,
      root: None,
      instanced_cube: InstancedCube::new(name, material),
      transform: Transform::new(),
    };

    object.generate_octree();
    return object;
  }

  fn ray_intersects_triangle(&self, orig: Vec3, dir: Vec3, v0: Vec3, v1: Vec3, v2: Vec3) -> bool {
    const EPSILON: f32 = 0.000001;
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let h = dir.cross(edge2);
    let a = edge1.dot(h);
    if a > -EPSILON && a < EPSILON {
      return false;
    }

    let f = 1.0 / a;
    let s = orig - v0;
    let u = f * s.dot(h);
    if u < 0.0 || u > 1.0 {
      return false;
    }

    let q = s.cross(edge1);
    let v = f * dir.dot(q);
    if v < 0.0 || u + v > 1.0 {
      return false;
    }

    let t = f * edge2.dot(q);
    if t < EPSILON {
      return false;
    }

    return true;
  }
}

impl OctreeObject for OctreeMesh {
  impl_partial_OctreeObject!();

  fn get_bounding_box(&self) -> AABB {
    let mut min = Vec3::splat(f32::MAX);
    let mut max = Vec3::splat(f32::MIN);

    for vertex in &self.vertices {
      min = min.min(*vertex);
      max = max.max(*vertex);
    }

    AABB { min, max }.transform(&self.transform)
  }

  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    let aabb = aabb.inverse_transform(&self.transform);
    let center = (aabb.min + aabb.max) / 2.0;
    let half_size = (aabb.max - aabb.min) / 2.0;

    for i in (0..self.indices.len()).step_by(3) {
      let v0 = self.vertices[self.indices[i]];
      let v1 = self.vertices[self.indices[i + 1]];
      let v2 = self.vertices[self.indices[i + 2]];

      if tri_box_overlap(center, half_size, v0, v1, v2) {
        return OctreeNodeType::PARTIAL;
      }
    }

    let directions = [
      Vec3::X, Vec3::NEG_X,
      Vec3::Y, Vec3::NEG_Y,
      Vec3::Z, Vec3::NEG_Z,
    ];

    for dir in directions {
      let mut intersections = 0;
      for i in (0..self.indices.len()).step_by(3) {
        let v0 = self.vertices[self.indices[i]];
        let v1 = self.vertices[self.indices[i + 1]];
        let v2 = self.vertices[self.indices[i + 2]];

        if self.ray_intersects_triangle(center, dir, v0, v1, v2) {
          intersections += 1;
        }
      }

      if intersections % 2 == 1 {
        return OctreeNodeType::IN;
      }
    }

    OctreeNodeType::OUT
  }
}


octree_derive_Object!(OctreeMesh);

fn tri_box_overlap(boxcenter: Vec3, boxhalfsize: Vec3, trivet0: Vec3, trivet1: Vec3, trivet2: Vec3) -> bool {
  let v0 = trivet0 - boxcenter;
  let v1 = trivet1 - boxcenter;
  let v2 = trivet2 - boxcenter;

  let e0 = v1 - v0;
  let e1 = v2 - v1;
  let e2 = v0 - v2;

  let axes = [
    Vec3::X.cross(e0), Vec3::X.cross(e1), Vec3::X.cross(e2),
    Vec3::Y.cross(e0), Vec3::Y.cross(e1), Vec3::Y.cross(e2),
    Vec3::Z.cross(e0), Vec3::Z.cross(e1), Vec3::Z.cross(e2),
  ];

  for axis in axes {
    let p0 = v0.dot(axis);
    let p1 = v1.dot(axis);
    let p2 = v2.dot(axis);
    let r = boxhalfsize.x * axis.x.abs() + boxhalfsize.y * axis.y.abs() + boxhalfsize.z * axis.z.abs();
    if p0.max(p1.max(p2)) < -r || p0.min(p1.min(p2)) > r {
      return false;
    }
  }

  // Teste dos 3 eixos da caixa
  let box_axes = [Vec3::X, Vec3::Y, Vec3::Z];
  for &axis in &box_axes {
    let p0 = v0.dot(axis);
    let p1 = v1.dot(axis);
    let p2 = v2.dot(axis);
    let r = boxhalfsize.dot(axis.abs());
    if p0.max(p1.max(p2)) < -r || p0.min(p1.min(p2)) > r {
      return false;
    }
  }

  // Teste da normal do triângulo
  let normal = e0.cross(e1);
  let p0 = v0.dot(normal);
  let r = boxhalfsize.dot(normal.abs());
  if p0.abs() > r {
    return false;
  }

  true
}
