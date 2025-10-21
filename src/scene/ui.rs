use std::{cell::RefCell, rc::Rc};

use egui::{vec2, ComboBox, FullOutput, SidePanel, Ui, Window as EguiWindow};
use glam::Vec3;
use uuid::Uuid;

use crate::{objects::{object::Object, octree::{octree_boolean::{BooleanOperator, OctreeBoolean}, octree_cone::OctreeCone, octree_cube::OctreeCube, octree_cylinder::OctreeCylinder, octree_sphere::OctreeSphere}}, opengl::renderer::ProgramType, scene::{scene::Scene, window::Window}};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum UITab {
  Objects,
  Properties,
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
}

pub struct NewObjectProperties {
  name: String,

  radius: f32,
  height: f32,
  size: Vec3,

  max_depth: u32,
  spacing: f32,
}

impl Default for NewObjectProperties {
  fn default() -> Self {
    Self {
      name: "New Object".to_string(),
      radius: 1.0,
      height: 2.0,
      size: Vec3::ONE,
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
  pub new_octree_primitive: OctreePrimitive,
  pub new_object_properties: NewObjectProperties,

  pub boolean_operator: BooleanOperator,
  pub selected_boolean_object_id: Option<Uuid>,
}

impl UIManager {
  pub fn new() -> Self {
    UIManager {
      selected_tab: UITab::Objects,
      selected_object_id: None,
      is_add_object_window_open: false,
      new_object_type: ObjectType::Octree,
      new_octree_primitive: OctreePrimitive::Sphere,
      new_object_properties: NewObjectProperties::default(),
      boolean_operator: BooleanOperator::UNION,
      selected_boolean_object_id: None,
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

    Window::draw_add_object_window(&self.egui.context, ui_manager, scene);

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

    {
      let mut object = object_rc.borrow_mut();

      ui.heading("Name");
      ui.add(egui::TextEdit::singleline(object.get_name_mut()));
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

      ui.separator();
      if ui.button("Rebuild Octree").clicked() {
        if let Some(object) = object.as_octree_object_mut() {
          object.generate_octree();
        }
      }
    }

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
            if Some(*id) != ui_manager.selected_object_id && object.borrow().as_octree_object().is_some() {
              ui.selectable_value(&mut ui_manager.selected_boolean_object_id, Some(*id), object.borrow().get_name());
            }
          }
        });

      if ui.button("Apply").clicked() {
        if let Some(right_id) = ui_manager.selected_boolean_object_id {
          let left_object_rc = scene.objects_by_id.get(&selected_id).unwrap().clone();
          let right_object_rc = scene.objects_by_id.get(&right_id).unwrap().clone();

          let new_object = match ui_manager.boolean_operator {
            BooleanOperator::UNION => OctreeBoolean::union(left_object_rc, right_object_rc, 0.0),
            BooleanOperator::INTERSECTION => OctreeBoolean::intersection(left_object_rc, right_object_rc, 0.0),
            BooleanOperator::DIFFERENCE => OctreeBoolean::difference(left_object_rc, right_object_rc, 0.0),
          };

          scene.remove_object(selected_id);
          scene.remove_object(right_id);

          scene.add_object(ProgramType::Instanced, Rc::new(RefCell::new(new_object)));
          ui_manager.selected_object_id = None;
          ui_manager.selected_boolean_object_id = None;
        }
      }
    }
  }

  fn draw_add_object_window(ctx: &egui::Context, ui_manager: &mut UIManager, scene: &mut Scene) {
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
          if ui.button("Create").clicked() {
            let new_object = Window::create_object(ui_manager);
            scene.add_object(ProgramType::Instanced, new_object);
            ui_manager.is_add_object_window_open = false;
            // Reseta as propriedades para a próxima criação
            ui_manager.new_object_properties = NewObjectProperties::default();
          }
          if ui.button("Cancel").clicked() {
            ui_manager.is_add_object_window_open = false;
          }
        });
      });
  }

  fn draw_octree_options(ui: &mut Ui, ui_manager: &mut UIManager) {
    ui.heading("Primitive");
    ComboBox::from_label("Select the primitive")
      .selected_text(format!("{:?}", ui_manager.new_octree_primitive))
      .show_ui(ui, |ui| {
        ui.selectable_value(
          &mut ui_manager.new_octree_primitive,
          OctreePrimitive::Cube,
          "Cube",
        );
        ui.selectable_value(
          &mut ui_manager.new_octree_primitive,
          OctreePrimitive::Sphere,
          "Sphere",
        );
        ui.selectable_value(
          &mut ui_manager.new_octree_primitive,
          OctreePrimitive::Cylinder,
          "Cylinder",
        );
        ui.selectable_value(
          &mut ui_manager.new_octree_primitive,
          OctreePrimitive::Cone,
          "Cone",
        );
      });

    ui.separator();
    ui.heading("Properties");
    let props = &mut ui_manager.new_object_properties;
    ui.horizontal(|ui| {
      ui.label("Name: ");
      ui.text_edit_singleline(&mut props.name);
    });

    match ui_manager.new_octree_primitive {
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

  fn create_object(ui_manager: &UIManager) -> Rc<RefCell<dyn Object>> {
    let props = &ui_manager.new_object_properties;
    let name = props.name.clone();

    match ui_manager.new_object_type {
      ObjectType::Octree => match ui_manager.new_octree_primitive {
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
      },
    }
  }
}
