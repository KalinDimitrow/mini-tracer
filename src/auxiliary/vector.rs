// use num::Float;
// use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
//
// type ValueType = f32;
// pub struct Vector3d {
//     x : ValueType,
//     y : ValueType,
//     z : ValueType,
// }
//
// impl Mul<Vector3d> for ValueType {
//     type Output = Vector3d;
//     fn mul(self, rhs: Self::Output) -> Self::Output {
//         Self::Output {x : rhs.x * self, y : rhs.y * self, z : rhs.z * self}
//     }
// }
//
// fn close<T>(a : &T, b : &T, epsilone :& T) -> bool
//     where T : Float + std::ops::Sub<Output = T> {
//     !((*a - *b).abs() > *epsilone)
// }
//
// impl Vector3d {
//     pub fn new(x : ValueType, y : ValueType, z : ValueType) -> Self {
//         Self{x, y, z}
//     }
//
//     pub fn close( first : &Self, second : &Self, epsilon : ValueType) -> bool {
//         close(&first.x, &second.x, &epsilon) &&
//             close(&first.y, &second.y, &epsilon) &&
//             close(&first.z, &second.z, &epsilon)
//     }
//
//     pub fn length(&self) -> ValueType {
//         Float::sqrt(self.length_squared())
//     }
//
//     pub fn length_squared(&self) -> ValueType {
//         Self::dot(self, self)
//     }
//
//     pub fn dot(first : &Vector3d, second : &Vector3d) -> ValueType {
//         first.x*second.x + first.y*second.y + first.z*second.z
//     }
//
//     pub fn cross(first : &Vector3d, second : &Vector3d) -> Vector3d {
//         Self::new(x : )
//     }
// }
//
// impl Add for Vector3d {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         Self {x : self.x + rhs.x, y : self.y + rhs.y, z : self.z + rhs.z}
//     }
// }
//
// impl Sub for Vector3d {
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self::Output {
//         Self {x : self.x - rhs.x, y : self.y - rhs.y, z : self.z - rhs.z}
//     }
// }
//
// impl Div<ValueType> for Vector3d
// {
//     type Output = Self;
//     fn div(self, rhs: ValueType) -> Self::Output {
//         Self::Output {x : self.x / rhs, y : self.y / rhs, z : self.z / rhs}
//     }
// }
//
// impl AddAssign<&Vector3d> for Vector3d {
//     fn add_assign(&mut self, rhs: &Vector3d) {
//         self.x += rhs.x;
//         self.y += rhs.y;
//         self.z += rhs.z;
//     }
// }
//
// impl AddAssign<Vector3d> for Vector3d {
//     fn add_assign(&mut self, rhs: Vector3d) {
//         self.x += rhs.x;
//         self.y += rhs.y;
//         self.z += rhs.z;
//     }
// }
//
// impl SubAssign<&Vector3d> for Vector3d {
//     fn sub_assign(&mut self, rhs: &Vector3d) {
//         self.x -= rhs.x;
//         self.y -= rhs.y;
//         self.z -= rhs.z;
//     }
// }
//
// impl SubAssign<Vector3d> for Vector3d {
//     fn sub_assign(&mut self, rhs: Vector3d) {
//         self.x -= rhs.x;
//         self.y -= rhs.y;
//         self.z -= rhs.z;
//     }
// }
//
// impl MulAssign<ValueType> for Vector3d {
//     fn mul_assign(&mut self, rhs: ValueType) {
//         self.x *= rhs;
//         self.y *= rhs;
//         self.z *= rhs;
//     }
// }
//
// impl DivAssign<ValueType> for Vector3d {
//     fn div_assign(&mut self, rhs: ValueType) {
//         self.x /= rhs;
//         self.y /= rhs;
//         self.z /= rhs;
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::auxiliary::vector::Vector3d;
//
//     #[test]
//     fn length() {
//         {
//             let v = Vector3d::new(1.0f32, 0.0f32, 0.0f32);
//             assert_eq!(v.length(), 1.0f32);
//         }
//         {
//             let v = Vector3d::new(3.0f32, 4.0f32, 12.0f32);
//             assert_eq!(v.length(), 13.0f32);
//         }
//         {
//             let v = Vector3d::new(5.0f32, 12.0f32, 84.0f32);
//             assert_eq!(v.length(), 85.0f32);
//         }
//     }
//
//     #[test]
//     fn add() {
//         {
//             let a = Vector3d::new(1.0, 1.0, 1.0);
//             let b = Vector3d::new(2.0, 2.0, 2.0);
//             let v = a + b;
//             assert!( Vector3d::close( &v, &Vector3d::new(3.0, 3.0, 3.0), std::f32::EPSILON));
//         }
//         {
//             let mut v = Vector3d::new(1.0, 1.0, 1.0);
//             let u = Vector3d::new(1.0, 1.0, 1.0);
//             v += &u;
//             assert!( Vector3d::close( &v, &Vector3d::new(2.0, 2.0, 2.0), std::f32::EPSILON));
//         }
//         {
//             let mut v = Vector3d::new(1.0, 1.0, 1.0);
//             let u = Vector3d::new(1.0, 1.0, 1.0);
//             v += u;
//             assert!( Vector3d::close( &v, &Vector3d::new(2.0, 2.0, 2.0), std::f32::EPSILON));
//         }
//     }
//
//     #[test]
//     fn sub() {
//         {
//             let a = Vector3d::new(1.0, 1.0, 1.0);
//             let b = Vector3d::new(2.0, 2.0, 2.0);
//             let v = a - b;
//             assert!( Vector3d::close( &v, &Vector3d::new(-1.0, -1.0, -1.0), std::f32::EPSILON));
//         }
//         {
//             let mut v = Vector3d::new(1.0, 1.0, 1.0);
//             let u = Vector3d::new(1.0, 1.0, 1.0);
//             v -= &u;
//             assert!( Vector3d::close( &v, &Vector3d::new(0.0, 0.0, 0.0), std::f32::EPSILON));
//         }
//         {
//             let mut v = Vector3d::new(1.0, 1.0, 1.0);
//             let u = Vector3d::new(1.0, 1.0, 1.0);
//             v -= u;
//             assert!( Vector3d::close( &v, &Vector3d::new(0.0, 0.0, 0.0), std::f32::EPSILON));
//         }
//     }
//
//     #[test]
//     fn mul() {
//         {
//             let a = Vector3d::new(1.0, 1.0, 1.0);
//             let v = 3.0*a;
//             assert!( Vector3d::close( &v, &Vector3d::new(3.0, 3.0, 3.0), std::f32::EPSILON));
//         }
//     }
//
//     #[test]
//     fn div() {
//         let a = Vector3d::new(3.0, 3.0, 3.0);
//         let v = a/3.0;
//         assert!( Vector3d::close( &v, &Vector3d::new(1.0, 1.0, 1.0), std::f32::EPSILON));
//     }
// }