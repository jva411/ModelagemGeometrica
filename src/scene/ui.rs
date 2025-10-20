use egui::{FullOutput, SidePanel, Ui};
use uuid::Uuid;

use crate::scene::{scene::Scene, window::Window};

#[derive(PartialEq, Eq)]
pub enum UITab {
  Objects,
  Properties,
}

#[allow(dead_code)]
pub struct UIManager {
  pub selected_tab: UITab,
  pub selected_object_id: Option<Uuid>,
}

impl UIManager {
  pub fn new() -> Self {
    UIManager {
      selected_tab: UITab::Objects,
      selected_object_id: None,
    }
  }
}

impl Window {
  pub fn draw_ui(&mut self) {
    unsafe {
      gl::Disable(gl::DEPTH_TEST);
      gl::Disable(gl::CULL_FACE);
    }

    self.egui.state.input.time = Some(self.elapsed_time as f64);
    self.egui.context.begin_pass(self.egui.state.input.take());

    let ui_manager = &mut self.ui_manager;
    let scene = &mut self.scene;

    SidePanel::right("painel_controles")
      .resizable(false)
      .exact_width((self.width - self.canvas_width) as f32)
      .show(&self.egui.context, |ui| {
        Window::draw_tabs(ui, ui_manager, scene);
      });

    let FullOutput { platform_output, textures_delta, shapes, pixels_per_point, .. } = self.egui.context.end_pass();
    self.egui.state.process_output(&self.sdl.window, &platform_output);
    let paint_jobs = self.egui.context.tessellate(shapes, pixels_per_point);
    self.egui.painter.paint_jobs(None, textures_delta, paint_jobs);
  }

  fn draw_tabs(ui: &mut Ui, ui_manager: &mut UIManager, scene: &mut Scene) {
    ui.horizontal(|ui| {
      ui.selectable_value(&mut ui_manager.selected_tab, UITab::Objects, "Objects");
      ui.separator();
      ui.selectable_value(&mut ui_manager.selected_tab, UITab::Properties, "Properties");
    });
    ui.separator();

    match ui_manager.selected_tab {
      UITab::Objects => Window::draw_objects_tab(ui, ui_manager, scene),
      UITab::Properties => Window::draw_properties_tab(ui, ui_manager, scene),
    }
  }

  fn draw_objects_tab(ui: &mut Ui, ui_manager: &mut UIManager, scene: &mut Scene) {
    for (id, object) in &mut scene.objects_by_id.iter() {
      let is_selected = ui_manager.selected_object_id == Some(*id);
      if ui.selectable_label(is_selected, object.borrow().get_name()).clicked() {
        ui_manager.selected_object_id = Some(*id);
      }
    }
  }

  fn draw_properties_tab(ui: &mut Ui, ui_manager: &mut UIManager, scene: &mut Scene) {
    if let None = ui_manager.selected_object_id {
      ui.label("No object selected");
      return;
    }

    let mut object = scene.objects_by_id.get_mut(&ui_manager.selected_object_id.unwrap()).unwrap().borrow_mut();
    ui.heading("Name");
    ui.add(egui::TextEdit::singleline(object.get_name_mut()));

    let transform = object.get_transform_mut();
    ui.heading("Translation");
    ui.horizontal(|ui| {
      ui.label("X: ");
      ui.add(egui::DragValue::new(&mut transform.translation.x).speed(0.1));

      ui.label("Y: ");
      ui.add(egui::DragValue::new(&mut transform.translation.y).speed(0.1));

      ui.label("Z: ");
      ui.add(egui::DragValue::new(&mut transform.translation.z).speed(0.1));
    });

    ui.heading("Rotation");
    ui.horizontal(|ui| {
      ui.label("Yaw: ");
      ui.add(egui::DragValue::new(&mut transform.rotation.yaw).speed(0.1));

      ui.label("Pitch: ");
      ui.add(egui::DragValue::new(&mut transform.rotation.pitch).speed(0.1));

      ui.label("Roll: ");
      ui.add(egui::DragValue::new(&mut transform.rotation.roll).speed(0.1));
    });

    ui.heading("Scale");
    ui.horizontal(|ui| {
      ui.label("X: ");
      ui.add(egui::DragValue::new(&mut transform.scale.x).speed(0.1));

      ui.label("Y: ");
      ui.add(egui::DragValue::new(&mut transform.scale.y).speed(0.1));

      ui.label("Z: ");
      ui.add(egui::DragValue::new(&mut transform.scale.z).speed(0.1));
    });
  }
}
