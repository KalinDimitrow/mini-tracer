use nalgebra::{Point3, Vector3, Rotation3};
use crate::auxiliary::ray::Ray;

#[allow(dead_code)]
pub struct Camera {
    pub position : Point3<f32>,
    pitch : f32,
    yaw : f32,
    row : f32,
    aspect_ratio : f32,
    fov : f32,
    top_left : Vector3<f32>,
    right_direction : Vector3<f32>,
    down_direction : Vector3<f32>,
    resolution : (u32, u32),
}

impl Camera {
    pub fn new(position : Point3<f32>, pitch : f32, yaw : f32, row : f32, aspect_ratio : f32, fov : f32, resolution : (u32, u32)) -> Camera {

        let short_diagonal = (fov.to_radians()/2.0).tan();
        let scale = (aspect_ratio * aspect_ratio + 1.0).sqrt();
        let x = (aspect_ratio * short_diagonal) / scale;
        let y = short_diagonal / scale;
        let top_left =  Vector3::new(-x, y, 1.0);
        let top_right =  Vector3::new(x, y, 1.0);
        let bottom_left =  Vector3::new(-x, -y, 1.0);
        let axis = Vector3::<f32>::z_axis();
        let mut rotation = Rotation3::from_axis_angle(&axis, row.to_radians());
        let axis = Vector3::<f32>::x_axis();
        rotation *= Rotation3::from_axis_angle(&axis, pitch.to_radians());
        let axis = Vector3::<f32>::y_axis();
        rotation *= Rotation3::from_axis_angle(&axis, yaw.to_radians());
        let top_left = rotation * top_left;
        let top_right = rotation * top_right;
        let bottom_left = rotation * bottom_left;
        let right_direction = (top_right - top_left).normalize();
        let down_direction = (bottom_left - top_left).normalize();
        Camera {position, pitch, yaw, row, aspect_ratio, fov, top_left, right_direction, down_direction, resolution}
    }


    pub fn get_screen_ray(&self, x : f32, y : f32) -> Ray {
        let mut direction = &self.top_left + (x / self.resolution.0 as f32) * &self.right_direction;
        direction  += (y / self.resolution.1 as f32) * &self.down_direction;
        let start = &self.position + direction;
        
        Ray {start , direction }
    }
}