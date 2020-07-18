use crate::components::{Camera, Light};
use crate::graphics::{Mesh, ShaderProgram};
use crate::resources::Resources;
use cgmath::Vector3;
use std::cell::RefCell;

pub struct Renderer {
    gl: gl::Gl,
    default_shader: RefCell<ShaderProgram>,
}

impl Renderer {
    pub fn new(gl: &gl::Gl, default_shader: ShaderProgram) -> Self {
        Renderer {
            gl: gl.clone(),
            default_shader: RefCell::new(default_shader),
        }
    }
    pub fn clear(&self, r: f32, g: f32, b: f32) {
        unsafe {
            self.gl.ClearColor(r, g, b, 1.0);
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn draw(&self, mesh: &Mesh, camera: &Camera, light: &Light, draw_type: u32) {
        for submesh in &mesh.submeshes {
            let mut shader;
            if mesh.shaders.is_empty() {
                shader = self.default_shader.borrow_mut();
            } else {
                shader = mesh.shaders[submesh.shader_index].borrow_mut();
            }
            shader.bind();

            shader.set_uniform_matrix4(String::from("model"), &mesh.matrix);
            shader.set_uniform_matrix4(String::from("view"), &camera.view_matrix());
            shader.set_uniform_matrix4(String::from("projection"), &camera.projection_matrix());

            // TODO: only set these on supported shaders
            shader.set_uniform_vector3(String::from("light_direction"), &light.direction());
            shader.set_uniform_vector3(String::from("light_color"), &light.color);
            shader.set_uniform_float(
                String::from("light_ambient_strength"),
                light.ambient_strength,
            );
            shader.set_uniform_float(String::from("light_power"), light.power);

            submesh.vertex_array().bind();
            submesh.index_buffer().bind();
            unsafe {
                self.gl.DrawElements(
                    draw_type,
                    submesh.index_buffer().count as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null_mut(),
                )
            }
        }
    }
}
