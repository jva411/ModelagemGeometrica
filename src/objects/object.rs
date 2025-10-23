use std::any::Any;

use uuid::Uuid;

use crate::{objects::octree::octree_object::OctreeObject, opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub trait Object {
  fn get_id(&self) -> Uuid;
  fn get_name(&self) -> String;
  fn get_name_mut(&mut self) -> &mut String;

  fn get_transform(&self) -> &Transform;
  fn get_transform_mut(&mut self) -> &mut Transform;
  fn get_material(&self) -> &Material;

  fn tick(&mut self);
  fn draw(&self, program: &Program);

  fn as_octree_object(&self) -> Option<&dyn OctreeObject> { None }
  fn as_octree_object_mut(&mut self) -> Option<&mut dyn OctreeObject> { None }

  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}
