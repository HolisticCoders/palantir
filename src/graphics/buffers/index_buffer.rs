pub struct IndexBuffer {
    pub count: u32,
    id: u32,
    gl: gl::Gl,
}

impl IndexBuffer {
    pub fn new(gl: &gl::Gl, indices: Vec<u32>) -> Self {
        let mut buffer = IndexBuffer {
            count: indices.len() as u32,
            id: 0,
            gl: gl.clone(),
        };
        unsafe {
            gl.GenBuffers(1, &mut buffer.id);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.id);
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
        buffer
    }
    pub fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &self.id);
        }
    }
}

