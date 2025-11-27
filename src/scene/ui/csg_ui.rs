use std::{cell::RefCell, fs::File, io::BufReader, path::PathBuf, rc::Rc};

use egui::{ComboBox, Ui};
use rfd::FileDialog;
use uuid::Uuid;

use crate::{objects::{csg::csg_object::{CSGObject, CSGPrimitives}, octree::octree_boolean::BooleanOperator}, scene::{scene::Scene, ui::ui::{NewObjectProperties, ObjectType, UICommand, UIManager}, window::Window}};

#[derive(Clone, Debug)]
pub struct NewCSGObjectProperties {
  primitive: CSGPrimitives,
  name: String,
  radius: f32,
  height: f32,
  subdivisions: u32,
  obj_path: Option<PathBuf>,
}

impl Default for NewCSGObjectProperties {
  fn default() -> Self {
    NewCSGObjectProperties {
      primitive: CSGPrimitives::Sphere,
      name: String::from("New Object"),
      subdivisions: 30,
      height: 2.0,
      radius: 1.0,
      obj_path: None,
    }
  }
}

impl UIManager {
  pub fn draw_csg_object_properties(&mut self, ui: &mut Ui, scene: &Scene) {
    let selected_id = self.selected_object_id.unwrap();

    ui.separator();
    ui.heading("Boolean Operation");

    ComboBox::from_label("Operation")
      .selected_text(format!("{}", self.boolean_operator))
      .show_ui(ui, |ui| {
        ui.selectable_value(&mut self.boolean_operator, BooleanOperator::UNION, "Union");
        ui.selectable_value(&mut self.boolean_operator, BooleanOperator::INTERSECTION, "Intersection");
        ui.selectable_value(&mut self.boolean_operator, BooleanOperator::DIFFERENCE, "Difference");
      });

    let mut selected_name = "Select Object".to_string();
    if let Some(selected_id) = self.selected_boolean_object_id {
      if let Some(object) = scene.objects_by_id.get(&selected_id) {
        selected_name = object.borrow().get_name();
      }
    }

    ComboBox::from_label("Object")
      .selected_text(selected_name)
      .show_ui(ui, |ui| {
        for (id, object) in scene.objects_by_id.iter() {
          if Some(*id) != self.selected_object_id && object.borrow().as_any().is::<CSGObject>() {
            ui.selectable_value(&mut self.selected_boolean_object_id, Some(*id), object.borrow().get_name());
          }
        }
      });

      if ui.button("Apply").clicked() {
        if let Some(right_id) = self.selected_boolean_object_id {
          self.commands_queue.push(UICommand::ApplyBoolean {
            object_type: ObjectType::CSG,
            left_id: selected_id,
            right_id,
            operator: self.boolean_operator,
          })
        }
      }
  }

  pub fn draw_csg_creation_options(&mut self, ui: &mut Ui) {
    if let NewObjectProperties::CSG(props) = &mut self.new_object_properties {
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
        ui.selectable_value(
          &mut props.primitive,
          CSGPrimitives::Generic,
          "Generic"
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
            ui.label("Radius: ");
            ui.add(egui::DragValue::new(&mut props.radius).speed(0.1));
          });
          ui.horizontal(|ui| {
            ui.label("Subdivisions: ");
            ui.add(egui::DragValue::new(&mut props.subdivisions).range(0..=100).speed(1));
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
            ui.add(egui::DragValue::new(&mut props.subdivisions).range(4..=100).speed(1));
          });
        },
        CSGPrimitives::Generic => {
          ui.label("CSG File: ");
          let placeholder = if let Some(path) = &props.obj_path {
            path.file_stem().unwrap().to_str().unwrap()
          } else {
            "Select csg"
          };
          if ui.button(placeholder).clicked() {
            let path = FileDialog::new().add_filter("CSG", &["csg"]).pick_file();
            let path = path.unwrap();
            let stem = path.file_stem().unwrap().to_str().unwrap();
            props.name = stem.to_string();
            props.obj_path = Some(path);
          }
        }
      }

      ui.separator();
      ui.horizontal(|ui| {
        let should_enable_creation = true;
        if ui.button("Create").clicked() {
          if should_enable_creation {
            self.commands_queue.push(UICommand::CreateObject(NewObjectProperties::CSG(props.clone())));
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
  pub fn create_csg_object(props: NewCSGObjectProperties) -> Rc<RefCell<CSGObject>> {
    match props.primitive {
      CSGPrimitives::Cube => Rc::new(RefCell::new(CSGObject::new_cube(
        props.name,
      ))),
      CSGPrimitives::Sphere => Rc::new(RefCell::new(CSGObject::new_sphere(
        props.name,
        props.radius,
        props.subdivisions,
      ))),
      CSGPrimitives::Cylinder => Rc::new(RefCell::new(CSGObject::new_cylinder(
        props.name,
        props.radius,
        props.height,
        props.subdivisions,
      ))),
      CSGPrimitives::Cone => Rc::new(RefCell::new(CSGObject::new_cone(
        props.name,
        props.radius,
        props.height,
        props.subdivisions,
      ))),
      CSGPrimitives::Generic => {
        let path = props.obj_path.unwrap();
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);

        let csg_object = CSGObject::deserialize(props.name, &mut reader).unwrap();
        Rc::new(RefCell::new(csg_object))
      },
    }
  }

  pub fn apply_csg_boolean(&mut self, left_id: Uuid, right_id: Uuid, operator: BooleanOperator) {
    let left_object_rc = self.scene.objects_by_id.get(&left_id).unwrap().clone();
    let right_object_rc = self.scene.objects_by_id.get(&right_id).unwrap().clone();

    self.scene.remove_object(right_id);

    let mut binding = left_object_rc.borrow_mut();
    let left = binding.as_any_mut().downcast_mut::<CSGObject>().unwrap();
    left.boolean(right_object_rc.borrow().as_any().downcast_ref::<CSGObject>().unwrap(), operator);
  }
}
