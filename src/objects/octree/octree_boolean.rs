use std::{cell::RefCell, fmt::Display, rc::Rc};

use glam::Vec3;
use uuid::Uuid;

use crate::{octree_derive_Object, impl_partial_OctreeObject, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{AABB, OctreeNode, OctreeNodeType, OctreeObject}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanOperator {
  UNION,
  INTERSECTION,
  DIFFERENCE,
}

impl Display for BooleanOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BooleanOperator::UNION => write!(f, "Union"),
      BooleanOperator::INTERSECTION => write!(f, "Intersection"),
      BooleanOperator::DIFFERENCE => write!(f, "Difference"),
    }
  }
}

#[allow(dead_code)]
pub struct OctreeBoolean {
  pub left: Rc<RefCell<dyn Object>>,
  pub right: Rc<RefCell<dyn Object>>,
  pub operator: BooleanOperator,

  pub max_depth: u32,
  pub spacing: f32,
  pub volume: f32,
  pub root: Option<OctreeNode>,
  pub instanced_cube: InstancedCube,
  pub transform: Transform,
}

#[allow(dead_code)]
impl OctreeBoolean {
  pub fn union(left: Rc<RefCell<dyn Object>>, right: Rc<RefCell<dyn Object>>, spacing: f32) -> Self {
    Self::new(left, right, BooleanOperator::UNION, spacing)
  }
  pub fn intersection(left: Rc<RefCell<dyn Object>>, right: Rc<RefCell<dyn Object>>, spacing: f32) -> Self {
    Self::new(left, right, BooleanOperator::INTERSECTION, spacing)
  }
  pub fn difference(left: Rc<RefCell<dyn Object>>, right: Rc<RefCell<dyn Object>>, spacing: f32) -> Self {
    Self::new(left, right, BooleanOperator::DIFFERENCE, spacing)
  }

  pub fn new(
    left: Rc<RefCell<dyn Object>>,
    right: Rc<RefCell<dyn Object>>,
    operator: BooleanOperator,
    spacing: f32,
  ) -> Self {
    let left_object = left.clone();
    let left_object = left_object.as_ref().borrow();
    let left_object = left_object.as_octree_object().expect("Left object is not an octree object");

    let right_object = right.clone();
    let right_object = right_object.as_ref().borrow();
    let right_object = right_object.as_octree_object().expect("Right object is not an octree object");

    let name = format!("{} {} {}", left_object.get_name(), operator, right_object.get_name());
    let max_depth = left_object.get_max_depth().max(right_object.get_max_depth());

    let mut object = Self {
      left,
      right,
      operator,

      max_depth,
      spacing,
      volume: 0.0,
      root: None,
      instanced_cube: InstancedCube::new(name, None),
      transform: Transform::new(),
    };

    object.generate_octree();
    return object;
  }
}

#[allow(dead_code)]
impl OctreeObject for OctreeBoolean {
  impl_partial_OctreeObject!();

  #[allow(unconditional_recursion)]
  fn get_bounding_box(&self) -> AABB {
    let left_aabb = self.left.as_ref().borrow().as_octree_object().unwrap().get_bounding_box();
    let right_aabb = self.right.as_ref().borrow().as_octree_object().unwrap().get_bounding_box();

    let max_element = left_aabb.max.max_element().abs()
      .max(left_aabb.min.min_element().abs())
      .max(right_aabb.max.max_element().abs())
      .max(right_aabb.min.min_element().abs());

    let min = Vec3::splat(-max_element);
    let max = Vec3::splat(max_element);

    AABB { min, max }.transform(&self.transform)
  }

  #[allow(unconditional_recursion)]
  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    let aabb = aabb.inverse_transform(&self.transform);
    let left_object = self.left.as_ref().borrow();
    let left_object = left_object.as_octree_object().unwrap();
    let right_object = self.right.as_ref().borrow();
    let right_object = right_object.as_octree_object().unwrap();

    let left_node_type = left_object.get_node_type(&aabb);
    let right_node_type = right_object.get_node_type(&aabb);

    match self.operator {
      BooleanOperator::UNION => {
        match (left_node_type, right_node_type) {
          (OctreeNodeType::IN, _) | (_, OctreeNodeType::IN) => OctreeNodeType::IN,
          (OctreeNodeType::OUT, OctreeNodeType::OUT) => OctreeNodeType::OUT,
          _ => OctreeNodeType::PARTIAL,
        }
      }

      BooleanOperator::INTERSECTION => {
        match (left_node_type, right_node_type) {
          (OctreeNodeType::IN, OctreeNodeType::IN) => OctreeNodeType::IN,
          (OctreeNodeType::OUT, _) | (_, OctreeNodeType::OUT) => OctreeNodeType::OUT,
          _ => OctreeNodeType::PARTIAL,
        }
      }

      BooleanOperator::DIFFERENCE => {
        match (left_node_type, right_node_type) {
          (OctreeNodeType::IN, OctreeNodeType::OUT) => OctreeNodeType::IN,
          (_, OctreeNodeType::IN) | (OctreeNodeType::OUT, _) => OctreeNodeType::OUT,
          _ => OctreeNodeType::PARTIAL,
        }
      }
    }
  }
}

octree_derive_Object!(OctreeBoolean);
