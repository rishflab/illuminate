pub struct Ray {
    pub origin: [f32; 4],
    pub direction: [f32; 4],
}

pub struct Intersection {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub edge: [f32; 4],
    pub float: f32,
}

pub struct PointLight{
    pub position: [f32; 4],
    pub intensity: f32,
}

pub struct Index(pub u32);

pub struct Vertex (pub [f32; 4]);

pub struct Aabb {
    pub min: [f32; 4],
    pub max: [f32; 4],
}

pub struct MeshHandle {
    pub min: [u32; 4],
    pub max: [u32; 4],
}

pub struct Camera {
    pub view: glm::Mat4,
    pub resolution: [u32; 2],
}
