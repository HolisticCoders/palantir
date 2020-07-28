mod camera;
mod light;
mod primitives;

pub use self::camera::*;
pub use self::light::*;
pub use self::primitives::*;

// pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Self, Box<dyn Error>> {
//     let path = res.resource_name_to_path(name);

use cgmath::{Vector2, Vector3};
use palantir_lib::{Mesh, SubMesh, Texture, Vertex};
use std::error::Error;
use std::path::{Path, PathBuf};
use tobj;

pub fn load_obj(path: PathBuf) -> Result<Mesh, Box<dyn Error>> {
    let (models, materials) = tobj::load_obj(path, true)?;

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
            let uv = Vector2::<f32>::new(obj_mesh.texcoords[i * 2], obj_mesh.texcoords[i * 2 + 1]);
            vertices.push(Vertex {
                position,
                normal,
                uv,
            });
        }
        let submesh = SubMesh::new(vertices, indices, obj_mesh.material_id);
        submeshes.push(submesh);
    }
    let mut mesh = Mesh::new(submeshes);

    // for material in materials {
    //     let mut shader = ShaderProgram::from_res(&res, "shaders/lambert").unwrap();
    //     shader.bind();
    //     shader.set_uniform_vector3("u_color".to_string(), &Vector3::<f32>::new(1.0, 1.0, 1.0));

    //     let texture_path = material.diffuse_texture;
    //     if texture_path != "" {
    //         let full_path = res.resource_name_to_path(&texture_path.replace("res://", ""));
    //         let texture = Texture::new(full_path);
    //         shader.set_texture(texture);
    //     }

    //     mesh.shaders.push(RefCell::new(shader));
    // }
    Ok(mesh)
}
