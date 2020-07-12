use crate::graphics::{IndexBuffer, Vertex, VertexArray, VertexBuffer, VertexBufferLayout};
use crate::resources::{self, Resources};
use cgmath::Vector3;
use obj::load_obj;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub enum MeshError {
    ResourceLoad {
        name: String,
        inner: resources::ResourceError,
    },
}

pub struct Mesh {
    vertex_buffer: VertexBuffer,
    layout: VertexBufferLayout,
    index_buffer: IndexBuffer,
    vertex_array: VertexArray,
}

impl Mesh {
    pub fn new(gl: &gl::Gl, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let mut mesh = Mesh {
            vertex_buffer: VertexBuffer::new(gl, vertices),
            index_buffer: IndexBuffer::new(gl, indices),
            vertex_array: VertexArray::new(gl),
            layout: VertexBufferLayout::new(),
        };

        mesh.layout.push::<f32>(3);
        mesh.layout.push::<f32>(3);

        mesh.vertex_array
            .add_buffer(&mesh.vertex_buffer, &mesh.layout);

        mesh
    }
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Self, Box<dyn Error>> {
        let path = res.resource_name_to_path(name);

        let input = BufReader::new(File::open(path)?);

        let obj = load_obj(input)?;

        let vertices = obj
            .vertices
            .into_iter()
            .map(|vertex: obj::Vertex| {
                let position =
                    Vector3::new(vertex.position[0], vertex.position[1], vertex.position[2]);
                let normal = Vector3::new(vertex.normal[0], vertex.normal[1], vertex.normal[2]);
                Vertex { position, normal }
            })
            .collect();

        let indices = obj
            .indices
            .into_iter()
            .map(|index: u16| index as u32)
            .collect();

        let mesh = Mesh::new(&gl, vertices, indices);

        Ok(mesh)
    }

    pub fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
    pub fn vertex_array(&self) -> &VertexArray {
        &self.vertex_array
    }
}
