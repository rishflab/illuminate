use specs::prelude::*;
use glm::vec3;

pub struct Player;

impl Component for Player {
    type Storage = VecStorage<Self>;
}

pub struct StaticMesh(pub usize);

impl Component for StaticMesh {
    type Storage = VecStorage<Self>;
}

pub struct PointLight(pub f32);

impl Component for PointLight {
    type Storage = VecStorage<Self>;
}

pub struct Transform{
    pub position: glm::Vec3,
    pub scale: glm::Vec3,
    pub rotation: glm::Vec3,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: vec3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
            rotation: vec3(0.0, 0.0, 0.0),
        }
    }
}

pub struct Position(pub glm::Vec3);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub struct Rotation(pub glm::Quat);

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}

pub struct Scale(pub glm::Vec3);

impl Component for Scale {
    type Storage = VecStorage<Self>;
}

pub struct Camera;

impl Component for Camera {
    type Storage = VecStorage<Self>;
}



