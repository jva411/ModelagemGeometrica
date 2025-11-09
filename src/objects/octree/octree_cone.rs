use glam::Vec3;
use uuid::Uuid;

use crate::{derive_Object, impl_partial_OctreeObject, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{AABB, OctreeNode, OctreeNodeType, OctreeObject}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub struct OctreeCone {
  pub radius: f32,
  pub height: f32,
  pub max_depth: u32,
  pub spacing: f32,
  pub volume: f32,
  pub root: Option<OctreeNode>,
  pub instanced_cube: InstancedCube,
  pub transform: Transform,
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
      radius,
      height,
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

impl OctreeObject for OctreeCone {
  impl_partial_OctreeObject!();

  fn get_bounding_box(&self) -> AABB {
    let min = Vec3::new(-self.radius, -self.height / 2.0, -self.radius);
    let max = Vec3::new(self.radius, self.height / 2.0, self.radius);
    AABB { min, max }.transform(self.get_transform())
  }

  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    let aabb = aabb.inverse_transform(self.get_transform());
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

    let mut dist_sq_closest = 0.0;
    for i in [0, 2] {
      if aabb.min[i] > 0.0 {
        dist_sq_closest += aabb.min[i] * aabb.min[i];
      } else if aabb.max[i] < 0.0 {
        dist_sq_closest += aabb.max[i] * aabb.max[i];
      }
    }

    if dist_sq_closest > cone_radius_at_y * cone_radius_at_y {
      let center_aabb = (aabb.min + aabb.max) / 2.0;
      if !is_point_in_cone(center_aabb) {
        return OctreeNodeType::OUT;
      }
    }

    return OctreeNodeType::PARTIAL;
  }
}

derive_Object!(OctreeCone);
