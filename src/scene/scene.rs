use std::{cell::RefCell, rc::Rc};

use egui::ahash::{HashMap, HashMapExt};
use glam::Vec3;
use uuid::Uuid;

use crate::{lights::light::Light, objects::{grid::{axes::Axes, grid::Grid}, object::Object}, opengl::renderer::{ProgramType, Renderer}, utils::camera::Camera};

#[allow(dead_code)]
pub struct Scene {
  pub camera: Camera,
  pub objects: [Vec<Rc<RefCell<dyn Object>>>; 2],
  pub objects_by_id: HashMap<Uuid, Rc<RefCell<dyn Object>>>,
  pub lights: Vec<Box<dyn Light>>,
  pub renderer: Rc<RefCell<Renderer>>,
  pub grid: Grid,
  pub axes: Axes,
}

#[allow(dead_code)]
impl Scene {
  pub fn new(camera: Camera, renderer: Rc<RefCell<Renderer>>) -> Self {
    return Scene {
      camera,
      objects: [Vec::new(), Vec::new()],
      objects_by_id: HashMap::new(),
      lights: Vec::new(),
      renderer,
      grid: Grid::new(20, 0.5, Vec3::splat(0.5)),
      axes: Axes::new(),
    };
  }

  pub fn tick(&mut self) {
    for object_list in &mut self.objects {
      for object in object_list {
        object.borrow_mut().tick();
      }
    }
  }

  pub fn draw(&self) {
    let mut renderer = self.renderer.borrow_mut();
    for i in 0..2 {
      renderer.bind_program_by_index(i);
      let program = &renderer.current_program;

      self.camera.send_to_program(&program);
      program.set_uniform1i("n_lights", self.lights.len() as i32).expect("Failed to set number of lights");

      for (i, light) in self.lights.iter().enumerate() {
        light.send_to_program(&program, i);
      }

      for object in &self.objects[i] {
        object.borrow().draw(program);
      }
    }

    renderer.bind_program(ProgramType::Grid);
    let program = &renderer.current_program;
    self.camera.send_to_program(&program);
    self.grid.draw(program);

    unsafe { gl::Disable(gl::DEPTH_TEST); }
    self.axes.draw(program);
    unsafe { gl::Enable(gl::DEPTH_TEST); }
  }


  pub fn add_object(&mut self, program_type: ProgramType, object: Rc<RefCell<dyn Object>>) {
    let id = object.borrow().get_id();
    self.objects[program_type as usize].push(object.clone());
    self.objects_by_id.insert(id, object);
  }

  pub fn remove_object(&mut self, id: Uuid) {
    if self.objects_by_id.remove(&id).is_none() {
      return
    };

    for objects in &mut self.objects {
      if let Some(index) = objects.iter().position(|object| object.borrow().get_id() == id) {
        objects.remove(index);
        break;
      }
    }
  }

  pub fn add_light(&mut self, light: Box<dyn Light>) { self.lights.push(light); }
}
