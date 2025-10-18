use crate::{opengl::renderer::Renderer, utils::{material::Material, transform::Transform}};

#[allow(dead_code)]
pub trait Object {
  fn get_transform(&self) -> &Transform;
  fn get_transform_mut(&mut self) -> &mut Transform;
  fn get_material(&mut self) -> &mut Material;

  fn tick(&mut self);
  fn draw(&self, renderer: &Renderer);
}
