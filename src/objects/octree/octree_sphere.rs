use glam::Vec3;
use uuid::Uuid;

use crate::{octree_derive_Object, impl_partial_OctreeObject, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{AABB, OctreeNode, OctreeNodeType, OctreeObject}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeSphere {
  pub radius: f32,
  pub max_depth: u32,
  pub spacing: f32,
  pub volume: f32,
  pub root: Option<OctreeNode>,
  pub instanced_cube: InstancedCube,
  pub transform: Transform,
}

#[allow(dead_code)]
impl OctreeSphere {
  pub fn new(
    name: String,
    radius: f32,
    max_depth: u32,
    spacing: f32,
    material: Option<Material>,
  ) -> Self {

    let mut object = OctreeSphere {
      radius,
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
}

impl OctreeObject for OctreeSphere {
  impl_partial_OctreeObject!();

  fn get_bounding_box(&self) -> AABB {
    let min = -Vec3::splat(self.radius);
    let max = Vec3::splat(self.radius);
    AABB { min, max }.transform(&self.transform)
  }

  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    let aabb = aabb.inverse_transform(self.get_transform());
    let radius_sq = self.radius * self.radius;

    let mut dist_sq_closest = 0.0;
    for i in 0..3 {
      if aabb.min[i] > 0.0 {
        dist_sq_closest += aabb.min[i] * aabb.min[i];
      } else if aabb.max[i] < 0.0 {
        dist_sq_closest += aabb.max[i] * aabb.max[i];
      }
    }

    if dist_sq_closest > radius_sq {
      return OctreeNodeType::OUT;
    }

    let mut dist_sq_farthest = 0.0;
    for i in 0..3 {
      let farthest_coord = aabb.min[i].abs().max(aabb.max[i].abs());
      dist_sq_farthest += farthest_coord * farthest_coord;
    }

    if dist_sq_farthest <= radius_sq {
      return OctreeNodeType::IN;
    }

    return OctreeNodeType::PARTIAL;
  }
}


octree_derive_Object!(OctreeSphere);
