use glam::Vec3;
use uuid::Uuid;

use crate::{derive_Object, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{OctreeNode, OctreeNodeType, OctreeObject, AABB}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeCube {
  pub size: Vec3,
  pub max_depth: u32,
  pub spacing: f32,
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
      root: None,
      instanced_cube: InstancedCube::new(name, material),
      transform: Transform::new(),
    };

    object.generate_octree();
    return object;
  }
}

impl OctreeObject for OctreeCube {
  fn get_max_depth(&self) -> u32 { self.max_depth }
  fn get_root(&self) -> Option<&OctreeNode> { self.root.as_ref() }

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

derive_Object!(OctreeCube);
