use crate::render_gl::{IndexBuffer, VertexArray, VertexBuffer, VertexBufferLayout};
use nalgebra::Vector3;

// TODO: Use trait to automatically generate layout
pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
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

        mesh.vertex_array.add_buffer(&mesh.vertex_buffer, &mesh.layout);

        mesh.layout.push::<f32>(3);
        mesh.layout.push::<f32>(3);

        mesh
    }

    pub fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
    pub fn vertex_array(&self) -> &VertexArray {
        &self.vertex_array
    }
}