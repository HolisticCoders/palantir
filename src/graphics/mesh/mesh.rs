use crate::graphics::{
    IndexBuffer, ShaderProgram, Texture, Vertex, VertexArray, VertexBuffer, VertexBufferLayout,
};
use crate::resources::Resources;
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector2, Vector3};
use std::cell::RefCell;
use std::error::Error;
use tobj::load_obj;

pub struct SubMesh {
    pub shader_index: Option<usize>,
    vertex_buffer: VertexBuffer,
    layout: VertexBufferLayout,
    index_buffer: IndexBuffer,
    vertex_array: VertexArray,
}
impl SubMesh {
    pub fn new(
        gl: &gl::Gl,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        shader_index: Option<usize>,
    ) -> Self {
        let mut submesh = SubMesh {
            shader_index,
            vertex_buffer: VertexBuffer::new(gl, vertices),
            layout: VertexBufferLayout::new(),
            index_buffer: IndexBuffer::new(gl, indices),
            vertex_array: VertexArray::new(gl),
        };

        submesh.layout.push::<f32>(3); // Position
        submesh.layout.push::<f32>(3); // Normal
        submesh.layout.push::<f32>(2); // UV coordinates

        submesh
            .vertex_array
            .add_buffer(&submesh.vertex_buffer, &submesh.layout);
        submesh
    }
    pub fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
    pub fn vertex_array(&self) -> &VertexArray {
        &self.vertex_array
    }
}

pub struct Mesh {
    pub submeshes: Vec<SubMesh>,
    pub matrix: Matrix4<f32>,
    pub shaders: Vec<RefCell<ShaderProgram>>,
}

impl Mesh {
    pub fn new(submeshes: Vec<SubMesh>) -> Self {
        Mesh {
            submeshes,
            matrix: Matrix4::identity(),
            shaders: Vec::new(),
        }
    }
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Self, Box<dyn Error>> {
        let path = res.resource_name_to_path(name);

        let (models, materials) = load_obj(path, true)?;

        let mut submeshes = Vec::<SubMesh>::new();
        for model in models {
            let obj_mesh = &model.mesh;
            let mut vertices: Vec<Vertex> = Vec::new();
            let indices = obj_mesh.indices.clone();
            for i in 0..obj_mesh.positions.len() / 3 {
                let position = Vector3::<f32>::new(
                    obj_mesh.positions[i * 3],
                    obj_mesh.positions[i * 3 + 1],
                    obj_mesh.positions[i * 3 + 2],
                );
                let normal = Vector3::<f32>::new(
                    obj_mesh.normals[i * 3],
                    obj_mesh.normals[i * 3 + 1],
                    obj_mesh.normals[i * 3 + 2],
                );
                let uv =
                    Vector2::<f32>::new(obj_mesh.texcoords[i * 2], obj_mesh.texcoords[i * 2 + 1]);
                vertices.push(Vertex {
                    position,
                    normal,
                    uv,
                });
            }
            let submesh = SubMesh::new(&gl, vertices, indices, obj_mesh.material_id);
            submeshes.push(submesh);
        }
        let mut mesh = Mesh::new(submeshes);

        for material in materials {
            let mut shader = ShaderProgram::from_res(&gl, &res, "shaders/lambert").unwrap();
            shader.bind();
            shader.set_uniform_vector3("u_color".to_string(), &Vector3::<f32>::new(1.0, 1.0, 1.0));

            let texture_path = material.diffuse_texture;
            if texture_path != "" {
                let full_path = res.resource_name_to_path(&texture_path.replace("res://", ""));
                let texture = Texture::new(gl, full_path);
                shader.set_texture(texture);
            }

            mesh.shaders.push(RefCell::new(shader));
        }
        Ok(mesh)
    }
}
