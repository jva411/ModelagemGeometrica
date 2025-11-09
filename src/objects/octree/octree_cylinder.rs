use glam::Vec3;
use uuid::Uuid;

use crate::{derive_Object, impl_partial_OctreeObject, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{AABB, OctreeNode, OctreeNodeType, OctreeObject}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeCylinder {
  pub radius: f32,
  pub height: f32,
  pub max_depth: u32,
  pub spacing: f32,
  pub volume: f32,
  pub root: Option<OctreeNode>,
  pub instanced_cube: InstancedCube,
  pub transform: Transform,
}

#[allow(dead_code)]
impl OctreeCylinder {
  pub fn new(
    name: String,
    radius: f32,
    height: f32,
    max_depth: u32,
    spacing: f32,
    material: Option<Material>,
  ) -> Self {
    let mut object = OctreeCylinder {
      radius,
      height,
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

impl OctreeObject for OctreeCylinder {
  impl_partial_OctreeObject!();

  fn get_bounding_box(&self) -> AABB {
    let half_height = self.height / 2.0;
    let min = Vec3::new(-self.radius, -half_height, -self.radius);
    let max = Vec3::new(self.radius, half_height, self.radius);

    AABB { min, max }.transform(self.get_transform())
  }

  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    let aabb = aabb.inverse_transform(self.get_transform());
    let radius_sq = self.radius * self.radius;
    let half_height = self.height / 2.0;

    if aabb.min.y > half_height || aabb.max.y < -half_height {
      return OctreeNodeType::OUT;
    }

    let mut dist_sq_closest = 0.0;
    for i in [0, 2] {
      if aabb.min[i] > 0.0 {
        dist_sq_closest += aabb.min[i] * aabb.min[i];
      } else if aabb.max[i] < 0.0 {
        dist_sq_closest += aabb.max[i] * aabb.max[i];
      }
    }

    if dist_sq_closest > radius_sq {
      return OctreeNodeType::OUT;
    }

    let dist_sq_farthest_xz = aabb.min.x.abs().max(aabb.max.x.abs()).powi(2)
      + aabb.min.z.abs().max(aabb.max.z.abs()).powi(2);

    if dist_sq_farthest_xz <= radius_sq && aabb.min.y >= -half_height && aabb.max.y <= half_height {
      return OctreeNodeType::IN;
    }

    return OctreeNodeType::PARTIAL;
  }
}


derive_Object!(OctreeCylinder);
