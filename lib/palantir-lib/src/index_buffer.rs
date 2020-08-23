use gl;

#[derive(Clone, Debug, PartialEq)]
pub struct IndexBuffer {
    pub count: u32,
    id: u32,
}

impl IndexBuffer {
    pub fn new(indices: Vec<u32>) -> Self {
        let mut buffer = IndexBuffer {
            count: indices.len() as u32,
            id: 0,
        };
        unsafe {
            gl::GenBuffers(1, &mut buffer.id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.id);
            gl::BufferData(
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
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
