use std::io::{Read, Result, Write};

use glam::{Vec3, Vec4};

use crate::{objects::{instanced::instanced_cube::InstancedCube, object::Object}, utils::transform::Transform};

#[allow(dead_code)]
pub trait OctreeObject: Object {
  fn get_max_depth(&self) -> u32;
  fn get_spacing(&self) -> f32;
  fn get_volume(&self) -> f32;
  fn get_root(&self) -> Option<&OctreeNode>;
  fn get_instanced_cube(&self) -> &InstancedCube;

  fn get_bounding_box(&self) -> AABB;
  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType;

  fn generate_octree(&mut self);
  fn generate_instanced_cube(&mut self);
}

#[derive(Debug, Clone, Copy)]
pub struct AABB {
  pub min: Vec3,
  pub max: Vec3,
}

impl AABB {
  pub fn transform(&self, transform: &Transform) -> AABB {
    let model = transform.build_model();

    let min = model.transform_point3(self.min);
    let max = model.transform_point3(self.max);

    let max_value = max.max_element().abs().max(min.min_element().abs());

    let min = Vec3::splat(-max_value);
    let max = Vec3::splat(max_value);

    AABB { min, max }
  }

  pub fn inverse_transform(&self, transform: &Transform) -> AABB {
    let inverse_model = transform.build_model().inverse();

    let corners = [
      inverse_model * Vec4::new(self.min.x, self.min.y, self.min.z, 1.0),
      inverse_model * Vec4::new(self.max.x, self.min.y, self.min.z, 1.0),
      inverse_model * Vec4::new(self.min.x, self.max.y, self.min.z, 1.0),
      inverse_model * Vec4::new(self.min.x, self.min.y, self.max.z, 1.0),
      inverse_model * Vec4::new(self.max.x, self.max.y, self.min.z, 1.0),
      inverse_model * Vec4::new(self.min.x, self.max.y, self.max.z, 1.0),
      inverse_model * Vec4::new(self.max.x, self.min.y, self.max.z, 1.0),
      inverse_model * Vec4::new(self.max.x, self.max.y, self.max.z, 1.0),
    ];

    let mut model_aabb_min = Vec3::splat(f32::MAX);
    let mut model_aabb_max = Vec3::splat(f32::MIN);
    for corner in corners.iter() {
      let p = corner.truncate();
      model_aabb_min = model_aabb_min.min(p);
      model_aabb_max = model_aabb_max.max(p);
    }

    AABB { min: model_aabb_min, max: model_aabb_max }
  }

  pub fn intersects(&self, other: &AABB) -> bool {
    (self.min.x <= other.max.x && self.max.x >= other.min.x) &&
    (self.min.y <= other.max.y && self.max.y >= other.min.y) &&
    (self.min.z <= other.max.z && self.max.z >= other.min.z)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OctreeNodeType {
  IN,
  OUT,
  PARTIAL,
}

#[derive(Debug, Clone)]
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

  pub fn serialize(&self, writer: &mut impl Write) {
    match self.node_type {
      OctreeNodeType::PARTIAL => {
        if let Some(children) = &self.children {
          writer.write_all(b"(").unwrap();
          for child in children {
            child.serialize(writer);
          }
        } else {
          writer.write_all(b"B").unwrap();
        }
      }
      OctreeNodeType::IN => writer.write_all(b"B").unwrap(),
      OctreeNodeType::OUT => writer.write_all(b"W").unwrap(),
    }
  }

  pub fn deserialize(reader: &mut impl Read, aabb: AABB, depth: u32, max_depth: u32) -> Result<(Self, u32)> {
    let mut buffer = [0; 1];
    reader.read_exact(&mut buffer)?;

    let mut max_depth_arrived = depth;
    let node = match buffer[0] {
      b'(' => {
        if depth >= max_depth {
          let node = OctreeNode::new(aabb, OctreeNodeType::IN);
          OctreeNode::deserialize_skip_node(reader)?;
          return Ok((node, max_depth_arrived));
        }
        let mut node = OctreeNode::new(aabb, OctreeNodeType::PARTIAL);
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
          let result = OctreeNode::deserialize(reader, child_aabb, depth + 1, max_depth);
          if let Ok((child, max_depth)) = result {
            children.push(Box::new(child));
            max_depth_arrived = max_depth_arrived.max(max_depth);
          } else {
            break;
          }
        }
        if children.len() == 8 {
          node.children = Some(children);
        }

        node
      }
      b'B' => OctreeNode::new(aabb, OctreeNodeType::IN),
      b'W' => OctreeNode::new(aabb, OctreeNodeType::OUT),
      _ => panic!("Invalid character in octree file"),
    };

    Ok((node, max_depth_arrived))
  }

  fn deserialize_skip_node(reader: &mut impl Read) -> Result<()> {
    for _ in 0..8 {
      let mut buffer = [0; 1];
      reader.read_exact(&mut buffer)?;

      match buffer[0] {
        b'(' => OctreeNode::deserialize_skip_node(reader)?,
        b'B' => {},
        b'W' => {},
        _ => panic!("Invalid character in octree file"),
      };
    }
    Ok(())
  }

  pub fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    if !self.aabb.intersects(aabb) {
      return OctreeNodeType::OUT;
    }

    if self.is_leaf() {
      return self.node_type;
    }

    let mut has_in = false;
    let mut has_out = false;

    if let Some(children) = &self.children {
      for child in children {
        let child_type = child.get_node_type(aabb);
        match child_type {
          OctreeNodeType::IN => has_in = true,
          OctreeNodeType::OUT => has_out = true,
          OctreeNodeType::PARTIAL => return OctreeNodeType::PARTIAL,
        }
      }
    }

    if has_in && has_out {
      OctreeNodeType::PARTIAL
    } else if has_in {
      OctreeNodeType::IN
    } else {
      OctreeNodeType::OUT
    }
  }
}

#[macro_export]
macro_rules! impl_partial_OctreeObject {
  () => {
    fn get_max_depth(&self) -> u32 { self.max_depth }
    fn get_spacing(&self) -> f32 { self.spacing }
    fn get_volume(&self) -> f32 { self.volume }
    fn get_root(&self) -> Option<&OctreeNode> { self.root.as_ref() }
    fn get_instanced_cube(&self) -> &InstancedCube { &self.instanced_cube }

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

      self.volume = self.instanced_cube.instances_transforms
        .iter()
        .fold(0.0, |acc, t| acc
          + t.scale.x * t.scale.y * t.scale.z / (1.0 - self.spacing).powi(3)
        );
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

      fn draw(&self, program: &Program) { self.instanced_cube.draw_minus_base_transform(program, self.transform.build_model()); }

      fn as_octree_object(&self) -> Option<&dyn OctreeObject> { Some(self) }
      fn as_octree_object_mut(&mut self) -> Option<&mut dyn OctreeObject> { Some(self) }

      fn as_any(&self) -> &dyn std::any::Any where Self: Sized { self }
      fn as_any_mut(&mut self) -> &mut dyn std::any::Any where Self: Sized { self }
    }
  };
}
