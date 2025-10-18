use crate::{opengl::renderer::Renderer, utils::{material::Material, transform::Transform}};


#[allow(dead_code)]
pub trait InstacedObject {
  fn get_transform(&self) -> &Vec<Transform>;
  fn get_transform_mut(&mut self) -> &mut Vec<Transform>;
  fn get_material(&mut self) -> &mut Material;

  fn add_instance(&mut self, transform: Transform);

  fn tick(&mut self);
  fn draw(&self, renderer: &Renderer);
}
