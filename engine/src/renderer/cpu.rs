use nalgebra_glm as glm;
use glm::{Vec3, vec3, vec2, vec4, normalize, cross, dot, vec3_to_vec4, Quat, quat_to_mat3, quat_look_at, distance, reflect_vec};
use crate::scene::light::PointLight;
use rayon::prelude::*;

#[derive(Debug)]
pub struct BBox {
    pub min: glm::Vec3,
    pub max: glm::Vec3,
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub index: (u32, u32),
    pub origin: glm::Vec3,
    pub direction: glm::Vec3,
}

#[derive(Debug)]
pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

#[derive(Debug)]
pub struct Intersection {
    pub index: (u32, u32),
    pub position: Vec3,
    pub normal: Vec3,
    pub edge: Vec3,
    pub a: f32,
}

impl Intersection {
    pub fn new() -> Intersection {
        Intersection {
            index: (0, 0),
            position: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            edge: vec3(0.0, 0.0, 0.0),
            a: -1.0,
        }
    }
}


pub fn trace_ray(camera_ray: &Ray, tris: &[Triangle], lights: &[PointLight]) -> f32 {

    let mut intersections = vec![];

    let mut ray = camera_ray.clone();

    [0..2].iter().for_each(|_|{
        let intersection = find_closest_intersection(&ray, tris);
        ray = reflection_ray(&intersection, &ray);
        intersections.push(intersection);
    });

    let mut shade = 0.0;

    for intersection in intersections.iter().rev() {
        if intersection.a > -1.0 {
            let shadow_ray = generate_shadow_ray(intersection, &lights[0]);
            if !intersects_any(&shadow_ray, tris) {
                shade += calculate_diffuse_shade(intersection, &lights[0], &shadow_ray);
            }
        }
    }

    shade
}

pub fn reflection_ray(intersection: &Intersection, ray: &Ray) -> Ray {
    Ray {
        index: ray.index,
        origin: intersection.position,
        direction: reflect_vec(&ray.direction, &intersection.normal),
    }
}

pub fn calculate_diffuse_shade(intersection: &Intersection, light: &PointLight, shadow_ray: &Ray) -> f32 {
    let angle = dot(&intersection.normal, &(-1.0 * shadow_ray.direction)

    );
    let distance = distance(&intersection.position, &light.position.xyz());
    light.intensity * (1.0/(distance * distance)) * angle
}


pub fn intersect_box(ray: Ray, aabb: BBox) -> bool {
    let tMin = comp_div3(aabb.min - ray.origin, ray.direction);
    let tMax = comp_div3(aabb.max - ray.origin, ray.direction);
    let t1 = glm::min2(&tMin, &tMax);
    let t2 = glm::max2(&tMin, &tMax);
    let tNear = glm::max(&glm::max(&glm::vec1(t1.x), t1.y), t1.z);
    let tFar = glm::min(&glm::min(&glm::vec1(t2.x), t2.y), t2.z);
    tFar >= tNear
}

pub fn calculate_intersection(ray: &Ray, tri: &Triangle) -> Intersection {
    let v1 = tri.0;
    let v2 = tri.1;
    let v3 = tri.2;
    let e1 = v2 - v1;
    let e2 = v3 - v1;
    let s1 = &ray.direction.cross(&e2);
    let det = dot(&e1, &s1);
    let invd = 1.0/det;
    let d = ray.origin - v1;
    let b1 = dot(&d, &s1) * invd;
    let s2 = &d.cross(&e1);
    let b2 = dot(&ray.direction, &s2) * invd;
    let temp = dot(&e2, &s2) * invd;
    let position = ray.origin + (ray.direction * temp);
    let normal = normalize(&e2.cross(&e1));
    if b1 < 0.0 || b1 > 1.0 || b2 < 0.0 || (b1 + b2) > 1.0 || temp <= 0.0 || det < 0.0 {
        Intersection {
            index: ray.index,
            position: vec3(position.x, position.y, position.z),
            normal: vec3(normal.x, normal.y, normal.z),
            edge: e2,
            a: -1.0,
        }
    } else {
        Intersection {
            index: ray.index,
            position: vec3(position.x, position.y, position.z),
            normal: vec3(normal.x, normal.y, normal.z),
            edge: e2,
            a: temp,
        }
    }
}

pub fn intersects(ray: &Ray, tri: &Triangle) -> bool {
    if calculate_intersection(ray, tri).a == -1.0 {
        false
    } else {
        true
    }
}

pub fn find_closest_intersection(ray: &Ray, tris: &[Triangle]) -> Intersection {
    let intersections: Vec<Intersection> = tris.iter()
        .map(|tri|{
            calculate_intersection(ray, tri)
        }).collect();

    let mut closest_intersection = Intersection::new();

    for intersection in intersections {
        if intersection.a != -1.0 && closest_intersection.a == -1.0 {
            closest_intersection = intersection;
        } else if intersection.a != -1.0 && closest_intersection.a != -1.0 {
            let old_dist = distance(&closest_intersection.position, &ray.origin);
            let new_dist = distance(&intersection.position, &ray.origin);
            if new_dist < old_dist {
               closest_intersection = intersection;
            }
        }
    }
    closest_intersection
}

pub fn intersects_any(ray: &Ray, tris: &[Triangle]) -> bool {
    let intersections: Vec<bool> = tris.iter()
        .map(|tri|{
            intersects(ray, tri)
        }).collect();

    let mut result = false;

    for intersection in intersections.iter() {
        if *intersection {
            result = true
        }
    }
    result
}

pub fn generate_camera_rays(resolution: (u32, u32)) -> Vec<Ray> {
    let mut rays = vec![];
    let aspect_ratio = resolution.0 as f32 / resolution.1 as f32;
    for i in 0..resolution.0 {
        for j in 0..resolution.1 {
            let mut cam_origin = vec3(0.0, 0.0, 0.0);
            let raster_coord = vec2(i as f32, j as f32);
            let norm_coords = comp_div2(raster_coord, vec2(resolution.0 as f32, resolution.1 as f32));
            let offset = vec2(aspect_ratio * 2.0 * norm_coords.x, -2.0 * norm_coords.y);
            let screen_coord = vec2(aspect_ratio*-1.0, 1.0) + offset;
            let screen_ray_intersection = vec3(screen_coord.x, screen_coord.y, -1.0);
            let ray_direction = normalize(&(screen_ray_intersection - cam_origin));
            let ray = Ray {
                index: (i, j),
                origin: cam_origin,
                direction: ray_direction,
            };
            rays.push(ray)
        }
    }
    rays
}

pub fn generate_shadow_ray(intersection: &Intersection, light: &PointLight) -> Ray {
    Ray {
        index: intersection.index,
        origin: intersection.position,
        direction: normalize(&(light.position.xyz() - intersection.position)),
    }
}

pub fn transform_camera_rays(rays: Vec<Ray>, position: &Vec3, rotation: &Quat) -> Vec<Ray> {
    let r = glm::quat_to_mat3(&rotation);
    rays.par_iter()
        .map(|ray|{
            Ray{
                index: ray.index,
                origin: position.clone(),
                direction: r * ray.direction,
            }
    }).collect()
}

fn comp_div3(a: glm::Vec3, b: glm::Vec3) -> glm::Vec3 {
    glm::vec3(a.x/b.x, a.y/b.y, a.z/b.z)
}

fn comp_div4(a: glm::Vec4, b: glm::Vec4) -> glm::Vec4 {
    glm::vec4(a.x/b.x, a.y/b.y, a.z/b.z, a.w/b.w)
}

fn comp_div2(a: glm::Vec2, b: glm::Vec2) -> glm::Vec2 {
    glm::vec2(a.x/b.x, a.y/b.y)
}

#[cfg(test)]
mod tests {

    use super::*;
    use glm::{vec3};

    #[test]
    fn intersects_box() {
        let bbox = BBox {
            min: vec3(0.0, 0.0, 0.0),
            max: vec3(1.0, 1.0, 1.0),
        };
        let ray = Ray {
            index: (0, 0),
            origin: vec3(0.0, 0.0, -1.0),
            direction: vec3(1.0, 1.0, 1.0),
        };
        assert!(intersect_box(ray, bbox))
    }

    #[test]
    fn does_not_intersect_box() {
        let bbox = BBox {
            min: vec3(0.0, 0.0, 0.0),
            max: vec3(1.0, 1.0, 1.0),
        };
        let ray = Ray {
            index: (0, 0),
            origin: vec3(0.0, 0.0, -1.0),
            direction: vec3(-1.0, 1.0, 1.0),
        };
        assert!(!intersect_box(ray, bbox))
    }

}

