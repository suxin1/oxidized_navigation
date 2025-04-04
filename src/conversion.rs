use bevy::prelude::{Transform, Vec3};
use bevy_editor_pls::egui::Shape::Vec;
use parry3d::{
    // math::Real,
    na::Point3,
    shape::{Ball, Capsule, Cone, Cuboid, Cylinder, Triangle},
};

use crate::{heightfields::TriangleCollection, Area};

pub struct GeometryCollection {
    pub transform: Transform,
    pub geometry_to_convert: GeometryToConvert,
    pub area: Option<Area>,
}

pub enum ColliderType {
    Cuboid(Cuboid),
    Ball(Ball),
    Capsule(Capsule),
    Cylinder(Cylinder),
    Cone(Cone),
    Triangle(Triangle),
}

pub enum GeometryToConvert {
    Collider(ColliderType),
    // Seems parry3d::math::Real not importable, use f32 instead
    ParryTriMesh(Box<[Point3<f32>]>, Box<[[u32; 3]]>),
}

pub(super) enum Triangles {
    Triangle([Vec3; 3]),
    TriMesh(Box<[Vec3]>, Box<[[u32; 3]]>),
}

const SUBDIVISIONS: u32 = 5;

pub(super) fn convert_geometry_collections(
    geometry_collections: Vec<GeometryCollection>,
) -> Box<[TriangleCollection]> {
    geometry_collections
        .into_iter()
        .map(|geometry_collection| TriangleCollection {
            transform: geometry_collection.transform,
            triangles: convert_geometry(geometry_collection.geometry_to_convert),
            area: geometry_collection.area,
        })
        .collect()
}

pub(super) fn convert_geometry(geometry_to_convert: GeometryToConvert) -> Triangles {
    match geometry_to_convert {
        GeometryToConvert::Collider(collider) => {
            let (vertices, triangles) = match collider {
                ColliderType::Cuboid(cuboid) => cuboid.to_trimesh(),
                ColliderType::Ball(ball) => ball.to_trimesh(SUBDIVISIONS, SUBDIVISIONS),
                ColliderType::Capsule(capsule) => capsule.to_trimesh(SUBDIVISIONS, SUBDIVISIONS),
                ColliderType::Cylinder(cylinder) => cylinder.to_trimesh(SUBDIVISIONS),
                ColliderType::Cone(cone) => cone.to_trimesh(SUBDIVISIONS),
                ColliderType::Triangle(triangle) => {
                    let vertices = triangle.vertices();
                    return Triangles::Triangle([
                            Vec3::new(vertices[0].x, vertices[0].y, vertices[0].z),
                            Vec3::new(vertices[1].x, vertices[1].y, vertices[1].z),
                            Vec3::new(vertices[2].x, vertices[2].y, vertices[2].z),
                        ]);
                }
            };

            let vertices = vertices
                .iter()
                .map(|point| Vec3::new(point.x, point.y, point.z))
                .collect();

            Triangles::TriMesh(vertices, triangles.into_boxed_slice())
        }
        GeometryToConvert::ParryTriMesh(vertices, triangles) => {
            let vertices = vertices
                .iter()
                .map(|point| Vec3::new(point.x, point.y, point.z))
                .collect();

            Triangles::TriMesh(vertices, triangles)
        }
    }
}
