use glam::Mat4;

use crate::{objects::object::Object, opengl::program::Program};

pub trait MeshObject: Object {
  fn clone_box(&self) -> Box<dyn MeshObject>;
  fn csg_draw(&self, program: &Program, base_model: Mat4);
}

#[macro_export]
macro_rules! mesh_implement_partial_Object {
  () => {
    fn get_id(&self) -> Uuid { self.id }
    fn get_name(&self) -> String { self.name.clone() }
    fn get_name_mut(&mut self) -> &mut String { &mut self.name }

    fn get_transform(&self) -> &Transform { &self.transform }
    fn get_transform_mut(&mut self) -> &mut Transform { &mut self.transform }
    fn get_material(&self) -> &Material { &self.material }

    fn tick(&mut self) { }

    fn as_octree_object(&self) -> Option<&dyn OctreeObject> { None }
    fn as_octree_object_mut(&mut self) -> Option<&mut dyn OctreeObject> { None }

    fn as_any(&self) -> &dyn std::any::Any where Self: Sized { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any where Self: Sized { self }
  };
}
