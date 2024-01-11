use cgmath::*;
use crate::math::euler::*;

pub trait QuaternionExt {
    fn euler_deg(x: f32, y: f32, z: f32) -> Quaternion<f32>;
    fn euler_deg_x(x: f32) -> Quaternion<f32>;
    fn euler_deg_y(y: f32) -> Quaternion<f32>;
    fn euler_deg_z(z: f32) -> Quaternion<f32>;
    fn euler_rad(x: f32, y: f32, z: f32) -> Quaternion<f32>;
    fn euler_rad_x(x: f32) -> Quaternion<f32>;
    fn euler_rad_y(y: f32) -> Quaternion<f32>;
    fn euler_rad_z(z: f32) -> Quaternion<f32>;
}

impl QuaternionExt for Quaternion<f32> {
    fn euler_deg(x: f32, y: f32, z: f32) -> Quaternion<f32> { Euler::deg(x, y, z).into() }
    fn euler_deg_x(x: f32) -> Quaternion<f32> { Euler::deg_x(x).into() }
    fn euler_deg_y(y: f32) -> Quaternion<f32> { Euler::deg_y(y).into() }
    fn euler_deg_z(z: f32) -> Quaternion<f32> { Euler::deg_z(z).into() }
    fn euler_rad(x: f32, y: f32, z: f32) -> Quaternion<f32> { Euler::rad(x, y, z).into() }
    fn euler_rad_x(x: f32) -> Quaternion<f32> { Euler::rad_x(x).into() }
    fn euler_rad_y(y: f32) -> Quaternion<f32> { Euler::rad_y(y).into() }
    fn euler_rad_z(z: f32) -> Quaternion<f32> { Euler::rad_z(z).into() }
}