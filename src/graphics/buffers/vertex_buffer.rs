use crate::graphics::Vertex;

pub struct VertexBuffer {
    id: u32,
    gl: gl::Gl,
}

impl VertexBuffer {
    pub fn new(gl: &gl::Gl, vertices: Vec<Vertex>) -> Self {
        let mut buffer = VertexBuffer {
            id: 0,
            gl: gl.clone(),
        };
        unsafe {
            gl.GenBuffers(1, &mut buffer.id);
            gl.BindBuffer(gl::ARRAY_BUFFER, buffer.id);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
        buffer
    }
    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(gl::ARRAY_BUFFER, self.id) }
    }
    pub fn unbind(&self) {
        unsafe { self.gl.BindBuffer(gl::ARRAY_BUFFER, 0) }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &self.id);
        }
    }
}

pub struct VertexBufferElement {
    pub gl_type: u32,
    pub gl_size: u32,
    pub count: u32,
    pub normalized: u8,
}
pub struct VertexBufferLayout {
    pub stride: u32,
    pub elements: Vec<VertexBufferElement>,
}

impl VertexBufferLayout {
    pub fn new() -> Self {
        VertexBufferLayout {
            stride: 0,
            elements: vec![],
        }
    }

    pub fn push<T: GLType>(&mut self, count: u32) {
        self.elements.push(VertexBufferElement {
            count,
            gl_type: T::gl_type(),
            gl_size: T::gl_size(),
            normalized: T::normalized(),
        });
        self.stride += count * T::gl_size()
    }
}

pub trait GLType {
    fn gl_size() -> u32;
    fn gl_type() -> u32;
    fn normalized() -> u8;
}

impl GLType for f32 {
    fn gl_size() -> u32 {
        4
    }
    fn gl_type() -> u32 {
        gl::FLOAT
    }
    fn normalized() -> u8 {
        gl::FALSE
    }
}
impl GLType for u32 {
    fn gl_size() -> u32 {
        4
    }
    fn gl_type() -> u32 {
        gl::UNSIGNED_INT
    }
    fn normalized() -> u8 {
        gl::FALSE
    }
}
impl GLType for u8 {
    fn gl_size() -> u32 {
        1
    }
    fn gl_type() -> u32 {
        gl::UNSIGNED_BYTE
    }
    fn normalized() -> u8 {
        gl::TRUE
    }
}
