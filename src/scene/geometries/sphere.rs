use nalgebra::{Point3, Vector3, Rotation3, Unit};
use crate::auxiliary::{ray::Ray, intersection_info::IntersectionInfo};
use crate::scene::elements::scene_element::{Geometry};

pub struct Sphere {
    pub position : Point3<f32>,
    pub rotation : Rotation3<f32>,
    pub radious : f32,
    radious_squared : f32,
}

impl Sphere {
    pub fn new(position : Point3<f32>, radious : f32, roll : f32, pitch : f32, yaw : f32) -> Self {
        // from_euler_angles
        Sphere {
            position,
            rotation : nalgebra::Rotation3::from_euler_angles(roll, pitch, yaw),
            radious,
            radious_squared : radious.powi(2)
        }
    }

    pub fn u(face : &Vector3<f32>) -> f32 {
        0.5f32 + face.as_ref()[0].atan2(face.as_ref()[2])/std::f32::consts::TAU
    }

    pub fn v(face : &Vector3<f32>) -> f32 {
        0.5f32 - face.as_ref()[1].asin()/std::f32::consts::PI
    }

    fn generate_intersection_data(&self, t: f32, ray : &Ray) -> Option<IntersectionInfo> {
        let point = ray.start + t*ray.direction.as_ref();
        let normal = Unit::new_normalize(point - self.position);
        let relative_normal = normal.as_ref();

        Some(IntersectionInfo{
            point,
            normal,
            distance : t,
            u : Self::u(&relative_normal),
            v : Self::v(&relative_normal)
        })
    }

    fn intersection_geometric_implementation(&self, ray : &Ray) -> Option<IntersectionInfo> {
        let distance_from_center = self.position - ray.start;
        let mid_point_length = distance_from_center.dot(ray.direction.as_ref());
        if mid_point_length < 0f32 {
            return None;
        }

        let mid_point_center_distance_squared = distance_from_center.magnitude_squared() - mid_point_length.powi(2);
        if mid_point_center_distance_squared >= self.radious_squared {
            return None;
        }
        
        let delta = (self.radious_squared - mid_point_center_distance_squared).sqrt();
        let t = if delta < mid_point_length { mid_point_length - delta } else { mid_point_length + delta };

        self.generate_intersection_data(t, ray)
    }

    // fn intersection_analytical_implementation(&self, ray : &Ray) -> Option<IntersectionInfo> {
    //     let delta = ray.start - self.position;
    //     let b = delta.dot(&ray.direction.as_ref());
    //     let c = delta.magnitude_squared() - self.radious_squared;
    //     let discriminant_squared = b.powi(2) - c;
    //     if discriminant_squared < 0f32 {
    //         return None;
    //     }

    //     let disciminant = discriminant_squared.sqrt();
    //     let t1 = -b + disciminant;
    //     let t2 = -b - disciminant;
    //     let tmin = t1.min(t2);
    //     let tmax = t1.max(t2);
    //     let t = if tmin > 0f32 {tmin} else {tmax};
    //     if t < 0f32 {
    //         return None;
    //     }

    //     self.generate_intersection_data(t, ray)
    // }
}

impl Geometry for Sphere {
    fn intersect(&self, ray : &Ray) ->Option<IntersectionInfo> {
        self.intersection_geometric_implementation(ray)
    }
}