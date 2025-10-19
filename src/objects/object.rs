use crate::{opengl::program::Program, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub trait Object {
  fn get_transform(&self) -> &Transform;
  fn get_transform_mut(&mut self) -> &mut Transform;
  fn get_material(&self) -> &Material;

  fn tick(&mut self);
  fn draw(&self, program: &Program);
}
