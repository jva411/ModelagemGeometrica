use std::{fs::File};

use crate::opengl::{program::Program, shaders::Shaders};

pub struct Renderer {
  pub current_program: Program,
  pub instanced_program: Program,
}

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

    return Some(Renderer {
      current_program: program,
      instanced_program,
    });
  }

  pub fn bind_program(&self, program: &Program) {
    program.bind();
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
