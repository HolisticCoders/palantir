use crate::graphics::{Mesh, ShaderProgram};

pub struct Renderer {}

impl Renderer {
    pub fn clear(gl: &gl::Gl, r: f32, g: f32, b: f32) {
        unsafe {
            gl.ClearColor(r, g, b, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn draw(gl: &gl::Gl, mesh: &Mesh, shader: &ShaderProgram, draw_type: u32) {
        shader.bind();
        for submesh in &mesh.submeshes {
            submesh.vertex_array().bind();
            submesh.index_buffer().bind();
            unsafe {
                gl.DrawElements(
                    draw_type,
                    submesh.index_buffer().count as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null_mut(),
                )
            }
        }
    }
}
