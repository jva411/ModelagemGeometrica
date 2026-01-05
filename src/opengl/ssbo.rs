#[allow(dead_code)]
pub struct SSBO {
  pub id: u32,
}

#[allow(dead_code)]
impl SSBO {
  pub fn new() -> SSBO {
    let mut id = 0;
    unsafe { gl::GenBuffers(1, &mut id); }

    SSBO {
      id,
    }
  }

  pub fn bind(&self) {
    unsafe { gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id); }
  }

  pub fn link(&self, index: u32) {
    unsafe { gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, index, self.id); }
  }

  pub fn send_data(&self, data: &[u8]) {
    unsafe {
      gl::BufferData(
        gl::SHADER_STORAGE_BUFFER,
        data.len() as isize,
        data.as_ptr().cast(),
        gl::STATIC_DRAW,
      );
    }
  }
}
