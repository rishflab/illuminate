use nalgebra_glm as glm;

use crate::input::Command;
use crate::asset::MeshData;

pub struct Scene {
    pub camera: glm::Mat4,
    pub cube_pos: glm::Vec3,
    pub color: glm::Vec4,
    pub mesh_data: MeshData,
}

impl Scene {
    pub fn new(mesh_data: MeshData) -> Scene {

        let view = glm::look_at(
            &glm::vec3(0.0,1.0,7.0), // Camera is at (4,3,3), in World Space
            &glm::vec3(0.0,0.0,0.0), // and looks at the origin
            &glm::vec3(0.0,1.0,0.0)  // Head is up (set to 0,-1,0 to look upside-down)
        );

        let view = glm::inverse(&view);

        let cube_pos = glm::vec3(0.0, 0.0, 0.0);

        let color = glm::vec4(0.0, 0.0, 0.0, 0.0);

        Scene {
            camera: view,
            cube_pos: cube_pos,
            color: color,
            mesh_data,
        }
    }

    pub fn camera_data(&self) -> Vec<f32> {


        let view_vec: Vec<f32> = self.camera.data.to_vec();

        let mut data = view_vec.clone();

        let model = self.cube_model_mat();

        let mut model_vec: Vec<f32> = model.as_slice().to_vec();

        data.append(&mut model_vec);

        let color = self.color;

        let mut color_vec: Vec<f32> = color.as_slice().to_vec();

        data.append(&mut color_vec);

        data

    }

    pub fn cube_model_mat(&self) -> glm::Mat4 {
        glm::translation(&self.cube_pos)
    }

    pub fn update_cube(&mut self, command: Command) {

        match command {
            Command::MoveLeft => {
                self.cube_pos = self.cube_pos + glm::vec3(-0.1, 0.0, 0.0);
            },
            Command::MoveRight => {
                self.cube_pos = self.cube_pos + glm::vec3(0.1, 0.0, 0.0);
            },
            _ => (),

        }
    }
}