use crate::components::*;
use crate::scene::{Scene, MeshInstance};
use crate::scene;
use specs::prelude::*;
use nalgebra_glm::{vec3, vec3_to_vec4};

pub struct SceneBuilder;

impl<'a> System<'a> for SceneBuilder {

    type SystemData = (
        ReadStorage<'a, StaticMesh>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, PointLight>,
        Write<'a, Scene>,
    );

    fn run(&mut self, data: Self::SystemData){

        let (meshes, transforms, cameras, lights, mut scene) = data;

        let mesh_instances: Vec<MeshInstance> = (&meshes, &transforms).join()
            .map(|(mesh, transform)|{
                MeshInstance{
                    position: transform.position,
                    scale: transform.scale,
                    rotation: transform.rotation,
                    mesh_id: mesh.0
                }
            })
            .collect();

        let mut cameraz: Vec<scene::camera::Camera> = (&cameras, &transforms).join()
            .map(|(camera, transform)|{
                scene::camera::Camera::new(transform.position, camera.look_at)
            })
            .collect();

        let mut lightz: Vec<scene::light::PointLight> = (&lights, &transforms).join()
            .map(|(light, transform)|{
                scene::light::PointLight{
                    position: vec3_to_vec4(&transform.position),
                    intensity: light.0
                }
            })
            .collect();

        scene.camera = cameraz.pop().unwrap();
        scene.lights = lightz;
        scene.mesh_instances = mesh_instances;
    }
}

