extern crate blackhole;

use blackhole::renderer::pathtracer::Pathtracer;
use blackhole::window::WindowState;
use blackhole::renderer::core::backend::{create_backend};
use blackhole::input::{InputState, Command};
use blackhole::scene::{Scene};
use specs::prelude::*;
use blackhole::asset::{load_gltf, MeshData};
use blackhole::scene::mesh::{StaticMeshData, MeshInstance};
use blackhole::scene;
use nalgebra_glm::{vec3, vec3_to_vec4};
use nalgebra_glm as glm;

struct StaticMesh(usize);

impl Component for StaticMesh {
    type Storage = VecStorage<Self>;
}

struct PointLight(f32);

impl Component for PointLight {
    type Storage = VecStorage<Self>;
}

struct Transform{
    position: glm::Vec3,
    scale: glm::Vec3,
    rotation: glm::Vec3,
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

struct Camera{
    look_at: glm::Vec3,
}

impl Component for Camera {
    type Storage = VecStorage<Self>;
}

//struct InitRenderer(Pathtracer<B::Backend>);

//impl InitRenderer{
//    unsafe fn new (scene: Scene) -> Self {
//        RenderSys(Pathtracer::new(backend, window, &scene))
//    }
//}


//impl<'a> System<'a> for InitRenderer {
//
//    type SystemData = (
//        Read<'a, Scene>,
//        Read<'a, Pathtracer>,
//    );
//
//    fn run(&mut self, data: Self::SystemData){
//
//
//    }
//}

struct SceneBuilder;

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

//        let new = Scene::multiple_boxes();
//
//        scene.camera = new.camera;
//        scene.lights = new.lights;
//        scene.mesh_instances = new.mesh_instances;
//        scene.mesh_data = new.mesh_data;

    }
}


fn main() {
    env_logger::init();

    let asset_folder = "assets";
    let gltf = load_gltf(asset_folder, "untitled.gltf")
        .expect("failed to load gltf");

    let mut window = WindowState::new();
    let mut input = InputState::new();
    let (backend, _instance) = create_backend(&mut window, &mut input);


    let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

    let cube_mesh = StaticMeshData {
        id: 0,
        indices: mesh_data.indices.clone(),
        vertices: mesh_data.vertices.clone(),
    };

    let scene = Scene::default();


    let mut world = World::new();

    world.register::<PointLight>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(SceneBuilder, "scene_builder", &[])
        .build();

    dispatcher.setup(&mut world);

    world.insert(scene);

    let floor = world.create_entity()
        .with(StaticMesh(0))
        .with(Transform{
            position: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(10.0, 1.0, 10.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
        })
        .build();

    let camera =  world.create_entity()
        .with(Transform{
            position: vec3(0.0, 4.0, 8.0),
            scale: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
        })
        .with(Camera{
            look_at: vec3( 0.0, 2.0, -7.0)
        })
        .build();

    let light = world.create_entity()
        .with(PointLight(20.0))
        .with(Transform{
            position: vec3(1.5, 4.0, 4.0, ),
            scale: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
        })
        .build();


    dispatcher.dispatch(&world);

    let mut renderer = unsafe {
        Pathtracer::new(backend, window, &world.fetch::<Scene>())
    };



    let mut running = true;

    while running {

        use std::time::Instant;

        match input.process_raw_input() {
            Some(command) => {
                match command {
                    Command::Close => {
                        running = false;
                    },
                    _ => {
                    },
                }
            },
            None => (),
        }

        let start = Instant::now();

        renderer.render(   &world.fetch::<Scene>());

        let duration = start.elapsed();

        println!("Frame time {:?}", duration);
    }
}