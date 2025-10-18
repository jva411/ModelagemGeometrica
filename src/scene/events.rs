use std::collections::HashSet;

use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton};

use crate::scene::window::Window;

pub enum EventResult {
  None,
  Quit,
}

macro_rules! match_event_result {
  ($expr:expr) => {
    match $expr {
      EventResult::Quit => return EventResult::Quit,
      _ => {}
    }
  };
}

pub struct EventsManager {
  pub keys_pressed: HashSet<Keycode>,
  pub buttons_pressed: HashSet<MouseButton>,
  pub camera_speed: f32,
}

impl EventsManager {
  pub fn new() -> Self {
    return EventsManager {
      keys_pressed: HashSet::new(),
      buttons_pressed: HashSet::new(),
      camera_speed: 2.5,
    };
  }
}

impl Window {
  pub fn proccess_events(&mut self) -> EventResult {
    let events: Vec<Event> = self.sdl.event_pump.poll_iter().collect();
    for event in events {
      match event {
        Event::Quit { .. } => return EventResult::Quit,
        Event::KeyDown { keycode: Some(key), .. } => match_event_result!(self.on_key_down(key)),
        Event::KeyUp { keycode: Some(key), .. } => match_event_result!(self.on_key_up(key)),
        Event::MouseWheel { y, .. } => match_event_result!(self.on_mouse_wheel(y)),
        Event::MouseButtonDown { mouse_btn, .. } => match_event_result!(self.on_mouse_button_down(mouse_btn)),
        Event::MouseButtonUp { mouse_btn, .. } => match_event_result!(self.on_mouse_button_up(mouse_btn)),
        Event::MouseMotion { xrel, yrel, .. } => match_event_result!(self.on_mouse_motion(xrel, yrel)),
        _ => {}
      }
      self.egui.state.process_input(&self.sdl.window, event, &mut self.egui.painter);
    }

    self.check_camera_movement();

    return EventResult::None;
  }

  fn on_key_down(&mut self, key: Keycode) -> EventResult {
    self.events_manager.keys_pressed.insert(key);
    return EventResult::None;
  }

  fn on_key_up(&mut self, key: Keycode) -> EventResult {
    self.events_manager.keys_pressed.remove(&key);

    match key {
      Keycode::Escape => return EventResult::Quit,
      _ => {},
    };

    return EventResult::None;
  }

  fn on_mouse_wheel(&mut self, y: i32) -> EventResult {
    self.events_manager.camera_speed = (self.events_manager.camera_speed + y as f32 / 10.0).clamp(0.2, 30.0);
    return EventResult::None;
  }

  fn on_mouse_button_down(&mut self, mouse_btn: MouseButton) -> EventResult {
    self.events_manager.buttons_pressed.insert(mouse_btn);
    return EventResult::None;
  }

  fn on_mouse_button_up(&mut self, mouse_btn: MouseButton) -> EventResult {
    self.events_manager.buttons_pressed.remove(&mouse_btn);
    return EventResult::None;
  }

  fn on_mouse_motion(&mut self, xrel: i32, yrel: i32) -> EventResult {
    const camera_sensitivity: f32 = 0.5;

    if self.events_manager.buttons_pressed.contains(&MouseButton::Left) {
      self.scene.camera.transform.add_yaw((-xrel as f32 * camera_sensitivity).to_radians());
      self.scene.camera.transform.add_pitch((-yrel as f32 * camera_sensitivity).to_radians());
    }
    return EventResult::None;
  }

  fn check_camera_movement(&mut self) {
    let smoothness = self.events_manager.camera_speed * self.delta_time;

    let mut forward = 0.0;
    if self.events_manager.keys_pressed.contains(&Keycode::W) {
      forward += 1.0;
    }
    if self.events_manager.keys_pressed.contains(&Keycode::S) {
      forward += -1.0;
    }

    let mut right = 0.0;
    if self.events_manager.keys_pressed.contains(&Keycode::D) {
      right += 1.0;
    }
    if self.events_manager.keys_pressed.contains(&Keycode::A) {
      right += -1.0;
    }

    let mut up = 0.0;
    if self.events_manager.keys_pressed.contains(&Keycode::Space) {
      up += 1.0;
    }
    if self.events_manager.keys_pressed.contains(&Keycode::LShift) {
      up += -1.0;
    }

    self.scene.camera.transform.translate(forward * smoothness, right * smoothness, up * smoothness);
  }
}
