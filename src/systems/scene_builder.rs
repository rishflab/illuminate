use crate::components::*;
use crate::scene::{Scene, MeshInstance};
use crate::scene;
use specs::prelude::*;
use nalgebra_glm::{vec3, vec3_to_vec4};

#[derive(Debug)]
pub struct SceneBuilder;

impl<'a> System<'a> for SceneBuilder {

    type SystemData = (
        ReadStorage<'a, StaticMesh>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rotation>,
        ReadStorage<'a, Scale>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, PointLight>,
        Write<'a, Scene>,
    );

    fn run(&mut self, data: Self::SystemData){

        let (meshes, positions, rotations, scales, cameras, lights, mut scene) = data;

        let mesh_instances: Vec<MeshInstance> = (&meshes, &positions, &rotations, &scales).join()
            .map(|(mesh, position, rotation, scale)|{
                MeshInstance::new(mesh.0, position.0, rotation.0, scale.0)
            })
            .collect();

        let mut cameraz: Vec<scene::camera::Camera> = (&cameras, &positions, &rotations).join()
            .map(|(_, position, rotation)|{
                scene::camera::Camera::new(position.0, rotation.0)
            })
            .collect();

        let mut lightz: Vec<scene::light::PointLight> = (&lights, &positions).join()
            .map(|(light, position)|{
                scene::light::PointLight::new(position.0, light.0)
            })
            .collect();

        scene.camera = cameraz.pop().unwrap();
        scene.lights = lightz;
        scene.mesh_instances = mesh_instances;
    }
}

