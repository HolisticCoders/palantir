use crate::components::{Camera, Light};
use crate::resources::Resources;
use cgmath::{Matrix4, Point3, Vector2, Vector3};
use palantir_lib::{Material, Mesh, SubMesh, Texture, Vertex};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use tobj;
use uuid::Uuid;

pub struct Node {
    uuid: Uuid,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
    pub name: String,
}

impl Node {
    fn new(uuid: Uuid, name: String, parent: Option<Uuid>) -> Self {
        Node {
            uuid,
            name,
            parent,
            children: Vec::new(),
        }
    }
    pub fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }
}
pub struct Scene {
    nodes: HashMap<Uuid, Node>,
    root_node: Uuid,
    meshes: Vec<Mesh>,
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

        let mut nodes = HashMap::new();
        let root_id = Uuid::new_v4();
        let root_node = Node::new(root_id, "Root".to_string(), None);
        nodes.insert(root_id, root_node);

        Scene {
            nodes,
            root_node: root_id,
            meshes: Vec::new(),
            camera,
            light,
        }
    }
}
// Nodes stuff
impl Scene {
    pub fn new_node(&mut self, name: String, parent: Option<Uuid>) -> Uuid {
        let mut parent = parent; // shadow parent into a mutable version of itself
        if let None = parent {
            parent = Some(self.root_node)
        }

        let new_node_id = Uuid::new_v4();

        if let Some(id) = parent {
            let parent_node = self.node_mut(&id);
            if let Some(node) = parent_node {
                node.children.push(new_node_id)
            }
        }

        self.nodes
            .insert(new_node_id, Node::new(new_node_id.clone(), name, parent));
        new_node_id
    }
    pub fn nodes(&self) -> &HashMap<Uuid, Node> {
        &self.nodes
    }

    pub fn root_node(&self) -> &Node {
        self.nodes.get(&self.root_node).unwrap()
    }
    pub fn node(&self, uuid: &Uuid) -> Option<&Node> {
        self.nodes.get(uuid)
    }
    pub fn node_mut(&mut self, uuid: &Uuid) -> Option<&mut Node> {
        self.nodes.get_mut(uuid)
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
    pub fn meshes(&self) -> &Vec<Mesh> {
        &self.meshes
    }
    fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
    pub fn load_obj(&mut self, path: PathBuf, res: &Resources) -> Result<(), Box<dyn Error>> {
        let name = String::from(path.file_name().unwrap().to_str().unwrap());
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
        let mut mesh = Mesh::new(submeshes);
        mesh.name = name;

        for material in materials {
            let texture: Option<Texture>;
            let texture_path = material.diffuse_texture;
            if texture_path != "" {
                let full_path = res.resource_name_to_path(&texture_path.replace("res://", ""));
                texture = Some(Texture::new(full_path));
            } else {
                texture = None;
            };

            mesh.materials.push(RefCell::new(Material::new(
                Vector3::<f32>::new(1.0, 1.0, 1.0),
                texture,
            )));
        }
        self.add_mesh(mesh);
        Ok(())
    }
}
