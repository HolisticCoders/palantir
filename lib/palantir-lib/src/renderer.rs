use crate::{Material, Mesh, ShaderProgram, TCamera, TLight};
use cgmath::Vector3;
use std::cell::RefCell;

pub struct Renderer {
    shader: RefCell<ShaderProgram>,
    default_material: RefCell<Material>,
}

impl Renderer {
    pub fn new(shader: ShaderProgram) -> Self {
        Renderer {
            shader: RefCell::new(shader),
            default_material: RefCell::new(Material::new(Vector3::new(1.0, 0.0, 1.0), None)),
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
        let mut shader = self.shader.borrow_mut();
        for submesh in &mesh.submeshes {
            let material;
            if mesh.materials.is_empty() {
                material = self.default_material.borrow_mut();
            } else {
                match submesh.material_index {
                    Some(i) => material = mesh.materials[i].borrow_mut(),
                    None => material = self.default_material.borrow_mut(),
                }
            }
            shader.bind();
            material.send_to_shader(&mut shader);

            shader.set_uniform_matrix4(String::from("u_model"), &mesh.matrix);
            shader.set_uniform_matrix4(String::from("u_view"), &camera.matrix());
            shader.set_uniform_matrix4(String::from("u_projection"), &camera.projection_matrix());

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
