use nalgebra::{Point3, Unit, Vector3, Rotation3};
use crate::auxiliary::ray::Ray;

#[allow(dead_code)]
pub struct Camera {
    pub position : Point3<f32>,
    roll : f32,
    pitch : f32,
    yaw : f32,
    aspect_ratio : f32,
    fov : f32,
    top_left : Vector3<f32>,
    right_direction : Vector3<f32>,
    down_direction : Vector3<f32>,
    pub resolution : (u32, u32),
}

impl Camera {
    pub fn new(position : Point3<f32>, roll : f32, pitch : f32, yaw : f32, aspect_ratio : f32, fov : f32, resolution : (u32, u32)) -> Camera {

        let short_diagonal = (fov.to_radians()/2.0).tan();
        let scale = (aspect_ratio * aspect_ratio + 1.0).sqrt();
        let x = (aspect_ratio * short_diagonal) / scale;
        let y = short_diagonal / scale;
        // default direction is (0.0, 0.0, -1.0)
        let top_left =  Vector3::new(-x, y, -1.0);
        let top_right =  Vector3::new(x, y, -1.0);
        let bottom_left =  Vector3::new(-x, -y, -1.0);
        let rotation = Rotation3::from_euler_angles(roll.to_radians(), pitch.to_radians(), yaw.to_radians());
        let top_left = rotation * top_left;
        let top_right = rotation * top_right;
        let bottom_left = rotation * bottom_left;
        let right_direction = top_right - top_left;
        let down_direction = bottom_left - top_left;
        Camera {position, roll, pitch, yaw, aspect_ratio, fov, top_left, right_direction, down_direction, resolution}
    }


    pub fn get_screen_ray(&self, x : f32, y : f32) -> Ray {
        let mut direction = &self.top_left + (x / self.resolution.0 as f32) * &self.right_direction;
        direction  += (y / self.resolution.1 as f32) * &self.down_direction;
        let start = &self.position + direction;
        let direction = Unit::new_and_get(direction).0;
        Ray {start , direction }
    }
}