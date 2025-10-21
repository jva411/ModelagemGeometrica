use glam::Vec3;
use uuid::Uuid;

use crate::{derive_Object, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{OctreeNode, OctreeNodeType, OctreeObject, AABB}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeCylinder {
  pub radius: f32,
  pub height: f32,
  pub max_depth: u32,
  pub spacing: f32,
  pub root: Option<OctreeNode>,
  pub instanced_cube: InstancedCube,
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
      root: None,
      instanced_cube: InstancedCube::new(name, material),
    };

    let root = OctreeNode::generate_octree(&object, max_depth);
    object.root = Some(root);
    object.generate_instanced_cube();

    return object;
  }
}

impl OctreeObject for OctreeCylinder {
  fn get_max_depth(&self) -> u32 { self.max_depth }
  fn get_root(&self) -> Option<&OctreeNode> { self.root.as_ref() }

  fn get_bounding_box(&self) -> AABB {
    let min = Vec3::new(-self.radius, -self.height / 2.0, -self.radius);
    let max = Vec3::new(self.radius, self.height / 2.0, self.radius);
    AABB { min, max }
  }

  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    let radius_sq = self.radius * self.radius;
    let half_height = self.height / 2.0;

    if aabb.min.y > half_height || aabb.max.y < -half_height {
      return OctreeNodeType::OUT;
    }

    let mut dist_sq_closest = 0.0;
    if aabb.min.x > 0.0 {
      dist_sq_closest += aabb.min.x * aabb.min.x;
    } else if aabb.max.x < 0.0 {
      dist_sq_closest += aabb.max.x * aabb.max.x;
    }

    if aabb.min.z > 0.0 {
      dist_sq_closest += aabb.min.z * aabb.min.z;
    } else if aabb.max.z < 0.0 {
      dist_sq_closest += aabb.max.z * aabb.max.z;
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

  fn generate_instanced_cube(&mut self) {
    let root = self.root.as_ref().unwrap();
    root.generate_transforms(self.spacing, self.instanced_cube.get_instances_transforms_mut());

    self.instanced_cube.setup_instances();
  }
}


derive_Object!(OctreeCylinder);
