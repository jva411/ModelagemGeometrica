use glam::Vec3;
use uuid::Uuid;

use crate::{derive_Object, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{OctreeNode, OctreeNodeType, OctreeObject, AABB}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeSphere {
  pub radius: f32,
  pub max_depth: u32,
  pub spacing: f32,
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
      root: None,
      instanced_cube: InstancedCube::new(name, material),
      transform: Transform::new(),
    };

    object.generate_octree();
    return object;
  }
}

impl OctreeObject for OctreeSphere {
  fn get_max_depth(&self) -> u32 { self.max_depth }
  fn get_root(&self) -> Option<&OctreeNode> { self.root.as_ref() }

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

  fn generate_octree(&mut self) {
    self.transform = self.instanced_cube.transform.clone();
    let root = OctreeNode::generate_octree(self, self.max_depth);
    self.root = Some(root);

    self.generate_instanced_cube();
  }

  fn generate_instanced_cube(&mut self) {
    let instances_transforms = self.instanced_cube.get_instances_transforms_mut();
    instances_transforms.clear();

    let root = self.root.as_ref().unwrap();
    root.generate_transforms(self.spacing, instances_transforms);

    self.instanced_cube.setup_instances();
  }
}


derive_Object!(OctreeSphere);
