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

  let left = crate::objects::octree::octree_cube::OctreeCube::new(
    "Octree Cube".to_string(),
    Vec3::splat(1.0),
    6,
    0.0,
    None
  );
  let right = crate::objects::octree::octree_sphere::OctreeSphere::new(
    "Octree Sphere".to_string(),
    0.65,
    6,
    0.0,
    None
  );

  let left = Rc::new(RefCell::new(left));
  let right = Rc::new(RefCell::new(right));
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
