use std::{cell::RefCell, fs::File, rc::Rc};

use egui::{vec2, ComboBox, FullOutput, SidePanel, Ui, Window as EguiWindow};
use rfd::FileDialog;
use uuid::Uuid;

use crate::{objects::{csg::csg_object::CSGObject, object::Object, octree::octree_boolean::BooleanOperator}, opengl::renderer::ProgramType, scene::{scene::Scene, ui::{csg_ui::NewCSGObjectProperties, octree_ui::NewOctreeObjectProperties, winged_edged_ui::NewWingedEdgeObjectProperties}, window::Window}};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum UITab {
  Objects,
  Properties,
}

#[derive(Clone, Debug)]
pub enum UICommand {
  CreateObject(NewObjectProperties),
  DeleteObject(Uuid),
  CopyObject(Uuid),
  ApplyBoolean {
    object_type: ObjectType,
    left_id: Uuid,
    right_id: Uuid,
    operator: BooleanOperator,
  },
  SaveObject(Uuid),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ObjectType {
  Octree,
  CSG,
  WingedEdge,
}

#[derive(Clone, Debug)]
pub enum NewObjectProperties {
  Octree(NewOctreeObjectProperties),
  CSG(NewCSGObjectProperties),
  WingedEdge(NewWingedEdgeObjectProperties),
}

impl Default for NewObjectProperties {
  fn default() -> Self {
    NewObjectProperties::WingedEdge(NewWingedEdgeObjectProperties::default())
  }
}

#[allow(dead_code)]
pub struct UIManager {
  pub selected_tab: UITab,
  pub selected_object_id: Option<Uuid>,

  pub is_add_object_window_open: bool,
  pub new_object_type: ObjectType,
  pub new_object_properties: NewObjectProperties,
  pub previous_new_object_type: ObjectType,

  pub boolean_operator: BooleanOperator,
  pub selected_boolean_object_id: Option<Uuid>,

  pub commands_queue: Vec<UICommand>,
}

impl UIManager {
  pub fn new() -> Self {
    UIManager {
      selected_tab: UITab::Objects,
      selected_object_id: None,
      is_add_object_window_open: false,
      new_object_type: ObjectType::WingedEdge,
      previous_new_object_type: ObjectType::WingedEdge,
      new_object_properties: NewObjectProperties::default(),
      boolean_operator: BooleanOperator::UNION,
      selected_boolean_object_id: None,
      commands_queue: Vec::new(),
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

    SidePanel::right("main_side_panel")
      .resizable(false)
      .exact_width((self.width - self.canvas_width) as f32)
      .show(&self.egui.context, |ui| {
          Window::draw_tabs(ui, ui_manager, scene);
      });

    Window::draw_add_object_window(&self.egui.context, ui_manager);

    let FullOutput {
      platform_output,
      textures_delta,
      shapes,
      pixels_per_point,
      ..
    } = self.egui.context.end_pass();

    self.egui
      .state
      .process_output(&self.sdl.window, &platform_output);

    let paint_jobs = self.egui.context.tessellate(shapes, pixels_per_point);
    self.egui.painter.paint_jobs(None, textures_delta, paint_jobs);
  }

  fn draw_tabs(ui: &mut Ui, ui_manager: &mut UIManager, scene: &mut Scene) {
    ui.horizontal(|ui| {
      ui.selectable_value(&mut ui_manager.selected_tab, UITab::Objects, "Objects");
      ui.selectable_value(
        &mut ui_manager.selected_tab,
        UITab::Properties,
        "Properties",
      );
    });
    ui.separator();

    match ui_manager.selected_tab {
      UITab::Objects => Window::draw_objects_tab(ui, ui_manager, scene),
      UITab::Properties => Window::draw_properties_tab(ui, ui_manager, scene),
    }
  }

  fn draw_objects_tab(ui: &mut Ui, ui_manager: &mut UIManager, scene: &mut Scene) {
    if ui.button("Add Object").clicked() {
      ui_manager.is_add_object_window_open = true;
      ui_manager.new_object_type = ObjectType::WingedEdge;
      ui_manager.new_object_properties = NewObjectProperties::default();
    }
    ui.separator();

    let mut sorted_objects = scene.objects_by_id
      .iter()
      .collect::<Vec<_>>();

    sorted_objects.sort_by_key(|(id, _)| scene.objects_by_id.get(id).unwrap().borrow().get_name());

    for (id, object) in sorted_objects {
      let is_selected = ui_manager.selected_object_id == Some(*id);
      if ui.selectable_label(is_selected, object.borrow().get_name()).clicked() {
        ui_manager.selected_object_id = Some(*id);
      }
    }
  }

  fn draw_properties_tab(ui: &mut Ui, ui_manager: &mut UIManager, scene: &mut Scene) {
    let Some(selected_id) = ui_manager.selected_object_id else {
      ui.label("No object selected");
      return;
    };
    let Some(object_rc) = scene.objects_by_id.get(&selected_id).cloned() else {
      ui_manager.selected_object_id = None;
      ui.label("No object selected");
      return;
    };

    let mut object = object_rc.borrow_mut();

    ui.heading("Name");
    ui.add(egui::TextEdit::singleline(object.get_name_mut()));
    ui.separator();
    ui.horizontal(|ui| {
      if ui.button("Save Object").clicked() {
        ui_manager.commands_queue.push(UICommand::SaveObject(selected_id));
      }

      let delete_button = egui::Button::new("Delete Object").fill(egui::Color32::from_rgb(180, 40, 40));
      if ui.add(delete_button).clicked() {
        ui_manager.commands_queue.push(UICommand::DeleteObject(selected_id));
      }

      if let Some(..) = object.as_any().downcast_ref::<CSGObject>() {
        if ui.button("Copy Object").clicked() {
          ui_manager.commands_queue.push(UICommand::CopyObject(selected_id));
        }
      }
    });
    ui.separator();

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
      ui.add(egui::DragValue::new(&mut transform.rotation.yaw).speed(0.5));
      ui.label("Pitch: ");
      ui.add(egui::DragValue::new(&mut transform.rotation.pitch).speed(0.5));
      ui.label("Roll: ");
      ui.add(egui::DragValue::new(&mut transform.rotation.roll).speed(0.5));
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
    ui.separator();

    if let Some(object) = object.as_octree_object_mut() {
      ui_manager.draw_octree_object_properties(ui, scene, object);
    } else if object.as_any_mut().is::<CSGObject>() {
      ui_manager.draw_csg_object_properties(ui, scene, object.as_any_mut().downcast_mut::<CSGObject>().unwrap());
    }
  }

  fn draw_add_object_window(ctx: &egui::Context, ui_manager: &mut UIManager) {
    if !ui_manager.is_add_object_window_open {
      return;
    }

    let mut is_still_open = ui_manager.is_add_object_window_open;
    EguiWindow::new("Add new object")
      .open(&mut is_still_open)
      .collapsible(false)
      .resizable(false)
      .default_size(vec2(300.0, 350.0))
      .show(ctx, |ui| {
        ui.heading("Type");
        ComboBox::from_label("Select the type")
          .selected_text(format!("{:?}", ui_manager.new_object_type))
          .show_ui(ui, |ui| {
            ui.selectable_value(
              &mut ui_manager.new_object_type,
              ObjectType::Octree,
              "Octree",
            );
            ui.selectable_value(
              &mut ui_manager.new_object_type,
              ObjectType::CSG,
              "CSG",
            );
            ui.selectable_value(
              &mut ui_manager.new_object_type,
              ObjectType::WingedEdge,
              "Winged Edge",
            );
          });

        ui.separator();

        if ui_manager.new_object_type != ui_manager.previous_new_object_type {
          ui_manager.new_object_properties = match ui_manager.new_object_type {
            ObjectType::Octree => NewObjectProperties::Octree(NewOctreeObjectProperties::default()),
            ObjectType::CSG => NewObjectProperties::CSG(NewCSGObjectProperties::default()),
            ObjectType::WingedEdge => NewObjectProperties::WingedEdge(NewWingedEdgeObjectProperties::default()),
          };
          ui_manager.previous_new_object_type = ui_manager.new_object_type;
        }

        match ui_manager.new_object_type {
          ObjectType::Octree => {
            ui_manager.draw_octree_creation_options(ui);
          },
          ObjectType::CSG => {
            ui_manager.draw_csg_creation_options(ui);
          },
          ObjectType::WingedEdge => {
            ui_manager.draw_winged_edge_creation_options(ui);
          },
        }
      });

    if ui_manager.is_add_object_window_open {
      ui_manager.is_add_object_window_open = is_still_open;
    }
  }

  pub fn process_ui_commands(&mut self) {
    let commands: Vec<UICommand> = self.ui_manager.commands_queue.drain(..).collect();

    for command in commands {
      match command {
        UICommand::CreateObject(props) => {
          match props {
            NewObjectProperties::Octree(props) => {
              let new_object = Window::create_octree_object(props);
              self.scene.add_object(ProgramType::Instanced, new_object);
            },
            NewObjectProperties::CSG(props) => {
              let new_object = Window::create_csg_object(props);
              self.scene.add_object(ProgramType::Common, new_object);
            },
            NewObjectProperties::WingedEdge(props) => {
              let new_object = Window::create_winged_edge_object(props);
              self.scene.add_object(ProgramType::Common, new_object);
            },
          }
        }

        UICommand::DeleteObject(id) => {
          self.scene.remove_object(id);
        }

        UICommand::CopyObject(id) => {
          let object = self.scene.objects_by_id.get(&id);
          if let None = object {
            continue;
          }

          let object = object.unwrap().clone();
          let object = object.borrow();
          let csg_object = object.as_any().downcast_ref::<CSGObject>();
          if let None = csg_object {
            continue;
          }

          let csg_object = csg_object.unwrap();
          let mut new_csg_object = csg_object.clone();
          let new_id = new_csg_object.id;
          new_csg_object.name = format!("{} Copy", csg_object.get_name());
          let new_object_rc = Rc::new(RefCell::new(new_csg_object));
          self.scene.add_object(ProgramType::Common, new_object_rc.clone());
          self.ui_manager.selected_object_id = Some(new_id);
        }

        UICommand::ApplyBoolean { object_type, left_id, right_id, operator } => {
          match object_type {
            ObjectType::Octree => self.apply_octree_boolean(left_id, right_id, operator),
            ObjectType::CSG => self.apply_csg_boolean(left_id, right_id, operator),
            ObjectType::WingedEdge => self.apply_winged_edge_boolean(left_id, right_id, operator),
          }
        }

        UICommand::SaveObject(id) => {
          if let Some(object) = self.scene.objects_by_id.get(&id) {
            let object = object.borrow();
            if let Some(octree_object) = object.as_octree_object() {
              if let Some(path) = FileDialog::new().add_filter("oct", &["oct"]).set_file_name(format!("{}.oct", object.get_name())).save_file() {
                let mut file = File::create(path).unwrap();
                if let Some(root) = octree_object.get_root() {
                  root.serialize(&mut file);
                }
              }
            }
            else if let Some(csg_object) = object.as_any().downcast_ref::<CSGObject>() {
              if let Some(path) = FileDialog::new().add_filter("csg", &["csg"]).set_file_name(format!("{}.csg", object.get_name())).save_file() {
                let mut file = File::create(path).unwrap();
                csg_object.serialize(&mut file).unwrap();
              }
            }
          }
        }
      }
    }
  }
}
