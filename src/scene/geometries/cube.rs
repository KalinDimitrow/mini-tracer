use nalgebra::{Point3, Vector3};
use crate::auxiliary::{ray::Ray, intersection_info::IntersectionInfo, rotation_utils::from_euler_angles};
use crate::scene::elements::scene_element::Geometry;
use crate::scene::geometries::rectangle::Rectangle;
use crate::scene::geometries::plane::Plane;

pub struct Cube {
    rects : [Rectangle; 6],
}

impl Cube {
    pub fn new(position : Point3<f32>, roll : f32, pitch : f32, yaw : f32, size : f32) -> Self {
        let offset = size/2.0f32;
        let rotation = from_euler_angles(roll, pitch, yaw);
        let up = rotation*Vector3::new(0.0f32, offset, 0.0f32);
        let forward = rotation*Vector3::new(0.0f32, 0.0f32, -offset);
        let right = rotation*Vector3::new(offset, 0.0f32, 0.0f32);
        Self {
            rects : [
                Rectangle{ plane : Plane::new(position + up, roll, pitch, yaw), size : (size, size)}, // top
                Rectangle{ plane : Plane::new(position - up, roll, pitch + std::f32::consts::PI, yaw), size : (size, size)}, // bottom
                Rectangle{ plane : Plane::new(position - right, roll + std::f32::consts::PI/2f32, pitch, yaw ), size : (size, size)}, // left
                Rectangle{ plane : Plane::new(position + right, roll - std::f32::consts::PI/2f32, pitch, yaw ), size : (size, size)}, // right
                Rectangle{ plane : Plane::new(position + forward, roll, pitch - std::f32::consts::PI/2f32, yaw), size : (size, size)}, // front
                Rectangle{ plane : Plane::new(position - forward, roll, pitch + std::f32::consts::PI/2f32, yaw), size : (size, size)}, // back
            ]
        }
    }
}

impl Geometry for Cube {
    fn intersect(&self, ray : &Ray) ->Option<IntersectionInfo> {
        let mut distance : f32 = f32::INFINITY;
        let mut result = None;
        for rect in &self.rects {
            let intersection = rect.intersect(ray);
            if let Some(intersection) = intersection {
                if intersection.distance < distance {
                    distance = intersection.distance;
                    result = Some(intersection);
                }
            }
        }

        result
    }
}