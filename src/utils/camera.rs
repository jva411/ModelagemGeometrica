use glam::{Mat4, Quat, Vec3};
use std::f32::consts::PI;

use crate::opengl::{program::Program, renderer::Renderer};

#[derive(Debug, Clone, Copy)]
pub enum Projection {
  Perspective,
  Orthographic,
}

#[derive(Debug, Clone, Copy)]
pub struct CameraTransform {
  pub position: Vec3,
  pub rotation: Quat,
  pub yaw: f32,
  pub pitch: f32,
  pub roll: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
  pub transform: CameraTransform,
  pub fov: f32,
  pub aspect: f32,
  pub near: f32,
  pub far: f32,
  pub projection: Projection,
}

impl Camera {
  pub fn new() -> Self {
    Camera {
      transform: CameraTransform::new(),
      fov: 45.0,
      aspect: 8.0/6.0,
      near: 0.1,
      far: 100.0,
      projection: Projection::Perspective,
    }
  }

  pub fn from(transform: CameraTransform, fov: f32, aspect: f32, near: f32, far: f32, projection: Projection) -> Self {
    return Camera {
      transform,
      fov,
      aspect,
      near,
      far,
      projection,
    };
  }

  pub fn get_view(&self) -> Mat4 {
    Mat4::from_rotation_translation(self.transform.rotation, self.transform.position).inverse()
  }

  pub fn get_projection(&self) -> Mat4 {
    return match self.projection {
      Projection::Perspective => Mat4::perspective_rh(self.fov.to_radians(), self.aspect, self.near, self.far),
      Projection::Orthographic => Mat4::orthographic_rh(-self.aspect, self.aspect, -1.0, 1.0, self.near, self.far),
    };
  }

  pub fn send_to_program(&self, program: &Program) {
    program.set_uniform_matrix4f("view", self.get_view()).unwrap();
    program.set_uniform_matrix4f("projection", self.get_projection()).unwrap();
    program.set_uniform_vec3f("cameraPosition", self.transform.position).unwrap();
  }
}

impl CameraTransform {
  pub fn new() -> Self {
    return CameraTransform {
      position: Vec3::ZERO,
      rotation: Quat::IDENTITY,
      yaw: 0.0,
      pitch: 0.0,
      roll: 0.0,
    };
  }

  pub fn direction(&self) -> Vec3 {
    self.rotation * Vec3::NEG_Z
  }

  pub fn up(&self) -> Vec3 {
    self.rotation * Vec3::Y
  }

  pub fn right(&self) -> Vec3 {
    self.rotation * Vec3::X
  }

  pub fn translate(&mut self, forward: f32, right: f32, up: f32) {
    let direction = self.direction();
    let right_vec = self.right();

    self.position += Vec3::new(direction.x, 0.0, direction.z).normalize_or_zero() * forward;
    self.position += Vec3::new(right_vec.x, 0.0, right_vec.z).normalize_or_zero() * right;
    self.position += Vec3::Y * up;
  }

  pub fn add_yaw(&mut self, radians: f32) {
    self.yaw += radians;
    self.update_orientation();
  }

  pub fn add_pitch(&mut self, radians: f32) {
    self.pitch += radians;
    let max_pitch = (PI / 2.0) - 0.01;
    self.pitch = self.pitch.clamp(-max_pitch, max_pitch);
    self.update_orientation();
  }

  pub fn add_roll(&mut self, radians: f32) {
    self.roll += radians;
    self.update_orientation();
  }

  fn update_orientation(&mut self) {
    let yaw_quat = Quat::from_axis_angle(Vec3::Y, self.yaw);
    let pitch_quat = Quat::from_axis_angle(Vec3::X, self.pitch);
    let roll_quat = Quat::from_axis_angle(Vec3::Z, self.roll);

    self.rotation = (yaw_quat * pitch_quat * roll_quat).normalize();
  }
}
