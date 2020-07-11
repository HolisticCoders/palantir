use crate::render_gl::{IndexBuffer, Vertex, VertexArray, VertexBuffer, VertexBufferLayout};

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
        mesh.layout.push::<f32>(3);

        mesh.vertex_array
            .add_buffer(&mesh.vertex_buffer, &mesh.layout);

        mesh
    }

    pub fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
    pub fn vertex_array(&self) -> &VertexArray {
        &self.vertex_array
    }
}
