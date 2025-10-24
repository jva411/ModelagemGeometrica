use glam::Vec3;
use uuid::Uuid;
use std::{fs, io::BufReader, path::PathBuf};

use crate::{derive_Object, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{OctreeNode, OctreeNodeType, OctreeObject, AABB}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeGeneric {
  pub max_depth: u32,
  pub spacing: f32,
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
    let root = OctreeNode::deserialize(&mut reader, AABB {
      min: Vec3::new(-0.5, -0.5, -0.5),
      max: Vec3::new(0.5, 0.5, 0.5),
    });

    let mut object = OctreeGeneric {
      max_depth,
      spacing,
      root: Some(root.expect("Failed to deserialize octree")),
      instanced_cube: InstancedCube::new(name, material),
      transform: Transform::new(),
    };

    object.generate_instanced_cube();
    return object;
  }
}

impl OctreeObject for OctreeGeneric {
  fn get_max_depth(&self) -> u32 { self.max_depth }
  fn get_root(&self) -> Option<&OctreeNode> { self.root.as_ref() }

  fn get_bounding_box(&self) -> AABB {
    AABB {
      min: Vec3::new(-0.5, -0.5, -0.5),
      max: Vec3::new(0.5, 0.5, 0.5),
    }.transform(&self.transform)
  }

  fn get_node_type(&self, _aabb: &AABB) -> OctreeNodeType {
    panic!("get_node_type should not be called on OctreeGeneric");
  }

  fn generate_octree(&mut self) {
    panic!("generate_octree should not be called on OctreeGeneric");
  }

  fn generate_instanced_cube(&mut self) {
    let instances_transforms = self.instanced_cube.get_instances_transforms_mut();
    instances_transforms.clear();

    let root = self.root.as_ref().unwrap();
    root.generate_transforms(self.spacing, instances_transforms);

    self.instanced_cube.setup_instances();
  }
}

derive_Object!(OctreeGeneric);
