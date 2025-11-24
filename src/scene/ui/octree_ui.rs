use std::{cell::RefCell, path::PathBuf, rc::Rc};

use egui::{ComboBox, Ui};
use glam::Vec3;
use rfd::FileDialog;
use uuid::Uuid;

use crate::{objects::{instanced::instanced_object::InstacedObject, object::Object, octree::{octree_boolean::{BooleanOperator, OctreeBoolean}, octree_cone::OctreeCone, octree_cube::OctreeCube, octree_cylinder::OctreeCylinder, octree_generic::OctreeGeneric, octree_mesh::OctreeMesh, octree_object::OctreeObject, octree_sphere::OctreeSphere}}, opengl::renderer::ProgramType, scene::{scene::Scene, ui::ui::{NewObjectProperties, ObjectType, UICommand, UIManager}, window::Window}};


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
  pub primitive: OctreePrimitive,
  pub name: String,

  pub radius: f32,
  pub height: f32,
  pub size: Vec3,
  pub obj_path: Option<PathBuf>,

  pub max_depth: u32,
  pub spacing: f32,
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

impl UIManager {
  pub fn draw_octree_object_properties(&mut self, ui: &mut Ui, scene: &Scene, object: &mut dyn OctreeObject) {
    let selected_id = self.selected_object_id.unwrap();
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

    } else if let Some(generic) = any_mut.downcast_mut::<OctreeGeneric>() {
      ui.heading("Octree Generic Properties");
      ui.add(egui::Slider::new(&mut generic.max_depth, 1..=generic.original_max_depth).text("Max Depth"));
      ui.add(egui::Slider::new(&mut generic.spacing, 0.0..=1.0).text("Spacing"));
    }

    if let Some(object) = object.as_octree_object_mut() {
      if ui.button("Rebuild Octree").clicked() {
        object.generate_octree();
      }

      ui.horizontal(|ui| {
        ui.label(format!("Nº cubes: {}", object.get_instanced_cube().get_instances_count()));
        ui.label(format!("Volume: {}", object.get_volume()));
      });

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
            if Some(*id) != self.selected_object_id
              && object.borrow().as_octree_object().is_some()
              && !object.borrow().as_any().is::<OctreeGeneric>()
            {
              ui.selectable_value(&mut self.selected_boolean_object_id, Some(*id), object.borrow().get_name());
            }
          }
        });

      if ui.button("Apply").clicked() {
        if let Some(right_id) = self.selected_boolean_object_id {
          self.commands_queue.push(UICommand::ApplyBoolean {
            object_type: ObjectType::Octree,
            left_id: selected_id,
            right_id,
            operator: self.boolean_operator,
          })
        }
      }
    }
  }

  pub fn draw_octree_creation_options(&mut self, ui: &mut Ui) {
    if let NewObjectProperties::Octree(octree_properties) = &mut self.new_object_properties {
      ui.heading("Primitive");
      ComboBox::from_label("Select the primitive")
        .selected_text(format!("{:?}", octree_properties.primitive))
        .show_ui(ui, |ui| {
          ui.selectable_value(
            &mut octree_properties.primitive,
            OctreePrimitive::Cube,
            "Cube",
          );
          ui.selectable_value(
            &mut octree_properties.primitive,
            OctreePrimitive::Sphere,
            "Sphere",
          );
          ui.selectable_value(
            &mut octree_properties.primitive,
            OctreePrimitive::Cylinder,
            "Cylinder",
          );
          ui.selectable_value(
            &mut octree_properties.primitive,
            OctreePrimitive::Cone,
            "Cone",
          );
          ui.selectable_value(
            &mut octree_properties.primitive,
            OctreePrimitive::Mesh,
            "Mesh",
          );
          ui.selectable_value(
            &mut octree_properties.primitive,
            OctreePrimitive::Generic,
            "Load Model",
          );
        });

      ui.separator();
      ui.heading("Properties");
      ui.horizontal(|ui| {
        ui.label("Name: ");
        ui.text_edit_singleline(&mut octree_properties.name);
      });

      match octree_properties.primitive {
        OctreePrimitive::Sphere => {
          ui.horizontal(|ui| {
            ui.label("Radius: ");
            ui.add(egui::DragValue::new(&mut octree_properties.radius).speed(0.1));
          });
        }
        OctreePrimitive::Cube => {
          ui.horizontal(|ui| {
            ui.label("Width: ");
            ui.add(egui::DragValue::new(&mut octree_properties.size.x).speed(0.1));
            ui.label("Height: ");
            ui.add(egui::DragValue::new(&mut octree_properties.size.y).speed(0.1));
            ui.label("Depth: ");
            ui.add(egui::DragValue::new(&mut octree_properties.size.z).speed(0.1));
          });
        }
        OctreePrimitive::Cylinder | OctreePrimitive::Cone => {
          ui.horizontal(|ui| {
            ui.label("Radius: ");
            ui.add(egui::DragValue::new(&mut octree_properties.radius).speed(0.1));
          });
          ui.horizontal(|ui| {
            ui.label("Height: ");
            ui.add(egui::DragValue::new(&mut octree_properties.height).speed(0.1));
          });
        }
        OctreePrimitive::Mesh => {
          ui.horizontal(|ui| {
            ui.label("Obj Path: ");
            let placeholder = if let Some(path) = &octree_properties.obj_path {
              path.file_stem().unwrap().to_str().unwrap()
            } else {
              "Select model"
            };
            if ui.button(placeholder).clicked() {
              let path = FileDialog::new().add_filter("OBJ", &["obj"]).pick_file();
              let path = path.unwrap();
              let stem = path.file_stem().unwrap().to_str().unwrap();
              octree_properties.name = stem.to_string();
              octree_properties.obj_path = Some(path);
            }
          });
        }
        OctreePrimitive::Generic => {
          ui.horizontal(|ui| {
            ui.label("Octree Path: ");
            let placeholder = if let Some(path) = &octree_properties.obj_path {
              path.file_stem().unwrap().to_str().unwrap()
            } else {
              "Select octree"
            };
            if ui.button(placeholder).clicked() {
              let path = FileDialog::new().add_filter("OBJ", &["oct"]).pick_file();
              let path = path.unwrap();
              let stem = path.file_stem().unwrap().to_str().unwrap();
              octree_properties.name = stem.to_string();
              octree_properties.obj_path = Some(path);
            }
          });
        }
      }

      ui.separator();
      ui.heading("Octree Properties");
        ui.horizontal(|ui| {
          ui.label("Max Tree Depth: ");
          ui.add(egui::DragValue::new(&mut octree_properties.max_depth));
      });
        ui.horizontal(|ui| {
          ui.label("Spacing: ");
          ui.add(egui::DragValue::new(&mut octree_properties.spacing).speed(0.01));
      });

      ui.separator();
      ui.horizontal(|ui| {
        let should_enable_creation = (
          octree_properties.primitive != OctreePrimitive::Mesh
          && octree_properties.primitive != OctreePrimitive::Generic
        ) || octree_properties.obj_path.is_some();

        if ui.button("Create").clicked() {
          if should_enable_creation {
            self.commands_queue.push(UICommand::CreateObject(NewObjectProperties::Octree(octree_properties.clone())));
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
  pub fn create_octree_object(props: NewOctreeObjectProperties) -> Rc<RefCell<dyn Object>> {
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

  pub fn apply_octree_boolean(&mut self, left_id: Uuid, right_id: Uuid, operator: BooleanOperator) {
    let left_object_rc = self.scene.objects_by_id.get(&left_id).unwrap().clone();
    let right_object_rc = self.scene.objects_by_id.get(&right_id).unwrap().clone();
    let new_object = OctreeBoolean::new(left_object_rc, right_object_rc, operator, 0.0);

    self.scene.remove_object(left_id);
    self.scene.remove_object(right_id);

    self.scene.add_object(ProgramType::Instanced, Rc::new(RefCell::new(new_object)));
    self.ui_manager.selected_object_id = None;
    self.ui_manager.selected_boolean_object_id = None;
  }
}
