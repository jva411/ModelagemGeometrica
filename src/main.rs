mod lights;
mod objects;
mod opengl;
mod scene;
mod utils;


use glam::Vec3;

use crate::lights::point_light::PointLight;
use crate::objects::cube::Cube;
use crate::objects::instanced::instanced_cube::InstancedCube;
use crate::objects::instanced::instanced_object::InstacedObject;
use crate::objects::triangle::Triangle;
use crate::scene::window::Window;
use crate::utils::material::Material;
use crate::utils::transform::Transform;


const SCENE_WIDTH: u32 = 800;
const UI_WIDTH: u32 = 300;
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

  let divisions = 60;
  let cube_size = 1.0 / (divisions as f32);

  let outer_radius = 0.8;
  let thickness = 0.05;
  let inner_radius = outer_radius - thickness;

  let scale = 0.5;
  let translate = Vec3::new(0.0, 0.0, -3.0);
  let spacing = 0.0;

  let mut instanced_cube = InstancedCube::new(
  Some(Material::new(
    Vec3::splat(0.8),
    Vec3::splat(1.0),
    128.0
  )));

  for i in -divisions..=divisions {
    for j in -divisions..=divisions {
      for k in -divisions..=divisions {
        let x = (i as f32) * cube_size;
        let y = (j as f32) * cube_size;
        let z = (k as f32) * cube_size;
        let vec = Vec3::new(x, y, z);

        let distance_from_center = vec.length();

        if distance_from_center > outer_radius || distance_from_center < inner_radius {
          continue;
        }

        // let mut cube = Cube::new(
        //   None,
        //   Some(Material::new(
        //     Vec3::splat(0.8),
        //     Vec3::splat(1.0),
        //     128.0
        //   ))
        // );
        // cube.transform.translatev3f(translate + vec * scale);
        // cube.transform.scale3f(scale * cube_size * (1.0 - spacing), scale * cube_size * (1.0 - spacing), scale * cube_size * (1.0 - spacing));
        // window.scene.add_object(Box::new(cube));

        let mut transform = Transform::new();
        transform.translatev3f(translate + vec * scale);
        transform.scale3f(scale * cube_size * (1.0 - spacing), scale * cube_size * (1.0 - spacing), scale * cube_size * (1.0 - spacing));
        instanced_cube.add_instance(transform);
      }
    }
  }
  instanced_cube.setup_instances();
  window.scene.add_instanced_object(Box::new(instanced_cube));

  let mut l0 = PointLight::new(
    Vec3::from([1.0, 1.0, 1.0]),
    Vec3::from([0.4, 0.4, 0.4]),
    Vec3::from([0.2, 0.2, 0.2]),
  );
  l0.transform.translate3f(-0.5, 1.0, 0.0);
  window.scene.add_light(Box::new(l0));

  window.init();
}
