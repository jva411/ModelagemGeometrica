use std::ops::Deref;

use glam::Mat4;
use uuid::Uuid;

use crate::{
  mesh_implement_partial_Object,
  objects::{
    mesh::{mesh_cone::MeshCone, mesh_cube::MeshCube, mesh_cylinder::MeshCylinder, mesh_object::MeshObject, mesh_sphere::MeshSphere},
    object::Object,
    octree::{octree_boolean::BooleanOperator, octree_object::OctreeObject},
  },
  opengl::program::Program,
  utils::{material::Material, transform::Transform},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CSGPrimitives {
  Cube,
  Sphere,
  Cylinder,
  Cone,
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
  pub fn draw(&self, program: &Program, parent_model: Mat4) {
    match self {
      CSGNode::Boolean { left, right, .. } => {
        left.draw(program, parent_model);
        right.draw(program, parent_model);
      }
      CSGNode::Transform { node, transform } => {
        let current_model = parent_model * transform.build_model();
        node.draw(program, current_model);
      }
      CSGNode::Primitive { object } => {
        object.csg_draw(program, parent_model);
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

#[allow(dead_code)]
pub struct CSGObject {
  pub id: Uuid,
  pub name: String,
  pub transform: Transform,
  pub material: Material,

  pub root: CSGNode,
}

impl CSGObject {
  fn new(name: String, root: CSGNode) -> Self {
    CSGObject {
      id: Uuid::new_v4(),
      name,
      transform: Transform::new(),
      material: Material::default(),
      root,
    }
  }

  pub fn new_cube(name: String) -> Self {
    let primitive_node = CSGNode::Primitive {
      object: Box::new(MeshCube::new(name.clone())),
    };

    CSGObject::new(name, primitive_node)
  }

  pub fn new_sphere(name: String, radius: f32, _subdivisions: u32) -> Self {
    let primitive_node = CSGNode::Primitive {
      object: Box::new(MeshSphere::new(name.clone(), radius, _subdivisions)),
    };

    CSGObject::new(name, primitive_node)
  }

  pub fn new_cylinder(name: String, radius: f32, height: f32, _subdivisions: u32) -> Self {
    let primitive_node = CSGNode::Primitive {
      object: Box::new(MeshCylinder::new(name.clone(), radius, height, _subdivisions)),
    };

    CSGObject::new(name, primitive_node)
  }

  pub fn new_cone(name: String, radius: f32, height: f32, _subdivisions: u32) -> Self {
    let primitive_node = CSGNode::Primitive {
      object: Box::new(MeshCone::new(name.clone(), radius, height, _subdivisions)),
    };

    CSGObject::new(name, primitive_node)
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
    let base_matrix = match _base_transform {
      Some(t) => t.build_model(),
      None => Mat4::IDENTITY,
    };

    let root_matrix = base_matrix * self.transform.build_model();

    self.root.draw(program, root_matrix);
  }
}
