use std::{cell::RefCell, rc::Rc};

use egui::{ComboBox, Ui};
use uuid::Uuid;

use crate::{objects::{brep::winged_edge_object::{MemberType, WingedEdgeObject}, csg::csg_object::CSGPrimitives, octree::octree_boolean::BooleanOperator}, scene::{scene::Scene, ui::ui::{NewObjectProperties, UICommand, UIManager}, window::Window}};

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
  pub fn draw_winged_edge_object_properties(&mut self, ui: &mut Ui, _scene: &Scene, object: &mut WingedEdgeObject) {
    ui.separator();
    ui.heading("Winged Edge Properties");

    ui.heading("Adjacents View");
    ui.horizontal(|ui| {
      ui.label("Vertex: ");
      ui.add(egui::DragValue::new(&mut self.winged_edge_vertex_selected).range(0..=object.vertices.len()-1).speed(1));
      if ui.button("Select").clicked() && object.vertices.len() > 0 && self.winged_edge_vertex_selected < object.vertices.len() {
        object.highlight_member(MemberType::Vertex, self.winged_edge_vertex_selected);
      }
    });

    ui.horizontal(|ui| {
      ui.label("Edge: ");
      ui.add(egui::DragValue::new(&mut self.winged_edge_edge_selected).range(0..=object.edges.len()-1).speed(1));
      if ui.button("Select").clicked() && object.edges.len() > 0 && self.winged_edge_edge_selected < object.edges.len() {
        object.highlight_member(MemberType::Edge, self.winged_edge_edge_selected);
      }
    });

    ui.horizontal(|ui| {
      ui.label("Face: ");
      ui.add(egui::DragValue::new(&mut self.winged_edge_face_selected).range(0..=object.faces.len()-1).speed(1));
      if ui.button("Select").clicked() && object.faces.len() > 0 && self.winged_edge_face_selected < object.faces.len() {
        object.highlight_member(MemberType::Face, self.winged_edge_face_selected);
      }
    });
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
            ui.add(egui::DragValue::new(&mut props.subdivisions).range(2..=1000).speed(1));
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
