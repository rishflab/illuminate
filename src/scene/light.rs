use crate::input::MoveCommand;

pub struct PointLight {
    pub position: glm::Vec4,
    pub intensity: f32,
}

impl PointLight {
    pub fn new(position: glm::Vec3, intensity: f32) -> PointLight {
        PointLight {
            position: glm::vec3_to_vec4(&position),
            intensity,
        }
    }
    pub fn data(&self) -> Vec<f32> {
        let mut data = self.position.data.to_vec();
        data.push(self.intensity.clone());
        data
    }

    pub fn update_position(&mut self, command: MoveCommand) {

        match command {
            MoveCommand::Left => {
                self.position = self.position + glm::vec4(-0.1, 0.0, 0.0, 0.0);
            },
            MoveCommand::Right => {
                self.position = self.position + glm::vec4(0.1, 0.0, 0.0, 0.0);
            },
            _ => (),

        }
    }
}
