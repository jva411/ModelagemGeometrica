use std::{cell::RefCell, rc::Rc};

use egui::{ComboBox, Ui};
use uuid::Uuid;

use crate::{objects::{brep::winged_edge_object::WingedEdgeObject, csg::csg_object::CSGPrimitives, octree::octree_boolean::BooleanOperator}, scene::{scene::Scene, ui::ui::{NewObjectProperties, UICommand, UIManager}, window::Window}};

#[derive(Clone, Debug)]
pub struct NewWingedEdgeObjectProperties {
  primitive: CSGPrimitives,
  name: String,
  radius: f32,
  height: f32,
  subdivisions: u32,
}

impl Default for NewWingedEdgeObjectProperties {
  fn default() -> Self {
    NewWingedEdgeObjectProperties {
      primitive: CSGPrimitives::Cube,
      name: String::from("New Object"),
      subdivisions: 30,
      height: 2.0,
      radius: 1.0,
    }
  }
}

impl UIManager {
  pub fn draw_winged_edge_object_properties(&mut self, _ui: &mut Ui, _scene: &Scene, _object: &mut WingedEdgeObject) {

  }

  pub fn draw_winged_edge_creation_options(&mut self, ui: &mut Ui) {
    if let NewObjectProperties::WingedEdge(props) = &mut self.new_object_properties {
      ui.heading("Primitive");
      ComboBox::from_label("Select the primitive")
      .selected_text(format!("{:?}", props.primitive))
      .show_ui(ui, |ui| {
        ui.selectable_value(
          &mut props.primitive,
          CSGPrimitives::Sphere,
          "Sphere",
        );
        ui.selectable_value(
          &mut props.primitive,
          CSGPrimitives::Cube,
          "Cube",
        );
        ui.selectable_value(
          &mut props.primitive,
          CSGPrimitives::Cylinder,
          "Cylinder",
        );
        ui.selectable_value(
          &mut props.primitive,
          CSGPrimitives::Cone,
          "Cone",
        );
      });

      ui.separator();
      ui.heading("Properties");
      ui.horizontal(|ui| {
        ui.label("Name: ");
        ui.text_edit_singleline(&mut props.name);
      });

      match props.primitive {
        CSGPrimitives::Cube => {},
        CSGPrimitives::Sphere => {
          ui.horizontal(|ui| {
            ui.label("Subdivisions: ");
            ui.add(egui::DragValue::new(&mut props.subdivisions).range(3..=100).speed(1));
          });
        },
        CSGPrimitives::Cylinder | CSGPrimitives::Cone => {
          ui.horizontal(|ui| {
            ui.label("Radius: ");
            ui.add(egui::DragValue::new(&mut props.radius).speed(0.1));
          });
          ui.horizontal(|ui| {
            ui.label("Height: ");
            ui.add(egui::DragValue::new(&mut props.height).speed(0.1));
          });
          ui.horizontal(|ui| {
            ui.label("Subdivisions: ");
            ui.add(egui::DragValue::new(&mut props.subdivisions).range(3..=1000).speed(1));
          });
        },
        _ => {}
      }

      ui.separator();
      ui.horizontal(|ui| {
        let should_enable_creation = true;
        if ui.button("Create").clicked() {
          if should_enable_creation {
            self.commands_queue.push(UICommand::CreateObject(NewObjectProperties::WingedEdge(props.clone())));
            self.is_add_object_window_open = false;
          }
        }
        if ui.button("Cancel").clicked() {
          self.is_add_object_window_open = false;
        }
      });
    }
  }
}

impl Window {
  pub fn create_winged_edge_object(props: NewWingedEdgeObjectProperties) -> Rc<RefCell<WingedEdgeObject>> {
    let mut object = match props.primitive {
      CSGPrimitives::Cube => WingedEdgeObject::new_cube(
        props.name,
      ),
      CSGPrimitives::Sphere => WingedEdgeObject::new_sphere(
        props.name,
        props.subdivisions as usize,
      ),
      CSGPrimitives::Cylinder => WingedEdgeObject::new_cylinder(
        props.name,
        props.subdivisions as usize,
        props.height,
        props.radius,
      ),
      CSGPrimitives::Cone => WingedEdgeObject::new_cone(
        props.name,
        props.subdivisions as usize,
        props.height,
        props.radius,
      ),
      _ => unimplemented!(),
    };

    object.build_opengl();
    return Rc::new(RefCell::new(object));
  }

  pub fn apply_winged_edge_boolean(&mut self, _left_id: Uuid, _right_id: Uuid, _operator: BooleanOperator) {
    unimplemented!();
  }
}
