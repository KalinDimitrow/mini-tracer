use std::default;

use nalgebra::{Point3, Vector3, Rotation3, Dim};
use crate::auxiliary::{ray::Ray, intersection_info::IntersectionInfo};
use crate::scene::elements::scene_element::Geometry;
use crate::scene::geometries::plane::Plane;

pub struct RegularPolygon {
    plane: Plane,
    points: Vec<Point3<f32>>,
    sides: Vec<Vector3<f32>>

}

impl RegularPolygon {
    pub fn new(position : Point3<f32>, roll : f32, pitch : f32, yaw : f32, vertecies : usize, size : f32) -> Self {
        let plane = Plane::new(position, roll, pitch, yaw);
        let mut points: Vec<Point3<f32>> = vec![];
        let mut sides: Vec<Vector3<f32>> = vec![];
        let size = size/2f32;
        
        let up_direction = plane.v.into_inner();
        let angle_step = std::f32::consts::TAU/vertecies as f32;
        for i in 0..vertecies {
            let rot = Rotation3::from_axis_angle(&plane.normal, angle_step*(i as f32));
            let point = plane.position + size*(rot*up_direction); //plane.u.into_inner()*size;
            points.push(point);
        }

        for i in 0..vertecies {
            sides.push(points[i] - points[(i + 1)%vertecies])
        }

        Self {
            plane,
            points,
            sides,
        }
    }
}

impl Geometry for RegularPolygon {
    fn intersect(&self, ray : &Ray) ->Option<IntersectionInfo> {
        if let Some(intersection) = self.plane.intersect(ray) {

            let intersection_vecs = self.points.iter()
            .map(|point| intersection.point - point).collect::<Vec<_>>();
            // let intersection_vecs = vec![
                // intersection.point - self.points[0],
                // intersection.point - self.points[1],
                // intersection.point - self.points[2]
            // ];

            for i in 0..self.points.len() {
                let begin_index = i;
                let end_index = (i + 1) % self.points.len();
                if self.sides[begin_index].cross(&intersection_vecs[end_index])
                .dot(&self.sides[end_index].cross(&intersection_vecs[end_index])) <= 0.0 {
                    return None;
                }
            }

            return Some(intersection);
        }

        None
    }
}