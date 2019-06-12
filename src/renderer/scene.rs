use nalgebra_glm as glm;

use crate::input::Command;
use crate::asset::{load_gltf, MeshData};

pub struct Scene {
    pub camera: glm::Mat4,
    pub model_pos: glm::Vec3,
    pub color: glm::Vec4,
    pub mesh_data: MeshData,
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

        let view = glm::inverse(&view);

        let cube_pos = glm::vec3(0.0, 0.0, 0.0);

        let color = glm::vec4(0.0, 1.0, 0.0, 0.0);

        Scene {
            camera: view,
            model_pos: cube_pos,
            color: color,
            mesh_data,
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

        let view = glm::inverse(&view);

        let cube_pos = glm::vec3(0.0, 0.0, 0.0);

        let color = glm::vec4(0.0, 1.0, 0.0, 0.0);

        Scene {
            camera: view,
            model_pos: cube_pos,
            color: color,
            mesh_data,
        }
    }

    pub fn camera_data(&self) -> Vec<f32> {

        let view_vec: Vec<f32> = self.camera.data.to_vec();

        let mut data = view_vec.clone();

        let model = self.model_mat();

        let mut model_vec: Vec<f32> = model.as_slice().to_vec();

        data.append(&mut model_vec);

        let color = self.color;

        let mut color_vec: Vec<f32> = color.as_slice().to_vec();

        data.append(&mut color_vec);

        data

    }

    pub fn model_mat(&self) -> glm::Mat4 {

        glm::translation(&self.model_pos)
    }

    pub fn update_model_position(&mut self, command: Command) {

        match command {
            Command::MoveLeft => {
                self.model_pos = self.model_pos + glm::vec3(-0.1, 0.0, 0.0);
            },
            Command::MoveRight => {
                self.model_pos = self.model_pos + glm::vec3(0.1, 0.0, 0.0);
            },
            _ => (),

        }
    }
}