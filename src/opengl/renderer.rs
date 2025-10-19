use std::{fs::File, rc::Rc};

use crate::opengl::{program::Program, shaders::Shaders};

#[allow(dead_code)]
pub enum ProgramType {
  Common,
  Instanced,
}

#[allow(dead_code)]
pub struct Renderer {
  pub programs: [Rc<Program>; 2],
  pub current_program: Rc<Program>,
}

#[allow(dead_code)]
impl Renderer {
  pub fn new() -> Option<Self> {
    let triangle_vertex_shader_file = File::open("assets/shaders/triangle/vertex.glsl").expect("Failed to open triangle vertex shader file");
    let triangle_fragment_shader_file = File::open("assets/shaders/triangle/fragment.glsl").expect("Failed to open triangle fragment shader file");
    let shaders = Shaders::from_files(&triangle_vertex_shader_file, &triangle_fragment_shader_file)?;
    let program = Program::new(shaders);

    let triangle_instanced_vertex_shader_file = File::open("assets/shaders/triangle/instanced/vertex.glsl").expect("Failed to open triangle instanced vertex shader file");
    let triangle_instanced_fragment_shader_file = File::open("assets/shaders/triangle/instanced/fragment.glsl").expect("Failed to open triangle instanced fragment shader file");
    let instanced_shaders = Shaders::from_files(&triangle_instanced_vertex_shader_file, &triangle_instanced_fragment_shader_file)?;
    let instanced_program = Program::new(instanced_shaders);

    let current_program = Rc::new(program);
    return Some(Renderer {
      programs: [current_program.clone(), Rc::new(instanced_program)],
      current_program,
    });
  }

  pub fn bind_program(&mut self, program_type: ProgramType) {
    let program_ref = &self.programs[program_type as usize];
    program_ref.bind();
    self.current_program = program_ref.clone();
  }

  pub fn bind_program_by_index(&mut self, index: usize) {
    let program_ref = &self.programs[index];
    program_ref.bind();
    self.current_program = program_ref.clone();
  }

  pub fn clear(&self, width: u32, height: u32) {
    unsafe {
      // egui canvas setup
      gl::Viewport(0, 0, width as i32, height as i32);
      gl::Disable(gl::SCISSOR_TEST);

      gl::ClearColor(0.53, 0.81, 0.92, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

      gl::Enable(gl::DEPTH_TEST);
      gl::Enable(gl::CULL_FACE);
    }
  }

  // pub fn enable_highlight(&self) {
  //   unsafe {
  //     gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
  //     gl::StencilMask(0x00);
  //     gl::Disable(gl::DEPTH_TEST);
  //   }
  // }

  // pub fn disable_highlight(&self) {
  //   unsafe {
  //     gl::StencilMask(0xFF);
  //     gl::StencilFunc(gl::ALWAYS, 0, 0xFF);
  //     gl::Enable(gl::DEPTH_TEST);
  //   }
  // }
}
