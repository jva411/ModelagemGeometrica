use glam::Vec3;

use crate::{objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{OctreeNode, OctreeNodeType, OctreeObject, AABB}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeCube {
  pub material: Material,
  pub size: Vec3,
  pub max_depth: u32,
  pub spacing: f32,
  pub root: Option<OctreeNode>,
  pub instanced_cube: Option<InstancedCube>,
}

#[allow(dead_code)]
impl OctreeCube {
  pub fn new(
    size: Vec3,
    max_depth: u32,
    spacing: f32,
    material: Option<Material>,
  ) -> Self {
    let mut object = OctreeCube {
      material: material.unwrap_or_default(),
      size,
      max_depth,
      spacing,
      root: None,
      instanced_cube: None,
    };

    let root = OctreeNode::generate_octree(&object, max_depth);
    object.root = Some(root);
    object.generate_instanced_cube();

    return object;
  }
}

impl OctreeObject for OctreeCube {
  fn get_bounding_box(&self) -> AABB {
    let half_size = self.size / 2.0;
    AABB {
      min: -half_size,
      max: half_size,
    }
  }

  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    let half_size = self.size / 2.0;
    if aabb.max.x <= half_size.x && aabb.min.x >= -half_size.x &&
      aabb.max.y <= half_size.y && aabb.min.y >= -half_size.y &&
      aabb.max.z <= half_size.z && aabb.min.z >= -half_size.z {
      return OctreeNodeType::IN;
    }

    OctreeNodeType::PARTIAL
  }

  fn generate_instanced_cube(&mut self) {
    let mut instanced_cube = InstancedCube::new(Some(self.material.clone()));

    if let Some(root) = self.root.as_ref() {
      root.generate_transforms(
        self.spacing,
        instanced_cube.get_instances_transforms_mut(),
      );
    }

    instanced_cube.setup_instances();
    self.instanced_cube = Some(instanced_cube);
  }
}

impl Object for OctreeCube {
  fn get_transform(&self) -> &Transform { &self.instanced_cube.as_ref().unwrap().transform }
  fn get_transform_mut(&mut self) -> &mut Transform { &mut self.instanced_cube.as_mut().unwrap().transform }
  fn get_material(&self) -> &Material { &self.material }

  fn tick(&mut self) {}

  fn draw(&self, program: &Program) { self.instanced_cube.as_ref().unwrap().draw(program); }
}
