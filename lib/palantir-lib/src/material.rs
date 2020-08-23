use crate::{ShaderProgram, Texture};
use cgmath::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    diffuse: Vector3<f32>,
    diffuse_texture: Option<Texture>,
}

impl Material {
    pub fn new(diffuse: Vector3<f32>, diffuse_texture: Option<Texture>) -> Self {
        Material {
            diffuse,
            diffuse_texture,
        }
    }
    pub fn send_to_shader(&self, shader: &mut ShaderProgram) {
        shader.set_uniform_vector3(String::from("material.diffuse"), &self.diffuse);
        match &self.diffuse_texture {
            Some(texture) => {
                texture.bind(0);
                shader.set_uniform_bool(String::from("material.use_diffuse_texture"), true)
            }
            None => shader.set_uniform_bool(String::from("material.use_diffuse_texture"), false),
        };
    }
}
