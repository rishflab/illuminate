pub mod camera;
pub mod mesh;
pub mod light;

use nalgebra_glm as glm;

use crate::asset::{load_gltf, MeshData};
use crate::scene::camera::Camera;
use crate::scene::mesh::{StaticMeshData};
use crate::scene::light::PointLight;

pub use mesh::MeshInstance;
use glm::{vec3, quat_identity};

#[derive(Debug)]
pub struct MeshView {
    pub instance_id: u32,
    pub start: u32,
    pub length: u32,
}

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<PointLight>,
    pub mesh_data: Vec<StaticMeshData>,
    pub mesh_instances: Vec<MeshInstance>,
}

impl Scene {
    pub fn total_unique_vertices(&self) -> usize {
        self.mesh_data.iter()
            .map(|mesh|{
                mesh.vertices.len()
            })
            .sum()
    }

    pub fn total_unique_indices(&self) -> usize {
        self.mesh_data.iter()
            .map(|mesh|{
                mesh.indices.len()
            })
            .sum()
    }

    pub fn total_indices(&self) -> usize {
        self.mesh_instances.iter()
            .map(|instance|{
                self.mesh_data[instance.mesh_id].indices.len()
            })
            .sum()
    }

    pub fn index_data(&self) -> Vec<u32> {
        self.mesh_data.iter()
            .map(|mesh|{
                mesh.indices.clone()
            })
            .flatten()
            .collect()

    }

    pub fn vertex_data(&self) -> Vec<f32> {
        self.mesh_data.iter()
            .map(|mesh|{
                mesh.vertices.clone()
            })
            .flatten()
            .map(|vert|{
                vert.data.to_vec()
            })
            .flatten()
            .collect()
    }

    pub fn instance_views(&self) -> Vec<MeshView> {
        let mut views = Vec::new();
        let mut start = 0;
        let mut id = 0;
        for instance in self.mesh_instances.iter(){
            views.push(MeshView{
                instance_id: id as u32,
                start: start,
                length: self.mesh_data[instance.mesh_id].indices.len() as u32
            });
            id += 1;
            start += self.mesh_data[instance.mesh_id].indices.len() as u32;
        }
        views
    }

    pub fn model_matrices(&self) -> Vec<f32> {
        self.mesh_instances.iter()
            .map(|instance|{
                instance.model_matrix().data.to_vec()
            })
            .flatten()
            .collect()
    }

    pub fn light_data(&self) -> Vec<f32> {
        self.lights.iter()
            .map(|light|{
                light.data()
            })
            .flatten()
            .collect()
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        self.camera.view_matrix
    }
}

impl Default for Scene {
    fn default() -> Self {

        let asset_folder = "assets";
        let gltf = load_gltf(asset_folder, "untitled.gltf")
                .expect("failed to load gltf");

        let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

        let cube_mesh = StaticMeshData {
            id: 0,
            indices: mesh_data.indices.clone(),
            vertices: mesh_data.vertices.clone(),
        };

        Scene {
            camera: Camera::new(vec3(0.0, 0.0, 0.0), quat_identity()),
            lights: vec![],
            mesh_data: vec![cube_mesh],
            mesh_instances: Vec::new()
        }
    }
}