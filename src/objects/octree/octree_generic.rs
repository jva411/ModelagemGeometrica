use glam::Vec3;
use uuid::Uuid;
use std::{fs, io::BufReader, path::PathBuf};

use crate::{derive_Object, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{OctreeNode, OctreeNodeType, OctreeObject, AABB}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeGeneric {
  pub original_max_depth: u32,
  pub max_depth: u32,
  pub spacing: f32,
  pub original_root: OctreeNode,
  pub root: OctreeNode,
  pub instanced_cube: InstancedCube,
  pub transform: Transform,
}

#[allow(dead_code)]
impl OctreeGeneric {
  pub fn new(
    name: String,
    path: PathBuf,
    max_depth: u32,
    spacing: f32,
    material: Option<Material>,
  ) -> Self {
    let file = fs::File::open(path).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let result = OctreeNode::deserialize(&mut reader, AABB {
      min: Vec3::new(-0.5, -0.5, -0.5),
      max: Vec3::new(0.5, 0.5, 0.5),
    }, 0, max_depth);

    let (root, max_depth_arrived) = result.expect("Failed to deserialize octree");
    let max_depth = max_depth_arrived.max(max_depth);

    let mut object = OctreeGeneric {
      original_max_depth: max_depth,
      max_depth,
      spacing,
      original_root: root.clone(),
      root: root,
      instanced_cube: InstancedCube::new(name, material),
      transform: Transform::new(),
    };

    object.generate_instanced_cube();
    return object;
  }
}

impl OctreeObject for OctreeGeneric {
  fn get_max_depth(&self) -> u32 { self.max_depth }
  fn get_root(&self) -> Option<&OctreeNode> { Some(&self.root) }

  fn get_bounding_box(&self) -> AABB {
    AABB {
      min: Vec3::new(-0.5, -0.5, -0.5),
      max: Vec3::new(0.5, 0.5, 0.5),
    }.transform(&self.transform)
  }

  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    let aabb = aabb.inverse_transform(self.get_transform());

    self.original_root.get_node_type(&aabb)
  }

  fn generate_octree(&mut self) {
    self.transform = self.instanced_cube.transform.clone();
    self.root = OctreeNode::generate_octree(self, self.max_depth);

    self.generate_instanced_cube();
  }

  fn generate_instanced_cube(&mut self) {
    let instances_transforms = self.instanced_cube.get_instances_transforms_mut();
    instances_transforms.clear();

    self.root.generate_transforms(self.spacing, instances_transforms);

    self.instanced_cube.setup_instances();
  }
}

derive_Object!(OctreeGeneric);
