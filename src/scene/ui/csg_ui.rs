use std::{cell::RefCell, rc::Rc};

use egui::{ComboBox, Ui};
use uuid::Uuid;

use crate::{objects::{csg::csg_object::{CSGObject, CSGPrimitives}, object::Object, octree::octree_boolean::BooleanOperator}, scene::{scene::Scene, ui::ui::{NewObjectProperties, ObjectType, UICommand, UIManager}, window::Window}};

#[derive(Clone, Debug)]
pub struct NewCSGObjectProperties {
  primitive: CSGPrimitives,
  name: String,
}

impl Default for NewCSGObjectProperties {
  fn default() -> Self {
    NewCSGObjectProperties {
      primitive: CSGPrimitives::Cube,
      name: String::from("Cube"),
    }
  }
}

impl UIManager {
  pub fn draw_csg_object_properties(&mut self, ui: &mut Ui, scene: &Scene) {
    let selected_id = self.selected_object_id.unwrap();
    // let any_mut = object.as_any_mut();

    // if let Some(cube) = any_mut.downcast_mut::<CSGObject>() {
    // }

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
          CSGPrimitives::Cube,
          "Cube",
        );
      });

      ui.separator();
      ui.heading("Properties");
      ui.horizontal(|ui| {
        ui.label("Name: ");
        ui.text_edit_singleline(&mut props.name);
      });

      match props.primitive {
        CSGPrimitives::Cube => { }
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
      CSGPrimitives::Cube => Rc::new(RefCell::new(CSGObject::new(
        CSGPrimitives::Cube,
        props.name,
      ))),
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
