use crate::{IndexBuffer, Material, Vertex, VertexArray, VertexBuffer, VertexBufferLayout};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct SubMesh {
    pub material_index: Option<usize>,
    vertex_buffer: VertexBuffer,
    layout: VertexBufferLayout,
    index_buffer: IndexBuffer,
    vertex_array: VertexArray,
}

impl SubMesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, material_index: Option<usize>) -> Self {
        let mut submesh = SubMesh {
            material_index,
            vertex_buffer: VertexBuffer::new(vertices),
            layout: VertexBufferLayout::new(),
            index_buffer: IndexBuffer::new(indices),
            vertex_array: VertexArray::new(),
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

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub submeshes: Vec<SubMesh>,
    pub materials: Vec<Arc<Material>>,
}

impl Mesh {
    pub fn new(submeshes: Vec<SubMesh>) -> Self {
        Mesh {
            submeshes,
            materials: Vec::new(),
        }
    }
}
