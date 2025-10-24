use std::{cell::RefCell, fs::File, path::PathBuf, rc::Rc};

use egui::{vec2, ComboBox, FullOutput, SidePanel, Ui, Window as EguiWindow};
use glam::Vec3;
use rfd::FileDialog;
use uuid::Uuid;

use crate::{objects::{object::Object, octree::{octree_boolean::{BooleanOperator, OctreeBoolean}, octree_cone::OctreeCone, octree_cube::OctreeCube, octree_cylinder::OctreeCylinder, octree_generic::OctreeGeneric, octree_mesh::OctreeMesh, octree_sphere::OctreeSphere}}, opengl::renderer::ProgramType, scene::{scene::Scene, window::Window}};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum UITab {
  Objects,
  Properties,
}

#[derive(Clone, Debug)]
pub enum UICommand {
  CreateOctree(NewOctreeObjectProperties),
  DeleteObject(Uuid),
  ApplyBoolean {
    left_id: Uuid,
    right_id: Uuid,
    operator: BooleanOperator,
  },
  SaveOctree(Uuid),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ObjectType {
  Octree,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum OctreePrimitive {
  Cube,
  Sphere,
  Cylinder,
  Cone,
  Mesh,
  Generic,
}

#[derive(Clone, Debug)]
pub struct NewOctreeObjectProperties {
  primitive: OctreePrimitive,
  name: String,

  radius: f32,
  height: f32,
  size: Vec3,
  obj_path: Option<PathBuf>,

  max_depth: u32,
  spacing: f32,
}

impl Default for NewOctreeObjectProperties {
  fn default() -> Self {
    Self {
      primitive: OctreePrimitive::Sphere,
      name: "New Object".to_string(),
      radius: 1.0,
      height: 2.0,
      size: Vec3::ONE,
      obj_path: None,
      max_depth: 5,
      spacing: 0.0,
    }
  }
}

#[allow(dead_code)]
pub struct UIManager {
  pub selected_tab: UITab,
  pub selected_object_id: Option<Uuid>,

  pub is_add_object_window_open: bool,
  pub new_object_type: ObjectType,
  pub new_object_properties: NewOctreeObjectProperties,

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
      new_object_type: ObjectType::Octree,
      new_object_properties: NewOctreeObjectProperties::default(),
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
    }
    ui.separator();

    for (id, object) in scene.objects_by_id.iter() {
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
      return;
    };

    let is_generic_octree = object_rc.borrow().as_any().is::<OctreeGeneric>();
    {
      let mut object = object_rc.borrow_mut();

      ui.heading("Name");
      ui.add(egui::TextEdit::singleline(object.get_name_mut()));
      ui.separator();
      ui.horizontal(|ui| {
        if ui.button("Save Object").clicked() {
          ui_manager.commands_queue.push(UICommand::SaveOctree(selected_id));
        }

        let delete_button = egui::Button::new("Delete Object").fill(egui::Color32::from_rgb(180, 40, 40));
        if ui.add(delete_button).clicked() {
          ui_manager.commands_queue.push(UICommand::DeleteObject(selected_id));
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

      if !is_generic_octree {
        let any_mut = object.as_any_mut();

        if let Some(cube) = any_mut.downcast_mut::<OctreeCube>() {
          ui.heading("Octree Cube Properties");
          ui.horizontal(|ui| {
            ui.label("Size X: ");
            ui.add(egui::DragValue::new(&mut cube.size.x).speed(0.1));
            ui.label("Size Y: ");
            ui.add(egui::DragValue::new(&mut cube.size.y).speed(0.1));
            ui.label("Size Z: ");
            ui.add(egui::DragValue::new(&mut cube.size.z).speed(0.1));
          });
          ui.add(egui::Slider::new(&mut cube.max_depth, 1..=10).text("Max Depth"));
          ui.add(egui::Slider::new(&mut cube.spacing, 0.0..=1.0).text("Spacing"));
        } else if let Some(sphere) = any_mut.downcast_mut::<OctreeSphere>() {
          ui.heading("Octree Sphere Properties");
          ui.add(egui::Slider::new(&mut sphere.radius, 0.1..=10.0).text("Radius"));
          ui.add(egui::Slider::new(&mut sphere.max_depth, 1..=10).text("Max Depth"));
          ui.add(egui::Slider::new(&mut sphere.spacing, 0.0..=1.0).text("Spacing"));
        } else if let Some(cylinder) = any_mut.downcast_mut::<OctreeCylinder>() {
          ui.heading("Octree Cylinder Properties");
          ui.add(egui::Slider::new(&mut cylinder.radius, 0.1..=10.0).text("Radius"));
          ui.add(egui::Slider::new(&mut cylinder.height, 0.1..=10.0).text("Height"));
          ui.add(egui::Slider::new(&mut cylinder.max_depth, 1..=10).text("Max Depth"));
          ui.add(egui::Slider::new(&mut cylinder.spacing, 0.0..=1.0).text("Spacing"));
        } else if let Some(cone) = any_mut.downcast_mut::<OctreeCone>() {
          ui.heading("Octree Cone Properties");
          ui.add(egui::Slider::new(&mut cone.radius, 0.1..=10.0).text("Radius"));
          ui.add(egui::Slider::new(&mut cone.height, 0.1..=10.0).text("Height"));
          ui.add(egui::Slider::new(&mut cone.max_depth, 1..=10).text("Max Depth"));
          ui.add(egui::Slider::new(&mut cone.spacing, 0.0..=1.0).text("Spacing"));
        } else if let Some(boolean) = any_mut.downcast_mut::<OctreeBoolean>() {
          ui.heading("Octree Boolean Properties");
          ui.add(egui::Slider::new(&mut boolean.max_depth, 1..=10).text("Max Depth"));
          ui.add(egui::Slider::new(&mut boolean.spacing, 0.0..=1.0).text("Spacing"));
        } else if let Some(mesh) = any_mut.downcast_mut::<OctreeMesh>() {
          ui.heading("Octree Mesh Properties");
          ui.add(egui::Slider::new(&mut mesh.max_depth, 1..=10).text("Max Depth"));
          ui.add(egui::Slider::new(&mut mesh.spacing, 0.0..=1.0).text("Spacing"));
        }

        if ui.button("Rebuild Octree").clicked() {
          if let Some(object) = object.as_octree_object_mut() {
            object.generate_octree();
          }
        }
      }
    }

    if !is_generic_octree {
      let object = object_rc.borrow();
      if object.as_octree_object().is_some() {
        ui.separator();
        ui.heading("Boolean Operation");

        ComboBox::from_label("Operation")
          .selected_text(format!("{}", ui_manager.boolean_operator))
          .show_ui(ui, |ui| {
            ui.selectable_value(&mut ui_manager.boolean_operator, BooleanOperator::UNION, "Union");
            ui.selectable_value(&mut ui_manager.boolean_operator, BooleanOperator::INTERSECTION, "Intersection");
            ui.selectable_value(&mut ui_manager.boolean_operator, BooleanOperator::DIFFERENCE, "Difference");
          });

        let mut selected_name = "Select Object".to_string();
        if let Some(selected_id) = ui_manager.selected_boolean_object_id {
          if let Some(object) = scene.objects_by_id.get(&selected_id) {
            selected_name = object.borrow().get_name();
          }
        }

        ComboBox::from_label("Object")
          .selected_text(selected_name)
          .show_ui(ui, |ui| {
            for (id, object) in scene.objects_by_id.iter() {
              if Some(*id) != ui_manager.selected_object_id
                && object.borrow().as_octree_object().is_some()
                && !object.borrow().as_any().is::<OctreeGeneric>()
              {
                ui.selectable_value(&mut ui_manager.selected_boolean_object_id, Some(*id), object.borrow().get_name());
              }
            }
          });

        if ui.button("Apply").clicked() {
          if let Some(right_id) = ui_manager.selected_boolean_object_id {
            ui_manager.commands_queue.push(UICommand::ApplyBoolean {
              left_id: selected_id,
              right_id,
              operator: ui_manager.boolean_operator,
            })
          }
        }
      }
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
          });

        ui.separator();

        match ui_manager.new_object_type {
          ObjectType::Octree => {
            Window::draw_octree_options(ui, ui_manager);
          }
        }

        ui.separator();
        ui.horizontal(|ui| {
          let should_enable_creation = (
            ui_manager.new_object_properties.primitive != OctreePrimitive::Mesh
            && ui_manager.new_object_properties.primitive != OctreePrimitive::Generic
          ) || ui_manager.new_object_properties.obj_path.is_some();

            if ui.button("Create").clicked() {
            if should_enable_creation {
              ui_manager.commands_queue.push(UICommand::CreateOctree(ui_manager.new_object_properties.clone()));
              ui_manager.is_add_object_window_open = false;
              ui_manager.new_object_properties = NewOctreeObjectProperties::default();
            }
          }
          if ui.button("Cancel").clicked() {
            ui_manager.is_add_object_window_open = false;
            ui_manager.new_object_properties = NewOctreeObjectProperties::default();
          }
        });
      });
  }

  fn draw_octree_options(ui: &mut Ui, ui_manager: &mut UIManager) {
    ui.heading("Primitive");
    ComboBox::from_label("Select the primitive")
      .selected_text(format!("{:?}", ui_manager.new_object_properties.primitive))
      .show_ui(ui, |ui| {
        ui.selectable_value(
          &mut ui_manager.new_object_properties.primitive,
          OctreePrimitive::Cube,
          "Cube",
        );
        ui.selectable_value(
          &mut ui_manager.new_object_properties.primitive,
          OctreePrimitive::Sphere,
          "Sphere",
        );
        ui.selectable_value(
          &mut ui_manager.new_object_properties.primitive,
          OctreePrimitive::Cylinder,
          "Cylinder",
        );
        ui.selectable_value(
          &mut ui_manager.new_object_properties.primitive,
          OctreePrimitive::Cone,
          "Cone",
        );
        ui.selectable_value(
          &mut ui_manager.new_object_properties.primitive,
          OctreePrimitive::Mesh,
          "Mesh",
        );
        ui.selectable_value(
          &mut ui_manager.new_object_properties.primitive,
          OctreePrimitive::Generic,
          "Generic",
        );
      });

    ui.separator();
    ui.heading("Properties");
    let props = &mut ui_manager.new_object_properties;
    ui.horizontal(|ui| {
      ui.label("Name: ");
      ui.text_edit_singleline(&mut props.name);
    });

    match props.primitive {
      OctreePrimitive::Sphere => {
        ui.horizontal(|ui| {
          ui.label("Radius: ");
          ui.add(egui::DragValue::new(&mut props.radius).speed(0.1));
        });
      }
      OctreePrimitive::Cube => {
        ui.horizontal(|ui| {
          ui.label("Width: ");
          ui.add(egui::DragValue::new(&mut props.size.x).speed(0.1));
          ui.label("Height: ");
          ui.add(egui::DragValue::new(&mut props.size.y).speed(0.1));
          ui.label("Depth: ");
          ui.add(egui::DragValue::new(&mut props.size.z).speed(0.1));
        });
      }
      OctreePrimitive::Cylinder | OctreePrimitive::Cone => {
        ui.horizontal(|ui| {
          ui.label("Radius: ");
          ui.add(egui::DragValue::new(&mut props.radius).speed(0.1));
        });
        ui.horizontal(|ui| {
          ui.label("Height: ");
          ui.add(egui::DragValue::new(&mut props.height).speed(0.1));
        });
      }
      OctreePrimitive::Mesh => {
        ui.horizontal(|ui| {
          ui.label("Obj Path: ");
          if ui.button("Select model").clicked() {
            let path = FileDialog::new().add_filter("OBJ", &["obj"]).pick_file();
            props.obj_path = path;
          }
        });
      }
      OctreePrimitive::Generic => {
        ui.horizontal(|ui| {
          ui.label("Octree Path: ");
          if ui.button("Select octree").clicked() {
            let path = FileDialog::new().add_filter("OBJ", &["oct"]).pick_file();
            props.obj_path = path;
          }
        });
      }
    }

    ui.separator();
    ui.heading("Octree Properties");
      ui.horizontal(|ui| {
        ui.label("Max Tree Depth: ");
        ui.add(egui::DragValue::new(&mut props.max_depth));
    });
      ui.horizontal(|ui| {
        ui.label("Spacing: ");
        ui.add(egui::DragValue::new(&mut props.spacing).speed(0.01));
    });
  }

  pub fn process_ui_commands(&mut self) {
    let commands: Vec<UICommand> = self.ui_manager.commands_queue.drain(..).collect();

    for command in commands {
      match command {
        UICommand::CreateOctree(props) => {
          let new_object = create_octree_object(props);
          self.scene.add_object(ProgramType::Instanced, new_object);
        }

        UICommand::DeleteObject(id) => {
          self.scene.remove_object(id);
        },

        UICommand::ApplyBoolean { left_id, right_id, operator } => {
          let left_object_rc = self.scene.objects_by_id.get(&left_id).unwrap().clone();
          let right_object_rc = self.scene.objects_by_id.get(&right_id).unwrap().clone();
          let new_object = OctreeBoolean::new(left_object_rc, right_object_rc, operator, 0.0);

          self.scene.remove_object(left_id);
          self.scene.remove_object(right_id);

          self.scene.add_object(ProgramType::Instanced, Rc::new(RefCell::new(new_object)));
          self.ui_manager.selected_object_id = None;
          self.ui_manager.selected_boolean_object_id = None;
        }

        UICommand::SaveOctree(id) => {
          if let Some(object) = self.scene.objects_by_id.get(&id) {
            let object = object.borrow();
            if let Some(octree_object) = object.as_octree_object() {
              if let Some(path) = FileDialog::new().add_filter("oct", &["oct"]).save_file() {
                let mut file = File::create(path).unwrap();
                if let Some(root) = octree_object.get_root() {
                  root.serialize(&mut file);
                }
              }
            }
          }
        }
      }
    }
  }
}

fn create_octree_object(props: NewOctreeObjectProperties) -> Rc<RefCell<dyn Object>> {
  let name = props.name.clone();

  match props.primitive {
    OctreePrimitive::Sphere => Rc::new(RefCell::new(OctreeSphere::new(
      name,
      props.radius,
      props.max_depth,
      props.spacing,
      None,
    ))),
    OctreePrimitive::Cube => Rc::new(RefCell::new(OctreeCube::new(
      name,
      props.size,
      props.max_depth,
      props.spacing,
      None,
    ))),
    OctreePrimitive::Cylinder => Rc::new(RefCell::new(OctreeCylinder::new(
      name,
      props.radius,
      props.height,
      props.max_depth,
      props.spacing,
      None,
    ))),
    OctreePrimitive::Cone => Rc::new(RefCell::new(OctreeCone::new(
      name,
      props.radius,
      props.height,
      props.max_depth,
      props.spacing,
      None,
    ))),
    OctreePrimitive::Mesh => Rc::new(RefCell::new(OctreeMesh::new(
      name,
      props.obj_path.unwrap(),
      props.max_depth,
      props.spacing,
      None,
    ))),
    OctreePrimitive::Generic => Rc::new(RefCell::new(OctreeGeneric::new(
      name,
      props.obj_path.unwrap(),
      props.max_depth,
      props.spacing,
      None,
    ))),
  }
}
