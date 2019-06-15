use nalgebra_glm as glm;
use crate::input::Command;

pub struct StaticMesh {
    pub id: usize,
    pub indices: Vec<u32>,
    pub vertices: Vec<glm::Vec4>,
}

pub struct MeshInstance {
    pub position: glm::Vec3,
    pub scale: glm::Vec3,
    pub rotation: glm::Vec3,
    pub mesh_id: usize,
}

impl MeshInstance {

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