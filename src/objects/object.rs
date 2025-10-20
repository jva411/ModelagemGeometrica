use uuid::Uuid;

use crate::{opengl::program::Program, utils::{material::Material, transform::Transform}};

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
}
