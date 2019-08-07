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
}
