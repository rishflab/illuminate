use crate::input::MoveCommand;

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

    pub fn update_position(&mut self, command: MoveCommand) {

        match command {
            MoveCommand::MoveLeft => {
                self.position = self.position + glm::vec3(-0.1, 0.0, 0.0);
                self.look_at = self.look_at + glm::vec3(-0.1, 0.0, 0.0);
            },
            MoveCommand::MoveRight => {
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