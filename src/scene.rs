pub mod camera;
pub mod mesh;
pub mod light;

use nalgebra_glm as glm;

use crate::asset::{load_gltf, MeshData};
use camera::Camera;
use mesh::StaticMesh;
use crate::scene::mesh::MeshInstance;
use crate::scene::light::PointLight;

#[derive(Debug)]
pub struct MeshView {
    pub instance_id: u32,
    pub start: u32,
    pub length: u32,
}

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<PointLight>,
    pub meshes: Vec<StaticMesh>,
    pub mesh_instances: Vec<MeshInstance>,
}

impl Scene {
    pub fn total_unique_vertices(&self) -> usize {
        self.meshes.iter()
            .map(|mesh|{
                mesh.vertices.len()
            })
            .sum()
    }

    pub fn total_unique_indices(&self) -> usize {
        self.meshes.iter()
            .map(|mesh|{
                mesh.indices.len()
            })
            .sum()
    }

    pub fn total_indices(&self) -> usize {
        self.mesh_instances.iter()
            .map(|instance|{
                self.meshes[instance.mesh_id].indices.len()
            })
            .sum()
    }

    pub fn index_data(&self) -> Vec<u32> {
        self.meshes.iter()
            .map(|mesh|{
                mesh.indices.clone()
            })
            .flatten()
            .collect()

    }

    pub fn vertex_data(&self) -> Vec<f32> {
        self.meshes.iter()
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
                length: self.meshes[instance.mesh_id].indices.len() as u32
            });
            id += 1;
            start += self.meshes[instance.mesh_id].indices.len() as u32;
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

    pub fn multiple_boxes() -> Self {
        let asset_folder = "assets";
        let gltf = load_gltf(asset_folder, "untitled.gltf")
            .expect("failed to load gltf");

        let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

        let cube_mesh = StaticMesh {
            id: 0,
            indices: mesh_data.indices.clone(),
            vertices: mesh_data.vertices.clone(),
        };

        let floor = MeshInstance {
            position: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(10.0, 1.0, 10.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id,
        };

        let ceiling = MeshInstance {
            position: glm::vec3(0.0, 5.0, 0.0),
            scale: glm::vec3(10.0, 1.0, 10.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id,
        };

        let left_wall = MeshInstance {
            position: glm::vec3(-3.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 10.0, 10.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id,
        };

        let right_wall = MeshInstance {
            position: glm::vec3(5.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 10.0, 10.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id,
        };

        let back_wall = MeshInstance {
            position: glm::vec3(0.0, 0.0, -5.0),
            scale: glm::vec3(10.0, 10.0, 1.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id,
        };

        let cube1 = MeshInstance {
            position: glm::vec3(-1.0, 1.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id.clone(),
        };

        let cube2 = MeshInstance {
            position: glm::vec3(1.0, 1.1, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id,
        };

        let cube3 = MeshInstance {
            position: glm::vec3(3.0, 1.0, 0.0),
            scale: glm::vec3(1.0, 3.0, 3.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id,
        };

        let camera = Camera::new(
            glm::vec3(0.0, 4.0, 8.0),
            glm::vec3( 0.0, 2.0, -7.0)
        );

        let light = PointLight{
            position: glm::vec4(1.5, 4.0, 4.0, 0.0),
            intensity: 20.0,
        };

        Scene {
            camera,
            lights: vec![light],
            meshes: vec![cube_mesh],
            mesh_instances: vec![
                cube1,
                cube2,
                cube3,
                floor,
//                ceiling,
                left_wall,
//                right_wall,
//                back_wall,
            ]
        }
    }

    pub fn two_cubes() -> Self {
        let asset_folder = "assets";
        let gltf = load_gltf(asset_folder, "untitled.gltf")
                .expect("failed to load gltf");

        let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

        let cube_mesh = StaticMesh {
            id: 0,
            indices: mesh_data.indices.clone(),
            vertices: mesh_data.vertices.clone(),
        };

        let cube1 = MeshInstance {
            position: glm::vec3(-1.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id.clone(),
        };

        let cube2 = MeshInstance {
            position: glm::vec3(1.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id,
        };

        let camera = Camera::new(
            glm::vec3(0.0, 2.0, 8.0),
            glm::vec3( 0.0, 0.0, 0.0)
        );

        let light = PointLight{
            position: glm::vec4(3.0, 5.0, 2.0, 0.0),
            intensity: 20.0,
        };

        Scene {
            camera,
            lights: vec![light],
            meshes: vec![cube_mesh],
            mesh_instances: vec![cube1, cube2],
        }
    }

    pub fn occluded_cubes() -> Self {
        let asset_folder = "assets";
        let gltf = load_gltf(asset_folder, "untitled.gltf")
            .expect("failed to load gltf");

        let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

        let cube_mesh = StaticMesh {
            id: 0,
            indices: mesh_data.indices.clone(),
            vertices: mesh_data.vertices.clone(),
        };

        let cube1 = MeshInstance {
            position: glm::vec3(1.0, 0.0, -1.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id.clone(),
        };

        let cube2 = MeshInstance {
            position: glm::vec3(1.0, 0.0, 0.5),
            scale: glm::vec3(1.0, 1.0, 1.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            mesh_id: cube_mesh.id,
        };

        let camera = Camera::new(
            glm::vec3(0.0, 2.0, 8.0),
            glm::vec3( 0.0, 0.0, 0.0)
        );

        let light = PointLight{
            position: glm::vec4(3.0, 5.0, 2.0, 0.0),
            intensity: 20.0,
        };

        Scene {
            camera,
            lights: vec![light],
            meshes: vec![cube_mesh],
            mesh_instances: vec![cube1, cube2],
        }
    }
//
//    pub fn cube() -> Self {
//        let asset_folder = "assets";
//        let gltf = load_gltf(asset_folder, "untitled.gltf")
//            .expect("failed to load gltf");
//        let mesh_data = MeshData::from_gltf(&gltf, asset_folder);
//
//        let cube_mesh = StaticMesh {
//            id: 0,
//            indices: mesh_data.indices.clone(),
//            vertices: mesh_data.vertices.clone(),
//        };
//
//        let cube1 = MeshInstance {
//            position: glm::vec3(1.0, 0.0, 0.0),
//            scale: glm::vec3(1.0, 1.0, 1.0),
//            rotation: glm::vec3(0.0, 0.0, 1.0),
//            mesh_id: cube_mesh.id,
//        };
//
//        let camera = Camera::new(
//            glm::vec3(0.0, 2.0, 10.0),
//            glm::vec3( 0.0, 0.0, 0.0)
//        );
//
//        let light = PointLight{
//            position: glm::vec3(0.0, 5.0, 3.0),
//            intensity: 20.0,
//        };
//
//        let scene = Scene {
//            camera,
//            lights: vec![light],
//            meshes: vec![cube_mesh],
//            mesh_instances: vec![cube1],
//
//        };
//
//        println!("mesh instance views: {:?}", scene.instance_views());
//        println!("total indices: {:?}", scene.total_indices());
//
//        scene
//    }

//    pub fn cat() -> Self {
//        let asset_folder = "assets";
//        let gltf = load_gltf(asset_folder, "cat.gltf").expect("failed to load gltf");
//        let mesh_data = MeshData::from_gltf(&gltf, asset_folder);
//
//        let mesh = StaticMesh {
//            position: glm::vec3(0.0, 0.0, 0.0),
//            scale: glm::vec3(1.0, 1.0, 1.0),
//            rotation: glm::vec3(0.0, 0.0, 1.0),
//            indices: mesh_data.indices.clone(),
//            vertices: mesh_data.vertices.clone(),
//            handle: glm::vec2(0, 36),
//        };
//
//        let camera = Camera::new(
//            glm::vec3(0.0,200.0,1000.0),
//            glm::vec3(0.0,0.0,0.0),
//        );
//
//        Scene {
//            camera,
//            meshes: vec![mesh]
//        }
//    }

}