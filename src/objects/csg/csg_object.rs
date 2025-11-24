use std::ops::Deref;

use uuid::Uuid;

use crate::{
  mesh_implement_partial_Object,
  objects::{
    mesh::{mesh_cube::MeshCube, mesh_object::MeshObject},
    object::Object,
    octree::{octree_boolean::BooleanOperator, octree_object::OctreeObject},
  },
  opengl::program::Program,
  utils::{material::Material, transform::Transform},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CSGPrimitives {
  Cube,
}

pub enum CSGNode {
  Boolean {
    left: Box<CSGNode>,
    right: Box<CSGNode>,
    operator: BooleanOperator,
  },
  Transform {
    node: Box<CSGNode>,
    transform: Transform,
  },
  Primitive {
    object: Box<dyn MeshObject>,
  },
}

impl CSGNode {
  pub fn draw(&self, program: &Program, base_transform: &Transform) {
    match self {
      CSGNode::Boolean { left, right, .. } => {
        left.draw(program, base_transform);
        right.draw(program, base_transform);
      }
      CSGNode::Transform { node, transform } => {
        node.draw(program, &base_transform.concat(transform));
      }
      CSGNode::Primitive { object } => {
        object.draw(program, Some(base_transform.clone()));
      }
    }
  }

  pub fn clone(&self) -> Self {
    match self {
      CSGNode::Boolean {
        left,
        right,
        operator,
      } => CSGNode::Boolean {
        left: Box::new(left.deref().clone()),
        right: Box::new(right.deref().clone()),
        operator: *operator,
      },
      CSGNode::Transform { node, transform } => CSGNode::Transform {
        node: Box::new(node.deref().clone()),
        transform: transform.clone(),
      },
      CSGNode::Primitive { object } => CSGNode::Primitive {
        object: object.clone_box(),
      },
    }
  }
}

pub struct CSGObject {
  pub id: Uuid,
  pub name: String,
  pub transform: Transform,
  pub material: Material,

  pub root: CSGNode,
}

impl CSGObject {
  pub fn new(primitive: CSGPrimitives, name: String) -> Self {
    let primitive_node = match primitive {
      CSGPrimitives::Cube => CSGNode::Primitive {
        object: Box::new(MeshCube::new(name.clone())),
      },
    };

    CSGObject {
      id: Uuid::new_v4(),
      name,
      transform: Transform::new(),
      material: Material::default(),
      root: primitive_node,
    }
  }

  pub fn boolean(&mut self, right: &CSGObject, operator: BooleanOperator) {
    let left_transform_node = CSGNode::Transform {
      node: Box::new(self.root.clone()),
      transform: self.transform.clone(),
    };

    let right_transform_node = CSGNode::Transform {
      node: Box::new(right.root.clone()),
      transform: right.transform.clone(),
    };

    let new_object_root = CSGNode::Boolean {
      left: Box::new(left_transform_node),
      right: Box::new(right_transform_node),
      operator,
    };

    self.root = new_object_root;
    self.name = format!("{} {} {}", self.name, operator, right.name);
    self.transform = Transform::new();
  }
}

impl Object for CSGObject {
  mesh_implement_partial_Object!();

  fn draw(&self, program: &Program, _base_transform: Option<Transform>) {
    self.root.draw(program, &self.transform);
  }
}
