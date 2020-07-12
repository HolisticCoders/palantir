use crate::graphics::{VertexBuffer, VertexBufferLayout};

pub struct VertexArray {
    id: u32,
    gl: gl::Gl,
}

impl VertexArray {
    pub fn new(gl: &gl::Gl) -> Self {
        let mut vertex_array = VertexArray {
            id: 0,
            gl: gl.clone(),
        };
        unsafe { gl.GenVertexArrays(1, &mut vertex_array.id) }
        vertex_array
    }
    pub fn add_buffer(&self, vertex_buffer: &VertexBuffer, layout: &VertexBufferLayout) {
        self.bind();
        vertex_buffer.bind();
        let mut offset: u32 = 0;
        for (i, element) in layout.elements.iter().enumerate() {
            let i = i as u32; //FIXME: should be defined in the for statement
            unsafe {
                self.gl.EnableVertexAttribArray(i);
                self.gl.VertexAttribPointer(
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
            self.gl.BindVertexArray(self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}
