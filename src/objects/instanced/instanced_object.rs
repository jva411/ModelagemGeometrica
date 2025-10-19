use crate::{objects::object::Object, utils::transform::Transform};


#[allow(dead_code)]
pub trait InstacedObject: Object {
  fn get_instances_transforms(&self) -> &Vec<Transform>;
  fn get_instances_transforms_mut(&mut self) -> &mut Vec<Transform>;
  fn get_instances_count(&self) -> usize;

  fn add_instance(&mut self, transform: Transform);

  fn setup_instances(&mut self);
}
