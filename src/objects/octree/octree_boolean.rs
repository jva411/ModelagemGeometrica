use std::{cell::RefCell, fmt::Display, rc::Rc};

use uuid::Uuid;

use crate::{derive_Object, objects::{instanced::{instanced_cube::InstancedCube, instanced_object::InstacedObject}, object::Object, octree::octree_object::{OctreeNode, OctreeNodeType, OctreeObject, AABB}}, opengl::program::Program, utils::{material::Material, transform::Transform}};

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
      BooleanOperator::DIFFERENCE => write!(f, "Diference"),
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
  pub root: Option<OctreeNode>,
  pub instanced_cube: InstancedCube,
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
      root: None,
      instanced_cube: InstancedCube::new(name, None),
    };

    let root = OctreeNode::generate_octree(&object, max_depth);
    object.root = Some(root);
    object.generate_instanced_cube();

    return object;
  }
}

#[allow(dead_code)]
impl OctreeObject for OctreeBoolean {
  fn get_max_depth(&self) -> u32 { self.max_depth }
  fn get_root(&self) -> Option<&OctreeNode> { self.root.as_ref() }

  #[allow(unconditional_recursion)]
  fn get_bounding_box(&self) -> AABB {
    let left_aabb = self.left.as_ref().borrow().as_octree_object().unwrap().get_bounding_box();
    let right_aabb = self.right.as_ref().borrow().as_octree_object().unwrap().get_bounding_box();

    AABB { min: left_aabb.min.min(right_aabb.min), max: left_aabb.max.max(right_aabb.max) }
  }

  #[allow(unconditional_recursion)]
  fn get_node_type(&self, aabb: &AABB) -> OctreeNodeType {
    let left_object = self.left.as_ref().borrow();
    let left_object = left_object.as_octree_object().unwrap();
    let right_object = self.right.as_ref().borrow();
    let right_object = right_object.as_octree_object().unwrap();

    let left_node_type = left_object.get_node_type(aabb);
    let right_node_type = right_object.get_node_type(aabb);

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
          (_, OctreeNodeType::IN) | (OctreeNodeType::OUT, _) => OctreeNodeType::OUT,
          (OctreeNodeType::IN, OctreeNodeType::OUT) => OctreeNodeType::IN,
          _ => OctreeNodeType::PARTIAL,
        }
      }
    }
  }

  fn generate_instanced_cube(&mut self) {
    if let Some(root) = self.root.as_ref() {
      root.generate_transforms(
        self.spacing,
        self.instanced_cube.get_instances_transforms_mut(),
      );
    }

    self.instanced_cube.setup_instances();
  }
}


derive_Object!(OctreeBoolean);
