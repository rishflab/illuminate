use nalgebra_glm as glm;

use crate::input::Command;
use crate::asset::{load_gltf, MeshData};
use glm::rotation;

pub struct Scene {
    pub camera: Camera,
    pub mesh: Mesh,
}

pub struct Camera {
    pub position: glm::Vec3,
    pub look_at: glm::Vec3,
    look_up: glm::Vec3,
}

impl Camera {

    pub fn new(position: glm::Vec3, look_at: glm::Vec3) -> Self {
        Camera{
            position,
            look_at,
            look_up: glm::vec3(0.0, 1.0, 0.0)
        }
    }

    pub fn update_position(&mut self, command: Command) {

        match command {
            Command::MoveLeft => {
                self.position = self.position + glm::vec3(-0.1, 0.0, 0.0);
                self.look_at = self.look_at + glm::vec3(-0.1, 0.0, 0.0);
            },
            Command::MoveRight => {
                self.position = self.position + glm::vec3(0.1, 0.0, 0.0);
                self.look_at = self.look_at + glm::vec3(0.1, 0.0, 0.0);
            },
            _ => (),

        }
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        glm::inverse(
            &glm::look_at(
                &self.position,
                &self.look_at,
                &self.look_up
            )
        )
    }
}

pub struct Mesh {
    pub position: glm::Vec3,
    pub scale: glm::Vec3,
    pub rotation: glm::Vec3,
    pub data: MeshData,
}

impl Mesh {
    pub fn model_matrix(&self) -> glm::Mat4 {
        let translation = glm::translation(&self.position);
        let rotation = glm::inverse(&glm::look_at(
            &self.position,
            &(self.position + self.rotation),
            &glm::vec3(0.0,1.0,0.0)
        ));
        let scale = glm::scaling(&self.scale);
        translation * rotation * scale
    }

    pub fn update_position(&mut self, command: Command) {
        match command {
            Command::MoveLeft => {
                self.position = self.position + glm::vec3(-0.1, 0.0, 0.0);
            },
            Command::MoveRight => {
                self.position = self.position + glm::vec3(0.1, 0.0, 0.0);
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

        let mesh = Mesh {
            position: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
            data: mesh_data,
        };

        let camera = Camera::new(
            glm::vec3(0.0, 2.0, 6.0),
            glm::vec3( 0.0, 0.0, 0.0)
        );

        Scene {
            camera,
            mesh
        }
    }

    pub fn cat() -> Scene {
        let asset_folder = "assets";
        let gltf = load_gltf(asset_folder, "cat.gltf").expect("failed to load gltf");
        let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

        let mesh = Mesh {
            position: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            data: mesh_data,
        };

        let camera = Camera::new(
            glm::vec3(0.0,200.0,1000.0),
            glm::vec3(0.0,0.0,0.0),
        );

        Scene {
            camera,
            mesh
        }
    }

}