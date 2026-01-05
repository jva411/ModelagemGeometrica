use bytemuck::{Pod, Zeroable, bytes_of};
use glam::Mat4;

use crate::objects::{csg::csg_object::{CSGNode, CSGPrimitives}, mesh::{mesh_cone::MeshCone, mesh_cylinder::MeshCylinder, mesh_sphere::MeshSphere}};

#[derive(Debug, Clone, Copy)]
pub enum CSGGPUPrimitive {
  Sphere(CSGGPUSphere),
  Cylinder(GPUCylinder),
  Cone(GPUCone),
  Cube(GPUCube),
}

impl CSGGPUPrimitive {
  pub fn to_bytes(&self) -> &[u8] {
    match self {
      CSGGPUPrimitive::Sphere(sphere) => bytes_of(sphere),
      CSGGPUPrimitive::Cylinder(cylinder) => bytes_of(cylinder),
      CSGGPUPrimitive::Cone(cone) => bytes_of(cone),
      CSGGPUPrimitive::Cube(cube) => bytes_of(cube),
    }
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GPUCube {
  pub inverse_model: Mat4,
  pub material: Mat4,
  pub id_heap: u32,
  pub id_list: u32,
  pub _padding: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CSGGPUSphere {
  pub invserse_transform: Mat4,
  pub material: Mat4,
  pub radius: f32,
  pub id_heap: u32,
  pub id_list: u32,
  pub _padding: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GPUCylinder {
  pub inverse_model: Mat4,
  pub material: Mat4,
  pub radius: f32,
  pub height: f32,
  pub id_heap: u32,
  pub id_list: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GPUCone {
  pub inverse_model: Mat4,
  pub material: Mat4,
  pub radius: f32,
  pub height: f32,
  pub id_heap: u32,
  pub id_list: u32,
}

impl TryFrom<CSGNode> for CSGGPUPrimitive {
  type Error = ();

  fn try_from(node: CSGNode) -> Result<Self, Self::Error> {
    match node {
      CSGNode::Primitive{ primitive, object} => {
        let transform = object.get_transform().clone();
        let material = object.get_material().clone();

        match primitive {
          CSGPrimitives::Cube => {
            return Ok(CSGGPUPrimitive::Cube(GPUCube {
              inverse_model: transform.build_model().inverse(),
              material: material.to_gpu_mat4(),
              id_heap: 0,
              id_list: u32::MAX,
              _padding: [0.0, 0.0],
            }));
          },

          CSGPrimitives::Sphere => {
            let sphere = object.as_any().downcast_ref::<MeshSphere>().unwrap();

            return Ok(CSGGPUPrimitive::Sphere(CSGGPUSphere {
              invserse_transform: transform.build_model().inverse(),
              material: material.to_gpu_mat4(),
              radius: sphere.radius,
              id_heap: 0,
              id_list: u32::MAX,
              _padding: 0.0,
            }));
          },

          CSGPrimitives::Cylinder => {
            let cylinder = object.as_any().downcast_ref::<MeshCylinder>().unwrap();

            return Ok(CSGGPUPrimitive::Cylinder(GPUCylinder {
              inverse_model: transform.build_model().inverse(),
              material: material.to_gpu_mat4(),
              radius: cylinder.radius,
              height: cylinder.height,
              id_heap: 0,
              id_list: u32::MAX,
            }));
          },

          CSGPrimitives::Cone => {
            let cone = object.as_any().downcast_ref::<MeshCone>().unwrap();

            return Ok(CSGGPUPrimitive::Cone(GPUCone {
              inverse_model: transform.build_model().inverse(),
              material: material.to_gpu_mat4(),
              radius: cone.radius,
              height: cone.height,
              id_heap: 0,
              id_list: u32::MAX,
            }));
          },

          _ => (),
        }
      },
      _ => (),
    };

    return Err(());
  }
}


#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GPUHeapNode {
  _type: u32,
  _id: u32,
}

impl GPUHeapNode {
  pub fn to_bytes(&self) -> &[u8] {
    bytes_of(self)
  }
}

impl CSGNode {
  pub fn get_boolean_heap(&self, heap: &mut Vec<GPUHeapNode>, index: u32) {
    match self {
      CSGNode::Primitive { primitive, object } => {
        heap[index as usize] = GPUHeapNode {
          _type: match primitive {
            CSGPrimitives::Cube => 2,
            CSGPrimitives::Sphere => 3,
            CSGPrimitives::Cylinder => 4,
            CSGPrimitives::Cone => 5,
            _ => panic!("Invalid primitive"),
          },
          _id: object.borrow().get_list_id(),
        };
      }
      CSGNode::Boolean { left, right, operator } => {
        heap[index as usize] = GPUHeapNode {
          _type: 1,
          _id: *operator as u32,
        };
        left.get_boolean_heap(heap, index * 2 + 1);
        right.get_boolean_heap(heap, index * 2 + 2);
      }
      CSGNode::Transform { node, .. } => node.get_boolean_heap(heap, index),
    }
  }
}
