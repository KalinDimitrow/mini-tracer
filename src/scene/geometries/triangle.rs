use nalgebra::{Point3, Vector3, Unit};
use crate::auxiliary::{ray::Ray, intersection_info::IntersectionInfo};
use crate::scene::elements::scene_element::Geometry;
use crate::scene::geometries::plane::Plane;
pub struct Triangle {
    pub plane : Plane,
    pub points : [Point3<f32>; 3],
    pub directed_sides : [Vector3<f32>; 3]
}

impl Triangle {
    pub fn new(a : Point3<f32>, b : Point3<f32>, c : Point3<f32>) -> Self {
        let points = [a, b, c];

        let mut directed_sides : [Vector3<f32>; 3] = [Vector3::new(0.0, 0.0, 0.0); 3];
        for i in 0..points.len() {
            let begin_index = i;
            let end_index = (i + 1) % points.len();
            directed_sides[begin_index] = (points[end_index] - points[begin_index]).normalize();
        }

        let projection = directed_sides[0].dot(&directed_sides[1]);
        let v = (directed_sides[1] - projection*directed_sides[0]).normalize();
        let normal = directed_sides[0].cross(&directed_sides[1]);
        let position = Point3::from((a.coords + b.coords + c.coords)/3f32);
        


        Triangle{ plane : Plane::new_from_vectors(
            position,
            Unit::new_unchecked(normal),
            Unit::new_unchecked(directed_sides[0]),
            Unit::new_unchecked(v)),
            points,
            directed_sides
        }
    }
}

impl Geometry for Triangle {
    fn intersect(&self, ray : &Ray) ->Option<IntersectionInfo> {
        let intersection = self.plane.intersect(ray);
        if let Some(intersection) = intersection {

            let intersection_vecs = vec![
                intersection.point - self.points[0],
                intersection.point - self.points[1],
                intersection.point - self.points[2]
            ];

            for i in 0..self.points.len() {
                let begin_index = i;
                let end_index = (i + 1) % self.points.len();
                if self.directed_sides[begin_index].cross(&intersection_vecs[end_index])
                .dot(&self.directed_sides[end_index].cross(&intersection_vecs[end_index])) <= 0.0 {
                    return None;
                }
            }

            return Some(intersection);
        }

        None
    }
}