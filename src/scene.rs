use nalgebra_glm as glm;

use crate::input::Command;
use crate::asset::{load_gltf, MeshData};
use glm::rotation;

pub struct Scene {
    pub camera: glm::Mat4,
    pub mesh: Mesh,
}

pub struct Mesh {
    pub translation: glm::Vec3,
    pub scale: glm::Vec3,
    pub rotation: glm::Vec3,
    pub data: MeshData,
}

impl Mesh {
    pub fn model_mat(&self) -> glm::Mat4 {

        let translation = glm::translation(&self.translation);
        let rotation = glm::inverse(&glm::look_at(
            &self.translation,
            &(self.translation + glm::vec3(-1.0,0.0,1.0)),
            &glm::vec3(0.0,1.0,0.0)
        ));
        let scale = glm::scaling(&self.scale);

        translation * rotation * scale
    }

    pub fn update_model_position(&mut self, command: Command) {

        match command {
            Command::MoveLeft => {
                self.translation = self.translation + glm::vec3(-0.1, 0.0, 0.0);
            },
            Command::MoveRight => {
                self.translation = self.translation + glm::vec3(0.1, 0.0, 0.0);
            },
            _ => (),

        }
    }
}

impl Scene {

    pub fn cube() -> Scene {

        let asset_folder = "assets";
        let gltf = load_gltf(asset_folder, "untitled.gltf").expect("failed to load gltf");
        let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

        let view = glm::look_at(
            &glm::vec3(0.0,2.0,6.0), // Camera is at (4,3,3), in World Space
            &glm::vec3(0.0,0.0,0.0), // and looks at the origin
            &glm::vec3(0.0,1.0,0.0)  // Head is up (set to 0,-1,0 to look upside-down)
        );

        let camera = glm::inverse(&view);

        let translation = glm::vec3(0.0, 0.0, 0.0);
        let scale = glm::vec3(2.0, 1.0, 1.0);
        let rotation = glm::vec3(0.0, 0.0, 0.0);

        let mesh = Mesh {
            translation,
            scale,
            rotation,
            data: mesh_data,
        };

        Scene {
            camera,
            mesh
        }
    }

    pub fn cat() -> Scene {

        let asset_folder = "assets";
        let gltf = load_gltf(asset_folder, "cat.gltf").expect("failed to load gltf");
        let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

        let view = glm::look_at(
            &glm::vec3(0.0,200.0,1000.0), // Camera is at (4,3,3), in World Space
            &glm::vec3(0.0,0.0,0.0), // and looks at the origin
            &glm::vec3(0.0,1.0,0.0)  // Head is up (set to 0,-1,0 to look upside-down)
        );

        let camera = glm::inverse(&view);

        let translation = glm::vec3(0.0, 0.0, 0.0);
        let scale = glm::vec3(2.0, 2.0, 2.0);
        let rotation = glm::vec3(0.0, 0.0, 0.0);

        let mesh = Mesh {
            translation,
            scale,
            rotation,
            data: mesh_data,
        };

        Scene {
            camera,
            mesh
        }
    }

    pub fn camera_data(&self) -> Vec<f32> {

        let view_vec: Vec<f32> = self.camera.data.to_vec();

        let mut data = view_vec.clone();

        let model = self.mesh.model_mat();

        let mut model_vec: Vec<f32> = model.as_slice().to_vec();

        data.append(&mut model_vec);

        data

    }
}