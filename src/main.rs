mod lights;
mod objects;
mod opengl;
mod scene;
mod utils;


use glam::Vec3;

use crate::lights::point_light::PointLight;
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
