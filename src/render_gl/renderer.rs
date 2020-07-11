use crate::render_gl::{Mesh, Program};
pub struct Renderer {}

impl Renderer {
    pub fn clear(gl: &gl::Gl, r: f32, g: f32, b: f32) {
        unsafe {
            gl.ClearColor(r, g, b, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn draw(gl: &gl::Gl, mesh: &Mesh, shader: &Program, draw_type: u32) {
        shader.bind();
        mesh.vertex_array().bind();
        mesh.index_buffer().bind();
        unsafe {
            gl.DrawElements(
                draw_type,
                mesh.index_buffer().count as i32,
                gl::UNSIGNED_INT,
                std::ptr::null_mut(),
            )
        }
    }
}
