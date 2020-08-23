use crate::{Material, Mesh, ShaderProgram, TCamera, TLight};
use cgmath::{Matrix4, Vector3};
use std::sync::Arc;

pub struct Renderer {
    shader: ShaderProgram,
    default_material: Arc<Material>,
}

impl Renderer {
    pub fn new(shader: ShaderProgram) -> Self {
        Renderer {
            shader,
            default_material: Arc::new(Material::new(Vector3::new(1.0, 0.0, 1.0), None)),
        }
    }
    pub fn clear(&self, r: f32, g: f32, b: f32) {
        unsafe {
            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn draw_mesh<A: TCamera, B: TLight>(
        &mut self,
        matrix: &Matrix4<f32>,
        mesh: &Mesh,
        camera: &A,
        light: &B,
        draw_type: u32,
    ) {
        for submesh in &mesh.submeshes {
            let material;
            if mesh.materials.is_empty() {
                material = Arc::clone(&self.default_material);
            } else {
                match submesh.material_index {
                    Some(i) => material = Arc::clone(&mesh.materials[i]),
                    None => material = Arc::clone(&self.default_material),
                }
            }
            self.shader.bind();
            material.send_to_shader(&mut self.shader);

            self.shader
                .set_uniform_matrix4(String::from("u_model"), &matrix);
            self.shader
                .set_uniform_matrix4(String::from("u_view"), &camera.matrix());
            self.shader
                .set_uniform_matrix4(String::from("u_projection"), &camera.projection_matrix());

            self.shader
                .set_uniform_vector3(String::from("u_light_direction"), &light.direction());
            self.shader
                .set_uniform_vector3(String::from("u_light_color"), &light.color());
            self.shader.set_uniform_float(
                String::from("u_light_ambient_strength"),
                light.ambient_strength(),
            );
            self.shader
                .set_uniform_float(String::from("u_light_power"), light.power());

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
