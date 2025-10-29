use crate::{opengl::{vao::VAO, vbo::VBO, program::Program}, utils::transform::Transform};

#[allow(dead_code)]
pub struct Axes {
  pub vao: VAO,
  pub vbo: VBO,
  pub transform: Transform,
  vertex_count: i32,
}

const AXES_VERTICES: [f32; 36] = [
//   X,   Y,   Z,     R,   G,   B
   0.0, 0.0, 0.0,   1.0, 0.0, 0.0, // Axis X
   1.0, 0.0, 0.0,   1.0, 0.0, 0.0,

   0.0, 0.0, 0.0,   0.0, 1.0, 0.0, // Axis Y
   0.0, 1.0, 0.0,   0.0, 1.0, 0.0,

   0.0, 0.0, 0.0,   0.0, 0.0, 1.0, // Axis Z
   0.0, 0.0, 1.0,   0.0, 0.0, 1.0,
];

#[allow(dead_code)]
impl Axes {
  pub fn new() -> Self {
    let vao = VAO::new();
    let vbo = VBO::new();
    vao.bind();
    vbo.bind();

    vbo.send_data(&AXES_VERTICES);
    let vertex_count = 6;

    let stride = (6 * std::mem::size_of::<f32>()) as u32;
    vao.add_attribute(0, stride, 0);
    vao.add_attribute(1, stride, (3 * std::mem::size_of::<f32>()) as u32);

    Self {
      vao,
      vbo,
      transform: Transform::new(),
      vertex_count,
    }
  }

  pub fn draw(&self, program: &Program) {
    self.vao.bind();
    self.vbo.bind();

    self.transform.send_to_program(program);

    unsafe {
      gl::LineWidth(3.0);
      gl::DrawArrays(gl::LINES, 0, self.vertex_count);
      gl::LineWidth(1.0);
    }
  }
}
