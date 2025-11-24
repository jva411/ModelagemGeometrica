use glam::Vec3;
use uuid::Uuid;
use std::{fs, io::BufReader, path::PathBuf};

use crate::{octree_derive_Object, impl_partial_OctreeObject, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{AABB, OctreeNode, OctreeNodeType, OctreeObject}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeGeneric {
  pub original_max_depth: u32,
  pub max_depth: u32,
  pub spacing: f32,
  pub volume: f32,
  pub original_root: OctreeNode,
  pub root: Option<OctreeNode>,
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
      volume: 0.0,
      original_root: root.clone(),
      root: Some(root),
      instanced_cube: InstancedCube::new(name, material),
      transform: Transform::new(),
    };

    object.generate_instanced_cube();
    return object;
  }
}

impl OctreeObject for OctreeGeneric {
  impl_partial_OctreeObject!();

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
}

octree_derive_Object!(OctreeGeneric);
