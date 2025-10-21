use glam::Vec3;

use crate::{objects::object::Object, utils::transform::Transform};

#[allow(dead_code)]
pub trait OctreeObject: Object {
  fn get_max_depth(&self) -> u32;
  fn get_root(&self) -> Option<&OctreeNode>;

  fn get_bounding_box(&self) -> AABB;
  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType;

  fn generate_instanced_cube(&mut self);
}

pub struct AABB {
  pub min: Vec3,
  pub max: Vec3,
}

#[derive(Debug)]
pub enum OctreeNodeType {
  IN,
  OUT,
  PARTIAL,
}

pub struct OctreeNode {
  pub aabb: AABB,
  pub node_type: OctreeNodeType,
  pub children: Option<Vec<Box<OctreeNode>>>,
}
impl OctreeNode {
  fn new(aabb: AABB, node_type: OctreeNodeType) -> Self {
    OctreeNode {
      aabb,
      node_type,
      children: None,
    }
  }

  pub fn is_leaf(&self) -> bool { self.children.is_none() }

  pub fn generate_octree(object: &dyn OctreeObject, max_depth: u32) -> Self {
    let root_aabb = object.get_bounding_box();
    let min = root_aabb.min.min_element();
    let max = root_aabb.max.max_element();

    let root_aabb = AABB { min: Vec3::splat(min), max: Vec3::splat(max) };
    let mut root_node = OctreeNode::new(root_aabb, OctreeNodeType::PARTIAL);
    OctreeNode::subdivide_node(object, &mut root_node, 0, max_depth);

    root_node
  }

  fn subdivide_node(object: &dyn OctreeObject, node: &mut OctreeNode, depth: u32, max_depth: u32) {
    if depth >= max_depth {
      return;
    }

    let mut children = Vec::new();
    let mid = (node.aabb.min + node.aabb.max) * 0.5;

    for i in 0..8 {
      let min = Vec3::new(
        if (i & 1) == 0 { node.aabb.min.x } else { mid.x },
        if (i & 2) == 0 { node.aabb.min.y } else { mid.y },
        if (i & 4) == 0 { node.aabb.min.z } else { mid.z },
      );
      let max = Vec3::new(
        if (i & 1) == 0 { mid.x } else { node.aabb.max.x },
        if (i & 2) == 0 { mid.y } else { node.aabb.max.y },
        if (i & 4) == 0 { mid.z } else { node.aabb.max.z },
      );

      let child_aabb = AABB { min, max };
      let child_node_type = object.get_node_type(&child_aabb);
      let mut child_node = OctreeNode::new(child_aabb, child_node_type);

      match child_node.node_type {
        OctreeNodeType::PARTIAL => OctreeNode::subdivide_node(object, &mut child_node, depth + 1, max_depth),
        _ => { }
      }
      children.push(Box::new(child_node));
    }

    node.children = Some(children);
  }


  pub fn generate_transforms(&self, spacing: f32, transforms: &mut Vec<Transform>) {
    if let OctreeNodeType::OUT = self.node_type {
      return;
    }

    if self.is_leaf() {
      let mut transform = Transform::new();
      let center = (self.aabb.min + self.aabb.max) * 0.5;
      let size = self.aabb.max - self.aabb.min;
      transform.translatev3f(center);
      transform.scale3f(size.x * (1.0-spacing), size.y * (1.0-spacing), size.z * (1.0-spacing));
      transforms.push(transform);
      return;
    }

    if let Some(children) = &self.children {
      for child in children {
        child.generate_transforms(spacing, transforms);
      }
    }
  }
}

#[macro_export]
macro_rules! derive_Object {
  ($type:ty) => {
    impl Object for $type {
      fn get_id(&self) -> Uuid { self.instanced_cube.id }
      fn get_name(&self) -> String { self.instanced_cube.name.clone() }
      fn get_name_mut(&mut self) -> &mut String { &mut self.instanced_cube.name }

      fn get_transform(&self) -> &Transform { &self.instanced_cube.transform }
      fn get_transform_mut(&mut self) -> &mut Transform { &mut self.instanced_cube.transform }
      fn get_material(&self) -> &Material { &self.instanced_cube.material }

      fn tick(&mut self) { }

      fn draw(&self, program: &Program) { self.instanced_cube.draw(program); }
    }
  };
}
