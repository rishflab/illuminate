extern crate not_mechanical_engine as engine;
extern crate image;

use engine::renderer::cpu::*;
use engine::scene::camera::Camera;
use nalgebra_glm as glm;
use glm::{vec3, Vec3, quat_look_at};
use nalgebra_glm::{mat4_to_mat3, vec4, quat_identity, Vec4};
use image::{RgbImage, ImageBuffer};
use image::hdr::RGBE8Pixel;
use image::ColorType::RGB;
use engine::scene::light::PointLight;
use rayon::prelude::*;
use engine::scene::{Scene, MeshInstance};
use std::ops::Deref;
use engine::asset::MeshData;
use engine::scene::mesh::StaticMeshData;


fn main() {

    let width = 200;
    let height = 200;

    let up_vec = vec3(0.0, 1.0, 0.0);

    let cam_pos = vec3(0.0, 1.5, 5.0);
    let cam_rot = quat_look_at(&(vec3(0.0, 0.0, -20.0) + cam_pos),&up_vec);

    let rays = generate_camera_rays((width, height), 1000.0);

    let rays = transform_camera_rays(rays, &cam_pos, &cam_rot);

    let tris = boxes_scene();

    let light = PointLight {
        position: vec4(0.0, 2.5, 0.0, 0.0),
        intensity: 200.0,
    };

    let lights = vec![light];

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let pixels: Vec<(u32, u32, u8)> = rays.par_iter().map(|ray|{
        let shade = (255.0 * trace_ray(ray, &tris, &lights)) as u8;
        (ray.index.0, ray.index.1, shade)
    }).collect();

    for (x, y, shade) in pixels {
        img.put_pixel(x, y, image::Rgb([shade, shade, shade]));
    }

    img.save("target/render.png").unwrap();
}

fn boxes_scene() -> Vec<Triangle> {
    use specs::prelude::*;
    use engine::asset::{load_gltf, MeshData};
    use engine::scene::mesh::{StaticMeshData, MeshInstance};
    use engine::scene;
    use engine::components::*;
    use engine::systems::scene_builder::SceneBuilder;

    let asset_folder = "assets";
    let gltf = load_gltf(asset_folder, "cube.gltf")
        .expect("failed to load gltf");

    let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

    let cube_mesh = StaticMeshData {
        id: 0,
        indices: mesh_data.indices.clone(),
        vertices: mesh_data.vertices.clone(),
    };

    let mut scene = Scene::default();
    scene.mesh_data.push(cube_mesh);

    let mut world = World::new();

    let mut init = DispatcherBuilder::new()
        .with(SceneBuilder, "scene_builder", &[])
        .build();

    init.setup(&mut world);

    world.insert(scene);

    let camera = world.create_entity()
        .with(Position(vec3(0.0, 2.0, 6.0)))
        .with(Rotation(quat_look_at(&vec3(0.0, 2.0, -7.0), &vec3(0.0, 1.0, 0.0))))
        .with(Camera)
        .build();

    let light = world.create_entity()
        .with(PointLight(40.0))
        .with(Position(vec3(0.0, 5.0, 0.0)))
        .build();

    let floor = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(0.0, -3.0, 0.0)))
        .with(Rotation(quat_identity()))
        .with(Scale(glm::vec3(5.0, 0.5, 5.0)))
        .build();

    let ceiling = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(0.0, 5.0, 0.0)))
        .with(Rotation(quat_identity()))
        .with(Scale(glm::vec3(5.0, 0.5, 5.0)))
        .build();

    let back_wall = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(0.0, 0.0, -5.0)))
        .with(Rotation(quat_identity()))
        .with(Scale(glm::vec3(5.0, 5.0, 0.5)))
        .build();

    let left_wall = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(-5.0, 0.0, 0.0)))
        .with(Scale(vec3(0.5, 5.0, 5.0)))
        .with(Rotation(quat_identity()))
        .build();

    let right_wall = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(5.0, 0.0, 0.0)))
        .with(Scale(vec3(0.5, 5.0, 5.0)))
        .with(Rotation(quat_identity()))
        .build();

    let tall_block = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(-1.0, -2.5, 1.0)))
        .with(Scale(vec3(0.5, 2.5, 0.5)))
        .with(Rotation(quat_identity()))
        //.with(Rotation(quat_look_at(&vec3(0.0, -2.5, 0.0), &up_vec))
        .build();

    let right_block = world.create_entity()
        .with(StaticMesh(0))
        .with(Position(vec3(2.0, -2.5, -1.0)))
        .with(Scale(vec3(1.0, 1.0, 0.5)))
        .with(Rotation(quat_identity()))
        .build();

    init.dispatch(&world);

    let instances = world.fetch::<Scene>().mesh_instances.clone();
    let data = world.fetch::<Scene>().mesh_data.clone();
    build_triangles(instances, data)
}

pub fn build_triangles(mesh_instances: Vec<MeshInstance>, mesh_data: Vec<StaticMeshData>) -> Vec<Triangle> {
    mesh_instances.iter().map(|instance|{
        let mesh_data = mesh_data.get(0).unwrap();
        let vertices: Vec<Vec4> = mesh_data.vertices.iter()
            .map(|vertex|{
                instance.model_matrix * vertex
            }).collect();
        let tris: Vec<Triangle> = mesh_data.indices.chunks(3).map(|ix|{
            Triangle(vertices[ix[0] as usize].xyz(), vertices[ix[1] as usize].xyz(), vertices[ix[2] as usize].xyz())
        }).collect();
        tris
    }).flatten().collect()
}


