pub struct VBO {
  pub id: u32,
}

impl VBO {
  pub fn new() -> Self {
    let mut id = 0;
    unsafe { gl::GenBuffers(1, &mut id); }

    return VBO { id };
  }

  pub fn bind(&self) {
    unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id); }
  }

  pub fn send_data(&self, data: &[f32]) {
    unsafe {
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (data.len() * size_of::<f32>()) as isize,
        data.as_ptr().cast(),
        gl::STATIC_DRAW,
      );
    }
  }
}
