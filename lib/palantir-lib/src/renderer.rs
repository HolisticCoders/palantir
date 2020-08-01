use crate::{Mesh, ShaderProgram, TCamera, TLight};
use std::cell::RefCell;

pub struct Renderer {
    default_shader: RefCell<ShaderProgram>,
}

impl Renderer {
    pub fn new(default_shader: ShaderProgram) -> Self {
        Renderer {
            default_shader: RefCell::new(default_shader),
        }
    }
    pub fn clear(&self, r: f32, g: f32, b: f32) {
        unsafe {
            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn draw_mesh<A: TCamera, B: TLight>(
        &self,
        mesh: &Mesh,
        camera: &A,
        light: &B,
        draw_type: u32,
    ) {
        for submesh in &mesh.submeshes {
            let mut shader;
            if mesh.shaders.is_empty() {
                shader = self.default_shader.borrow_mut();
            } else {
                match submesh.shader_index {
                    Some(i) => shader = mesh.shaders[i].borrow_mut(),
                    None => shader = self.default_shader.borrow_mut(),
                }
            }
            shader.bind();

            shader.set_uniform_matrix4(String::from("u_model"), &mesh.matrix);
            shader.set_uniform_matrix4(String::from("u_view"), &camera.matrix());
            shader.set_uniform_matrix4(String::from("u_projection"), &camera.projection_matrix());

            // TODO: only set these on supported shaders
            shader.set_uniform_vector3(String::from("u_light_direction"), &light.direction());
            shader.set_uniform_vector3(String::from("u_light_color"), &light.color());
            shader.set_uniform_float(
                String::from("u_light_ambient_strength"),
                light.ambient_strength(),
            );
            shader.set_uniform_float(String::from("u_light_power"), light.power());

            submesh.vertex_array().bind();
            submesh.index_buffer().bind();
            unsafe {
                gl::DrawElements(
                    draw_type,
                    submesh.index_buffer().count as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null_mut(),
                )
            }
        }
    }
}
