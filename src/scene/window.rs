use std::{cell::RefCell, rc::Rc, time::Instant};

use egui::{Checkbox, FullOutput, SidePanel};
use sdl2::video::{GLProfile, SwapInterval};
use egui_sdl2_gl::{self as egui_backend, DpiScaling, ShaderVersion};

use crate::{opengl::renderer::Renderer, scene::{events::{EventResult, EventsManager}, scene::Scene}, utils::camera::Camera};

#[allow(dead_code)]
pub struct SdlContext {
  pub window: sdl2::video::Window,
  pub context: sdl2::Sdl,
  pub event_pump: sdl2::EventPump,
  pub video_subsystem: sdl2::VideoSubsystem,
  pub gl_context: sdl2::video::GLContext,
}

#[allow(dead_code)]
pub struct EguiContext {
  pub context: egui::Context,
  pub painter: egui_sdl2_gl::painter::Painter,
  pub state: egui_sdl2_gl::EguiStateHandler,
}

#[allow(dead_code)]
pub struct Window {
  pub width: u32,
  pub height: u32,
  pub title: &'static str,

  pub canvas_width: u32,
  pub renderer: Rc<RefCell<Renderer>>,
  pub scene: Scene,

  pub delta_time: f32,
  pub sdl: SdlContext,
  pub egui: EguiContext,
  pub events_manager: EventsManager,
}

impl Window {
  pub fn new(title: &'static str, width: u32, height: u32, canvas_width: u32) -> Self {
    let (sdl, egui) = Window::load_gl(title, width, height);
    let renderer = Rc::new(RefCell::new(Renderer::new().unwrap()));
    let scene = Scene::new(Camera::new(), Rc::clone(&renderer));
    let events_manager = EventsManager::new();

    let window = Window {
      width,
      height,
      title,
      canvas_width,
      renderer,
      scene,

      delta_time: 0.0,
      sdl,
      egui,
      events_manager,
    };

    return window;
  }

  fn load_gl(title: &'static str, width: u32, height: u32) -> (SdlContext, EguiContext) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
      .window(title, width, height)
      .opengl()
      .build()
      .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
    unsafe {
      gl::Enable(gl::DEPTH_TEST);
      gl::Enable(gl::CULL_FACE);
    };

    let event_pump = sdl_context.event_pump().unwrap();

    let sdl = SdlContext {
      window,
      context: sdl_context,
      event_pump,
      video_subsystem,
      gl_context,
    };


    let (painter, egui_state) = egui_backend::with_sdl2(&sdl.window, ShaderVersion::Default, DpiScaling::Default);
    let egui_ctx = egui::Context::default();
    let egui = EguiContext {
      context: egui_ctx,
      painter,
      state: egui_state,
    };

    return (sdl, egui);
  }

  pub fn init(&mut self) {
    // UI interactive values
    let mut test_str: String = "A text box to write in. Cut, copy, paste commands are available.".to_owned();
    let mut enable_vsync = true;
    let mut quit = false;
    let mut slider = 0.0;

    let start_time = Instant::now();
    let mut last_time = start_time.clone();

    // Main loop
    'running: loop {
      let event_result = self.proccess_events();
      match event_result {
        EventResult::Quit => {
          break 'running;
        }
        EventResult::None => {}
      }

      // 1. Limpa a tela inteira com uma cor de fundo.
      // Egui irá desenhar sobre esta cor na sua área.
      unsafe {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
      }

      // Render scene
      self.renderer.borrow().clear(self.canvas_width, self.height);
      self.scene.tick();
      self.scene.draw();

      // 3. Prepara e constrói a UI do Egui
      unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::CULL_FACE);
      }
      self.egui.state.input.time = Some(start_time.elapsed().as_secs_f64());
      self.egui.context.begin_pass(self.egui.state.input.take());

      // Cria um painel na direita com uma largura fixa.
      SidePanel::right("painel_controles")
        .resizable(false)
        .exact_width((self.width - self.canvas_width) as f32)
        .show(&self.egui.context, |ui| {
          ui.heading("Controles");
          ui.separator();
          ui.text_edit_multiline(&mut test_str);
          ui.add(egui::Slider::new(&mut slider, 0.0..=50.0).text("Slider"));
          if ui.add(Checkbox::new(&mut enable_vsync, "VSync")).clicked() {
            let interval = if enable_vsync { SwapInterval::VSync } else { SwapInterval::Immediate };
            self.sdl.video_subsystem.gl_set_swap_interval(interval).unwrap();
          }
          ui.separator();
          if ui.button("Sair").clicked() {
            quit = true;
          }
        });

      let FullOutput { platform_output, textures_delta, shapes, pixels_per_point, .. } = self.egui.context.end_pass();
      self.egui.state.process_output(&self.sdl.window, &platform_output);

      // 4. Pinta a UI do Egui sobre o que já foi desenhado.
      // O painter do egui irá configurar seu próprio viewport e scissor test,
      // então não precisamos nos preocupar com isso.
      let paint_jobs = self.egui.context.tessellate(shapes, pixels_per_point);
      self.egui.painter.paint_jobs(None, textures_delta, paint_jobs);

      // 5. Troca o buffer da janela UMA VEZ no final do frame.
      self.sdl.window.gl_swap_window();

      if quit {
        break;
      }

      self.delta_time = last_time.elapsed().as_secs_f32();
      last_time = Instant::now();
    }
  }
}
