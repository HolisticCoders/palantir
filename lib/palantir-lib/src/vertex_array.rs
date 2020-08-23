use crate::vertex_buffer::{VertexBuffer, VertexBufferLayout};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexArray {
    id: u32,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut vertex_array = VertexArray { id: 0 };
        unsafe { gl::GenVertexArrays(1, &mut vertex_array.id) }
        vertex_array
    }
    pub fn add_buffer(&self, vertex_buffer: &VertexBuffer, layout: &VertexBufferLayout) {
        self.bind();
        vertex_buffer.bind();
        let mut offset: u32 = 0;
        for (i, element) in layout.elements.iter().enumerate() {
            let i = i as u32; //FIXME: should be defined in the for statement
            unsafe {
                gl::EnableVertexAttribArray(i);
                gl::VertexAttribPointer(
                    i,
                    element.count as i32,
                    element.gl_type,
                    element.normalized,
                    layout.stride as i32,
                    offset as *const gl::types::GLvoid,
                );
            }
            offset += element.count * element.gl_size;
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
}
