use glam::Vec3;

use crate::{objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{OctreeNode, OctreeNodeType, OctreeObject, AABB}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeSphere {
  pub material: Material,
  pub radius: f32,
  pub max_depth: u32,
  pub spacing: f32,
  pub root: Option<OctreeNode>,
  pub instanced_cube: Option<InstancedCube>,
}

#[allow(dead_code)]
impl OctreeSphere {
  pub fn new(
    radius: f32,
    max_depth: u32,
    spacing: f32,
    material: Option<Material>,
  ) -> Self {

    let mut object = OctreeSphere {
      material: material.unwrap_or_default(),
      radius,
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

impl OctreeObject for OctreeSphere {
  fn get_bounding_box(&self) -> AABB {
    let min = -Vec3::splat(self.radius);
    let max = Vec3::splat(self.radius);
    super::octree_object::AABB { min, max }
  }

  fn get_node_type(&self, aabb: &super::octree_object::AABB) -> OctreeNodeType {
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

  fn generate_instanced_cube(&mut self) {
    let mut instanced_cube = InstancedCube::new(Some(self.material.clone()));

    let root = self.root.as_ref().unwrap();
    root.generate_transforms(self.spacing, instanced_cube.get_instances_transforms_mut());

    instanced_cube.setup_instances();
    self.instanced_cube = Some(instanced_cube);
  }
}

impl Object for OctreeSphere {
  fn get_transform(&self) -> &Transform { &self.instanced_cube.as_ref().unwrap().transform }
  fn get_transform_mut(&mut self) -> &mut Transform { &mut self.instanced_cube.as_mut().unwrap().transform }
  fn get_material(&self) -> &Material { &self.material }

  fn tick(&mut self) { }

  fn draw(&self, program: &Program) {
    if let Some(instanced_cube) = &self.instanced_cube {
      instanced_cube.draw(program);
    }
  }
}
