use glam::Vec3;
use uuid::Uuid;

use crate::{objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{OctreeNode, OctreeNodeType, OctreeObject, AABB}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeCone {
  pub id: Uuid,
  pub name: String,
  pub material: Material,
  pub radius: f32,
  pub height: f32,
  pub max_depth: u32,
  pub spacing: f32,
  pub root: Option<OctreeNode>,
  pub instanced_cube: Option<InstancedCube>,
}

#[allow(dead_code)]
impl OctreeCone {
  pub fn new(
    name: String,
    radius: f32,
    height: f32,
    max_depth: u32,
    spacing: f32,
    material: Option<Material>,
  ) -> Self {
    let mut object = OctreeCone {
      id: Uuid::new_v4(),
      name,
      material: material.unwrap_or_default(),
      radius,
      height,
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

impl OctreeObject for OctreeCone {
  fn get_max_depth(&self) -> u32 { self.max_depth }
  fn get_root(&self) -> Option<&OctreeNode> { self.root.as_ref() }

  fn get_bounding_box(&self) -> AABB {
    let min = Vec3::new(-self.radius, -self.height / 2.0, -self.radius);
    let max = Vec3::new(self.radius, self.height / 2.0, self.radius);
    AABB { min, max }
  }

  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    let half_height = self.height / 2.0;

    if aabb.min.y > half_height || aabb.max.y < -half_height {
      return OctreeNodeType::OUT;
    }

    let is_point_in_cone = |p: Vec3| -> bool {
      if p.y < -half_height || p.y > half_height {
        return false;
      }
      let cone_radius_at_y = self.radius * (half_height - p.y) / self.height;
      p.x * p.x + p.z * p.z <= cone_radius_at_y * cone_radius_at_y
    };

    let mut all_corners_in = true;
    for i in 0..8 {
      let corner = Vec3::new(
        if (i & 1) == 0 { aabb.min.x } else { aabb.max.x },
        if (i & 2) == 0 { aabb.min.y } else { aabb.max.y },
        if (i & 4) == 0 { aabb.min.z } else { aabb.max.z },
      );
      if !is_point_in_cone(corner) {
        all_corners_in = false;
        break;
      }
    }

    if all_corners_in {
      return OctreeNodeType::IN;
    }

    let y_check = aabb.min.y.max(-half_height).min(half_height);
    let cone_radius_at_y = self.radius * (half_height - y_check) / self.height;

    let closest_x = aabb.min.x.max(-aabb.max.x).min(aabb.max.x);
    let closest_z = aabb.min.z.max(-aabb.max.z).min(aabb.max.z);

    if closest_x * closest_x + closest_z * closest_z > cone_radius_at_y * cone_radius_at_y {
      let center_aabb = (aabb.min + aabb.max) / 2.0;
      if !is_point_in_cone(center_aabb) {
        return OctreeNodeType::OUT;
      }
    }

    return OctreeNodeType::PARTIAL;
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

impl Object for OctreeCone {
  fn get_id(&self) -> Uuid { self.id }
  fn get_name(&self) -> String { self.name.clone() }
  fn get_name_mut(&mut self) -> &mut String { &mut self.name }

  fn get_transform(&self) -> &Transform { &self.instanced_cube.as_ref().unwrap().transform }
  fn get_transform_mut(&mut self) -> &mut Transform { &mut self.instanced_cube.as_mut().unwrap().transform }
  fn get_material(&self) -> &Material { &self.material }

  fn tick(&mut self) {}

  fn draw(&self, program: &Program) { self.instanced_cube.as_ref().unwrap().draw(program); }
}
