use std::{io::{Read, Write}, ops::Deref};

use glam::Mat4;
use uuid::Uuid;

use crate::{
  mesh_implement_partial_Object,
  objects::{
    instanced::instanced_cube::SIZE_F32, mesh::{mesh_cone::MeshCone, mesh_cube::MeshCube, mesh_cylinder::MeshCylinder, mesh_object::MeshObject, mesh_sphere::MeshSphere}, object::Object, octree::{octree_boolean::BooleanOperator, octree_object::OctreeObject}
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
  Generic,
}

impl CSGPrimitives {
  fn serialize(&self, writer: &mut impl Write) -> std::io::Result<()> {
    match self {
      CSGPrimitives::Cube => writer.write_all(b"0")?,
      CSGPrimitives::Sphere => writer.write_all(b"1")?,
      CSGPrimitives::Cylinder => writer.write_all(b"2")?,
      CSGPrimitives::Cone => writer.write_all(b"3")?,
      _ => panic!("Unserializable primitive"),
    }

    Ok(())
  }

  fn deserialize(reader: &mut impl Read) -> std::io::Result<CSGPrimitives> {
    let mut buffer = [0; 1];
    reader.read_exact(&mut buffer)?;

    let primitive = match buffer[0] {
      b'0' => CSGPrimitives::Cube,
      b'1' => CSGPrimitives::Sphere,
      b'2' => CSGPrimitives::Cylinder,
      b'3' => CSGPrimitives::Cone,
      _ => panic!("Invalid primitive"),
    };
    Ok(primitive)
  }
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
    primitive: CSGPrimitives,
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
      CSGNode::Primitive { object, .. } => {
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
      CSGNode::Primitive { primitive, object } => CSGNode::Primitive {
        primitive: primitive.clone(),
        object: object.clone_box(),
      },
    }
  }

  pub fn serialize(&self, writer: &mut impl Write) -> std::io::Result<()> {
    match self {
      CSGNode::Boolean { left, right, operator } => {
        writer.write_all(b"0")?;
        operator.serialize(writer)?;
        left.serialize(writer)?;
        right.serialize(writer)?;
      }
      CSGNode::Transform { node, transform } => {
        writer.write_all(b"1")?;
        transform.serialize(writer)?;
        node.serialize(writer)?;
      }
      CSGNode::Primitive { primitive, object } => {
        writer.write_all(b"2")?;
        primitive.serialize(writer)?;
        match primitive {
          CSGPrimitives::Cube => {},
          CSGPrimitives::Sphere => {
            let sphere = object.as_any().downcast_ref::<MeshSphere>().unwrap();
            writer.write_all(&sphere.radius.to_be_bytes())?;
          },
          CSGPrimitives::Cylinder => {
            let cylinder = object.as_any().downcast_ref::<MeshCylinder>().unwrap();
            writer.write_all(&cylinder.radius.to_be_bytes())?;
            writer.write_all(&cylinder.height.to_be_bytes())?;
          },
          CSGPrimitives::Cone => {
            let cone = object.as_any().downcast_ref::<MeshCone>().unwrap();
            writer.write_all(&cone.radius.to_be_bytes())?;
            writer.write_all(&cone.height.to_be_bytes())?;
          },
          _ => panic!("Unserializable primitive"),
        }
      }
    }

    Ok(())
  }

  pub fn deserialize(reader: &mut impl Read) -> std::io::Result<CSGNode> {
    let mut buffer = [0; 1];
    reader.read_exact(&mut buffer)?;

    match buffer[0] {
      b'0' => {
        let operator = BooleanOperator::deserialize(reader)?;
        let left = CSGNode::deserialize(reader)?;
        let right = CSGNode::deserialize(reader)?;

        Ok(CSGNode::Boolean {
          left: Box::new(left),
          right: Box::new(right),
          operator,
        })
      }
      b'1' => {
        let transform = Transform::deserialize(reader)?;
        let node = CSGNode::deserialize(reader)?;

        Ok(CSGNode::Transform {
          node: Box::new(node),
          transform,
        })
      }
      b'2' => {
        let primitive = CSGPrimitives::deserialize(reader)?;
        let object: Box<dyn MeshObject> = match primitive {
          CSGPrimitives::Cube => Box::new(MeshCube::new(String::from("Cube"))),
          CSGPrimitives::Sphere => {
            let mut buffer = [0; SIZE_F32 as usize];
            reader.read_exact(&mut buffer)?;
            let radius = f32::from_be_bytes(buffer);
            Box::new(MeshSphere::new(String::from("Sphere"), radius, 50))
          },
          CSGPrimitives::Cylinder => {
            let mut buffer = [0; 2 * SIZE_F32 as usize];
            reader.read_exact(&mut buffer)?;
            let (radius, height) = (
              f32::from_be_bytes(buffer[0..SIZE_F32 as usize].try_into().unwrap()),
              f32::from_be_bytes(buffer[SIZE_F32 as usize..].try_into().unwrap()),
            );
            Box::new(MeshCylinder::new(String::from("Cylinder"), radius, height, 50))
          },
          CSGPrimitives::Cone => {
            let mut buffer = [0; 2 * SIZE_F32 as usize];
            reader.read_exact(&mut buffer)?;
            let (radius, height) = (
              f32::from_be_bytes(buffer[0..SIZE_F32 as usize].try_into().unwrap()),
              f32::from_be_bytes(buffer[SIZE_F32 as usize..].try_into().unwrap()),
            );
            Box::new(MeshCone::new(String::from("Cone"), radius, height, 50))
          },
          _ => panic!("Unserializable primitive"),
        };

        Ok(CSGNode::Primitive {
          primitive,
          object,
        })
      }
      _ => panic!("Invalid node"),
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
      primitive: CSGPrimitives::Cube,
      object: Box::new(MeshCube::new(name.clone())),
    };

    CSGObject::new(name, primitive_node)
  }

  pub fn new_sphere(name: String, radius: f32, _subdivisions: u32) -> Self {
    let primitive_node = CSGNode::Primitive {
      primitive: CSGPrimitives::Sphere,
      object: Box::new(MeshSphere::new(name.clone(), radius, _subdivisions)),
    };

    CSGObject::new(name, primitive_node)
  }

  pub fn new_cylinder(name: String, radius: f32, height: f32, _subdivisions: u32) -> Self {
    let primitive_node = CSGNode::Primitive {
      primitive: CSGPrimitives::Cylinder,
      object: Box::new(MeshCylinder::new(name.clone(), radius, height, _subdivisions)),
    };

    CSGObject::new(name, primitive_node)
  }

  pub fn new_cone(name: String, radius: f32, height: f32, _subdivisions: u32) -> Self {
    let primitive_node = CSGNode::Primitive {
      primitive: CSGPrimitives::Cone,
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

  pub fn serialize(&self, writer: &mut impl Write) -> std::io::Result<()> {
    let new_root = CSGNode::Transform {
      node: Box::new(self.root.clone()),
      transform: self.transform.clone(),
    };

    new_root.serialize(writer)
  }

  pub fn deserialize(name: String, reader: &mut impl Read) -> std::io::Result<CSGObject> {
    let root = CSGNode::deserialize(reader)?;

    match root {
      CSGNode::Transform { node, transform } => {
        let mut csg_object = CSGObject::new(name, *node);
        csg_object.transform = transform;
        Ok(csg_object)
      },
      _ => panic!("Invalid root node"),
    }
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
