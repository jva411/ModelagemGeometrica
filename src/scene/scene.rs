use std::{cell::RefCell, rc::Rc};

use crate::{lights::light::Light, objects::{instanced::instanced_object::InstacedObject, object::Object}, opengl::renderer::Renderer, utils::camera::Camera};

pub struct Scene {
  pub camera: Camera,
  pub objects: Vec<Box<dyn Object>>,
  pub instanced_objects: Vec<Box<dyn InstacedObject>>,
  pub lights: Vec<Box<dyn Light>>,
  pub renderer: Rc<RefCell<Renderer>>,
}

impl Scene {
  pub fn new(camera: Camera, renderer: Rc<RefCell<Renderer>>) -> Self {
    return Scene {
      camera,
      objects: Vec::new(),
      instanced_objects: Vec::new(),
      lights: Vec::new(),
      renderer,
    };
  }

  pub fn tick(&mut self) {
    for object in &mut self.objects {
      object.tick();
    }
  }

  pub fn draw(&self) {
    let renderer = self.renderer.borrow();
    renderer.bind_program(&renderer.current_program);

    self.camera.send_to_program(&renderer.current_program);
    renderer.current_program.set_uniform1i("n_lights", self.lights.len() as i32).expect("Failed to set number of lights");

    for (i, light) in self.lights.iter().enumerate() {
      light.send_to_program(&renderer.current_program, i);
    }

    for object in &self.objects {
      object.draw(&self.renderer.borrow());
    }

    renderer.bind_program(&renderer.instanced_program);
    self.camera.send_to_program(&renderer.instanced_program);
    renderer.instanced_program.set_uniform1i("n_lights", self.lights.len() as i32).expect("Failed to set number of lights");

    for (i, light) in self.lights.iter().enumerate() {
      light.send_to_program(&renderer.instanced_program, i);
    }

    for object in &self.instanced_objects {
      object.draw(&self.renderer.borrow());
    }
  }

  pub fn add_object(&mut self, object: Box<dyn Object>) { self.objects.push(object); }
  pub fn add_instanced_object(&mut self, object: Box<dyn InstacedObject>) { self.instanced_objects.push(object); }
  pub fn add_light(&mut self, light: Box<dyn Light>) { self.lights.push(light); }
}
