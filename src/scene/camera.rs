#[derive(Debug)]
pub struct Camera {
    pub view_matrix: glm::Mat4
}

impl Camera {
    pub fn new(position: glm::Vec3, rotation: glm::Quat) -> Self {
        let t = glm::translation(&position);
        let r = glm::quat_to_mat4(&rotation);
        Camera {
            view_matrix: t * r,
        }
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        self.view_matrix
    }
}