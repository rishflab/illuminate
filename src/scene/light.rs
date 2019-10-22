use crate::input::Command;

pub struct PointLight {
    pub position: glm::Vec4,
    pub intensity: f32,
}

impl PointLight {
    pub fn data(&self) -> Vec<f32> {
        let mut data = self.position.data.to_vec();
        data.push(self.intensity.clone());
        data
    }

    pub fn update_position(&mut self, command: Command) {

        match command {
            Command::MoveLeft => {
                self.position = self.position + glm::vec4(-0.1, 0.0, 0.0, 0.0);
                //self.look_at = self.look_at + glm::vec4(-0.1, 0.0, 0.0, 0.0);
            },
            Command::MoveRight => {
                self.position = self.position + glm::vec4(0.1, 0.0, 0.0, 0.0);
                //self.look_at = self.look_at + glm::vec4(0.1, 0.0, 0.0, 0.0);
            },
            _ => (),

        }
    }
}
