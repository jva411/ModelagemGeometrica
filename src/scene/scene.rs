use std::{cell::RefCell, rc::Rc};

use crate::{lights::light::Light, objects::object::Object, opengl::renderer::{ProgramType, Renderer}, utils::camera::Camera};

pub struct Scene {
  pub camera: Camera,
  pub objects: [Vec<Box<dyn Object>>; 2],
  pub lights: Vec<Box<dyn Light>>,
  pub renderer: Rc<RefCell<Renderer>>,
}

impl Scene {
  pub fn new(camera: Camera, renderer: Rc<RefCell<Renderer>>) -> Self {
    return Scene {
      camera,
      objects: [Vec::new(), Vec::new()],
      lights: Vec::new(),
      renderer,
    };
  }

  pub fn tick(&mut self) {
    for object_list in &mut self.objects {
      for object in object_list {
        object.tick();
      }
    }
  }

  // pub fn draw(&self) {
  //   let renderer = self.renderer.borrow();
  //   renderer.bind_program(&renderer.current_program);

  //   self.camera.send_to_program(&renderer.current_program);
  //   renderer.current_program.set_uniform1i("n_lights", self.lights.len() as i32).expect("Failed to set number of lights");

  //   for (i, light) in self.lights.iter().enumerate() {
  //     light.send_to_program(&renderer.current_program, i);
  //   }

  //   for object in &self.objects {
  //     object.draw(&self.renderer.borrow());
  //   }

  //   renderer.bind_program(&renderer.instanced_program);
  //   self.camera.send_to_program(&renderer.instanced_program);
  //   renderer.instanced_program.set_uniform1i("n_lights", self.lights.len() as i32).expect("Failed to set number of lights");

  //   for (i, light) in self.lights.iter().enumerate() {
  //     light.send_to_program(&renderer.instanced_program, i);
  //   }

  //   for object in &self.instanced_objects {
  //     object.draw(&self.renderer.borrow());
  //   }
  // }

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
        object.draw(program);
      }
    }
  }


  pub fn add_object(&mut self, program_type: ProgramType, object: Box<dyn Object>) { self.objects[program_type as usize].push(object); }
  pub fn add_light(&mut self, light: Box<dyn Light>) { self.lights.push(light); }
}
