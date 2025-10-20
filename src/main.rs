mod lights;
mod objects;
mod opengl;
mod scene;
mod utils;


use std::cell::RefCell;
use std::rc::Rc;

use glam::Vec3;

use crate::lights::point_light::PointLight;
use crate::objects::cube::Cube;
use crate::objects::object::Object;
use crate::objects::octree::octree_cone::OctreeCone;
use crate::objects::octree::octree_cube::OctreeCube;
use crate::objects::octree::octree_cylinder::OctreeCylinder;
use crate::objects::octree::octree_sphere::OctreeSphere;
use crate::objects::triangle::Triangle;
use crate::opengl::renderer::ProgramType;
use crate::scene::window::Window;
use crate::utils::material::Material;
use crate::utils::transform;


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

  // Scene setup
  let mut triangle = Triangle::new(
    None,
    Some(Material::new(
      Vec3::from([1.0, 0.15, 0.15]),
      Vec3::from([0.3, 0.15, 0.15]),
      1.0,
    ))
  );
  triangle.transform.translate3f(0.0, 0.0, -2.5);
  // window.scene.add_object(Box::new(triangle));

  let mut cube = Cube::new(None, None);
  cube.transform.translate3f(0.0, 0.0, -3.0);
  // cube.transform.scale3f(1.0, 1.5, 0.7);
  // window.scene.add_object(Box::new(cube));

  // let mut octree_sphere = OctreeSphere::new(
  //   1.3,
  //   7,
  //   0.0,
  //   None,
  // );
  // let transform = octree_sphere.get_transform_mut();
  // transform.translate3f(0.0, 0.0, -4.0);
  // println!("OctreeSphere instances: {}", octree_sphere.instanced_cube.as_ref().unwrap().instances_transforms.len());
  // window.scene.add_object(ProgramType::Instanced, Box::new(octree_sphere));

  // let mut octree_cylinder = OctreeCylinder::new(
  //   0.7,
  //   2.5,
  //   9,
  //   0.0,
  //   None,
  // );
  // let transform = octree_cylinder.get_transform_mut();
  // transform.translate3f(0.0, 0.0, -4.0);
  // println!("OctreeCylinder instances: {}", octree_cylinder.instanced_cube.as_ref().unwrap().instances_transforms.len());
  // window.scene.add_object(ProgramType::Instanced, Box::new(octree_cylinder));

  // let mut octree_cone = OctreeCone::new(
  //   0.7,
  //   2.5,
  //   7,
  //   0.4,
  //   None,
  // );
  // let transform = octree_cone.get_transform_mut();
  // transform.translate3f(0.0, 0.0, -4.0);
  // println!("OctreeCone instances: {}", octree_cone.instanced_cube.as_ref().unwrap().instances_transforms.len());
  // window.scene.add_object(ProgramType::Instanced, Rc::new(RefCell::new(octree_cone)));

  // let mut octree_cube = OctreeCube::new(
  //   Vec3::from([1.0, 1.5, 1.0]),
  //   7,
  //   0.0,
  //   None,
  // );
  // let transform = octree_cube.get_transform_mut();
  // transform.translate3f(0.0, 0.0, -4.0);
  // println!("OctreeCube instances: {}", octree_cube.instanced_cube.as_ref().unwrap().instances_transforms.len());
  // window.scene.add_object(ProgramType::Instanced, Box::new(octree_cube));

  let mut l0 = PointLight::new(
    Vec3::from([1.0, 1.0, 1.0]),
    Vec3::from([0.4, 0.4, 0.4]),
    Vec3::from([0.3, 0.3, 0.3]),
  );
  l0.transform.translate3f(-0.5, 1.0, 0.0);
  window.scene.add_light(Box::new(l0));

  window.init();
}
