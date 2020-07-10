use nalgebra::Vector3;

pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
}

impl Vertex {
    pub fn vertex_attrib_pointer(gl: &gl::Gl) {
        let stride = std::mem::size_of::<Self>();

        let mut location = 0; // layout (location = 0)
        let mut offset = 0; // offset of the first component

        unsafe {
            add_vertex_attribute::<Vector3<f32>>(gl, stride, &mut location, &mut offset);
            add_vertex_attribute::<Vector3<f32>>(gl, stride, &mut location, &mut offset);
        }
    }
}

unsafe fn add_vertex_attribute<T>(gl: &gl::Gl, stride:usize, location:&mut i32, offset: &mut usize){
    gl.EnableVertexAttribArray(*location as gl::types::GLuint);
    gl.VertexAttribPointer(
        *location as gl::types::GLuint,
        3, // the number of components per generic vertex attribute
        gl::FLOAT, // data type
        gl::FALSE, // normalized (int-to-float conversion)
        stride as gl::types::GLint,
        *offset as *const gl::types::GLvoid
    );
    *location += 1;
    *offset += std::mem::size_of::<T>();
}

