use nalgebra_glm as glm;

use crate::input::Command;

pub struct Scene {
    pub camera: glm::Mat4,
    pub cube_pos: glm::Vec3,
    pub color: glm::Vec4,
}

impl Scene {
    pub fn new() -> Scene {


        let view = glm::look_at(
            &glm::vec3(0.0,0.0,7.0), // Camera is at (4,3,3), in World Space
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
        }
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