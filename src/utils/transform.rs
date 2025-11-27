use std::io::Write;

use glam::{Mat4, Quat, Vec3};

use crate::{objects::instanced::instanced_cube::SIZE_F32, opengl::program::Program};

#[derive(Debug, Clone)]
pub struct Rotation {
  pub yaw: f32,
  pub pitch: f32,
  pub roll: f32,
}

impl Rotation {
  pub fn new(yaw: f32, pitch: f32, roll: f32) -> Self {
    Rotation {
      yaw,
      pitch,
      roll,
    }
  }

  const fn zero() -> Self {
    Rotation {
      yaw: 0.0,
      pitch: 0.0,
      roll: 0.0,
    }
  }

  pub fn to_quat(&self) -> Quat {
    Quat::from_axis_angle(Vec3::Y, self.yaw.to_radians())
      * Quat::from_axis_angle(Vec3::X, self.pitch.to_radians())
      * Quat::from_axis_angle(Vec3::Z, self.roll.to_radians())
  }
}

#[derive(Debug, Clone)]
pub struct Transform {
  pub translation: Vec3,
  pub rotation: Rotation,
  pub scale: Vec3,
}

#[allow(dead_code)]
impl Transform {
  pub fn new() -> Self {
    Transform {
      translation: Vec3::ZERO,
      rotation: Rotation::zero(),
      scale: Vec3::ONE,
    }
  }

  pub fn translate3f(&mut self, x: f32, y: f32, z: f32) {
    self.translation += Vec3::new(x, y, z);
  }
  pub fn translatev3f(&mut self, v: Vec3) {
    self.translation += Vec3::new(v.x, v.y, v.z);
  }

  pub fn scale3f(&mut self, x: f32, y: f32, z: f32) {
    self.scale *= Vec3::new(x, y, z);
  }
  pub fn scalev3f(&mut self, v: Vec3) {
    self.scale *= Vec3::new(v.x, v.y, v.z);
  }

  pub fn add_yaw(&mut self, radians: f32) {
    self.rotation.yaw += radians;
  }
  pub fn add_pitch(&mut self, radians: f32) {
    self.rotation.pitch += radians;
  }
  pub fn add_roll(&mut self, radians: f32) {
    self.rotation.roll += radians;
  }

  pub fn build_model(&self) -> Mat4 {
    Mat4::from_scale_rotation_translation(self.scale, self.rotation.to_quat(), self.translation)
  }

  pub fn send_to_program(&self, program: &Program) {
    program.set_uniform_matrix4f("model", self.build_model()).unwrap();
  }

  pub fn send_to_program_instanced(&self, program: &Program, index: i32) {
    program.set_uniform_matrix4f(&format!("models[{}]", index), self.build_model()).unwrap();
  }

  pub fn concat(&self, other: &Transform) -> Transform {
    Transform {
      translation: self.translation + other.translation,
      rotation: Rotation {
        yaw: self.rotation.yaw + other.rotation.yaw,
        pitch: self.rotation.pitch + other.rotation.pitch,
        roll: self.rotation.roll + other.rotation.roll,
      },
      scale: self.scale * other.scale,
    }
  }

  pub fn inverse(&self) -> Transform {
    Transform {
      translation: -self.translation,
      rotation: Rotation {
        yaw: -self.rotation.yaw,
        pitch: -self.rotation.pitch,
        roll: -self.rotation.roll,
      },
      scale: 1.0 / self.scale,
    }
  }

  pub fn serialize(&self, writer: &mut impl Write) -> std::io::Result<()> {
    writer.write_all(&self.translation.x.to_be_bytes())?;
    writer.write_all(&self.translation.y.to_be_bytes())?;
    writer.write_all(&self.translation.z.to_be_bytes())?;

    writer.write_all(&self.rotation.yaw.to_be_bytes())?;
    writer.write_all(&self.rotation.pitch.to_be_bytes())?;
    writer.write_all(&self.rotation.roll.to_be_bytes())?;

    writer.write_all(&self.scale.x.to_be_bytes())?;
    writer.write_all(&self.scale.y.to_be_bytes())?;
    writer.write_all(&self.scale.z.to_be_bytes())?;

    Ok(())
  }

  pub fn deserialize(reader: &mut impl std::io::Read) -> std::io::Result<Self> {
    let mut buffer = [0; 9 * SIZE_F32 as usize];
    reader.read_exact(&mut buffer)?;

    let mut buffer = buffer
      .chunks(SIZE_F32 as usize)
      .map(|chunk| f32::from_be_bytes(chunk.try_into().unwrap()));

    let translation = Vec3::new(
      buffer.next().unwrap(),
      buffer.next().unwrap(),
      buffer.next().unwrap(),
    );

    let rotation = Rotation::new(
      buffer.next().unwrap(),
      buffer.next().unwrap(),
      buffer.next().unwrap(),
    );

    let scale = Vec3::new(
      buffer.next().unwrap(),
      buffer.next().unwrap(),
      buffer.next().unwrap(),
    );

    Ok(Transform {
      translation,
      rotation,
      scale,
    })
  }
}

impl Default for Transform {
  fn default() -> Self { Self::new() }
}
