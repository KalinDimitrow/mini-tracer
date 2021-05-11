use nalgebra::{Point3, Vector3, Rotation3, Unit};
use crate::auxiliary::{ray::Ray, intersection_info::IntersectionInfo};
use crate::scene::elements::scene_element::{Geometry};

pub struct Sphere {
    pub position : Point3<f32>,
    pub rotation : Rotation3<f32>,
    pub radious : f32,
}

impl Sphere {
    pub fn new(position : Point3<f32>, radious : f32, roll : f32, pitch : f32, yaw : f32) -> Self {
        // from_euler_angles
        Sphere {
            position,
            rotation : nalgebra::Rotation3::from_euler_angles(roll, pitch, yaw),
            radious,
        }
    }

    pub fn u(face : &Vector3<f32>) -> f32 {
        0.5f32 + face.as_ref()[0].atan2(face.as_ref()[2])/std::f32::consts::TAU
    }

    pub fn v(face : &Vector3<f32>) -> f32 {
        0.5f32 - face.as_ref()[1].asin()/std::f32::consts::PI
    }
}

impl Geometry for Sphere {
    fn intersect(&self, ray : &Ray) ->Option<IntersectionInfo> {
        let delta = ray.start - self.position;
        // let a = ray.direction.dot(&ray.direction);
        let b = delta.dot(&ray.direction.as_ref());
        let c = delta.dot(&delta) - self.radious*self.radious;
        let discriminant_squared = b.powi(2) - c;
        if discriminant_squared < 0f32 {
            return None;
        }

        let disciminant = discriminant_squared.sqrt();
        let t1 = -b + disciminant;
        let t2 = -b - disciminant;
        let tmin = t1.min(t2);
        let tmax = t1.max(t2);
        let t = if tmin > 0f32 {tmin} else {tmax};
        let point = ray.start + t*ray.direction.as_ref();
        let normal = Unit::new_normalize(point - self.position);
        let relative_normal = self.rotation*normal.as_ref();
        Some(IntersectionInfo{
            point,
            normal,
            distance : t,
            u : Self::u(&relative_normal),
            v : Self::v(&relative_normal)
        })
    }
}