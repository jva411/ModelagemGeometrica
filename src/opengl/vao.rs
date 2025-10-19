#[allow(dead_code)]
pub struct VAO {
    pub id: u32,
}

#[allow(dead_code)]
impl VAO {
  pub fn new() -> Self {
    let mut id = 0;
    unsafe { gl::GenVertexArrays(1, &mut id); }

    return VAO { id };
  }

  pub fn bind(&self) {
    unsafe { gl::BindVertexArray(self.id); }
  }

  pub fn add_attribute(&self, index: u32, stride: u32, offset: u32) {
    unsafe {
      gl::VertexAttribPointer(
        index,
        3,
        gl::FLOAT,
        gl::FALSE,
        stride as i32,
        offset as *const _,
      );
      gl::EnableVertexAttribArray(index);
    }
  }
}
