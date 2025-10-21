mod lights;
mod objects;
mod opengl;
mod scene;
mod utils;


use std::cell::RefCell;
use std::rc::Rc;

use glam::Vec3;

use crate::lights::point_light::PointLight;
use crate::objects::octree::octree_boolean::OctreeBoolean;
use crate::objects::octree::octree_cone::OctreeCone;
use crate::objects::octree::octree_sphere::OctreeSphere;
use crate::scene::window::Window;


const SCENE_WIDTH: u32 = 800;
const UI_WIDTH: u32 = 350;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = SCENE_WIDTH + UI_WIDTH;


fn main() {
  // Window setup
  let mut window = Window::new(
    "Modelagem Geométrica",
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
    SCENE_WIDTH,
  );

  let cone = OctreeCone::new(
    "Octree Cone".to_string(),
    1.0,
    2.0,
    8,
    0.0,
    None
  );
  let sphere = OctreeSphere::new(
    "Octree Sphere".to_string(),
    0.7,
    8,
    0.0,
    None
  );

  let left = Rc::new(RefCell::new(cone));
  let right = Rc::new(RefCell::new(sphere));
  let result = OctreeBoolean::difference(left, right, 0.0);
  window.scene.add_object(opengl::renderer::ProgramType::Instanced, Rc::new(RefCell::new(result)));

  // Scene setup
  let mut l0 = PointLight::new(
    Vec3::from([1.0, 1.0, 1.0]),
    Vec3::from([0.4, 0.4, 0.4]),
    Vec3::from([0.3, 0.3, 0.3]),
  );
  l0.transform.translate3f(-0.5, 1.0, 4.0);
  window.scene.add_light(Box::new(l0));

  window.init();
}
