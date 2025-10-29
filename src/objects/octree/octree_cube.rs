use glam::Vec3;
use uuid::Uuid;

use crate::{derive_Object, impl_partial_OctreeObject, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{AABB, OctreeNode, OctreeNodeType, OctreeObject}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeCube {
  pub size: Vec3,
  pub max_depth: u32,
  pub spacing: f32,
  pub volume: f32,
  pub root: Option<OctreeNode>,
  pub instanced_cube: InstancedCube,
  pub transform: Transform,
}

#[allow(dead_code)]
impl OctreeCube {
  pub fn new(
    name: String,
    size: Vec3,
    max_depth: u32,
    spacing: f32,
    material: Option<Material>,

  ) -> Self {
    let mut object = OctreeCube {
      size,
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

impl OctreeObject for OctreeCube {
  impl_partial_OctreeObject!();

  fn get_bounding_box(&self) -> AABB {
    let half_size = self.size / 2.0;
    AABB {
      min: -half_size,
      max: half_size,
    }.transform(&self.transform)
  }

  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    let aabb = aabb.inverse_transform(&self.transform);
    let half_size = self.size / 2.0;
    if aabb.max.x <= half_size.x && aabb.min.x >= -half_size.x &&
      aabb.max.y <= half_size.y && aabb.min.y >= -half_size.y &&
      aabb.max.z <= half_size.z && aabb.min.z >= -half_size.z {
      return OctreeNodeType::IN;
    }

    if aabb.min.x > half_size.x || aabb.max.x < -half_size.x ||
      aabb.min.y > half_size.y || aabb.max.y < -half_size.y ||
      aabb.min.z > half_size.z || aabb.max.z < -half_size.z {
      return OctreeNodeType::OUT;
    }

    OctreeNodeType::PARTIAL
  }
}

derive_Object!(OctreeCube);
