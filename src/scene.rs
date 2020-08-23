use crate::resources::Resources;
use crate::{Camera, Light};
use cgmath::{Matrix4, Point3, Vector2, Vector3};
use palantir_lib::{Material, Mesh, SubMesh, Texture, Vertex};
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use tobj;

pub struct Scene {
    camera: Camera,
    light: Light,
}

impl Scene {
    pub fn new() -> Self {
        let camera = Camera::from_focal_length(50.0, 36.0, 0.01, 1000.0, 1.0);
        let mut light = Light::new();
        light.set_matrix(Matrix4::look_at_dir(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, -1.0, 1.0),
            Vector3::unit_y(),
        ));

        Scene { camera, light }
    }
}

// Light stuff
impl Scene {
    pub fn light(&self) -> &Light {
        &self.light
    }
    pub fn light_mut(&mut self) -> &mut Light {
        &mut self.light
    }
}

// Camera stuff
impl Scene {
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

// Meshes stuff
impl Scene {
    pub fn load_obj(&mut self, path: PathBuf, res: &Resources) -> Result<Mesh, Box<dyn Error>> {
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
                let uv =
                    Vector2::<f32>::new(obj_mesh.texcoords[i * 2], obj_mesh.texcoords[i * 2 + 1]);
                vertices.push(Vertex {
                    position,
                    normal,
                    uv,
                });
            }
            let submesh = SubMesh::new(vertices, indices, obj_mesh.material_id);
            submeshes.push(submesh);
        }
        let mut mesh = Mesh {
            submeshes,
            materials: Vec::new(),
        };

        for material in materials {
            let texture: Option<Texture>;
            let texture_path = material.diffuse_texture;
            if texture_path != "" {
                let full_path = res.resource_name_to_path(&texture_path.replace("res://", ""));
                texture = Some(Texture::new(full_path));
            } else {
                texture = None;
            };

            mesh.materials.push(Arc::new(Material::new(
                Vector3::<f32>::new(1.0, 1.0, 1.0),
                texture,
            )));
        }
        Ok(mesh)
    }
}
