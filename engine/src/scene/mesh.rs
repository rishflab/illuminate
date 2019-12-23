use nalgebra_glm as glm;

#[derive(Debug)]
pub struct StaticMeshData {
    pub id: usize,
    pub indices: Vec<u32>,
    pub vertices: Vec<glm::Vec4>,
}

#[derive(Debug)]
pub struct MeshInstance {
    pub model_matrix: glm::Mat4,
    pub mesh_id: usize,
}

impl MeshInstance {

    pub fn new(mesh_id: usize, position: glm::Vec3, rotation: glm::Quat, scale: glm::Vec3) -> MeshInstance {
        let t = glm::translation(&position);
        let r = glm::quat_to_mat4(&rotation);
        let s = glm::scaling(&scale);
        let model_matrix = t * r * s;
        MeshInstance {
            model_matrix,
            mesh_id,
        }
    }

    pub fn model_matrix(&self) -> glm::Mat4 {
        self.model_matrix
    }
}