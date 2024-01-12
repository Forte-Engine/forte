use cgmath::*;
use crate::math::euler::*;

/// A trait to make the creation of cgmath::Quaternion easier.
pub trait QuaternionExt {
    /// Create a new quaternion using a euler rotation with all three axis' using degrees.
    fn euler_deg(x: f32, y: f32, z: f32) -> Quaternion<f32>;

    /// Create a new quaternion using a euler rotation on the x axis.
    fn euler_deg_x(x: f32) -> Quaternion<f32>;
    
    /// Create a new quaternion using a euler rotation on the y axis.
    fn euler_deg_y(y: f32) -> Quaternion<f32>;

    /// Create a new quaternion using a euler rotation on the z axis.
    fn euler_deg_z(z: f32) -> Quaternion<f32>;

    /// Create a new quaternion using a euler rotation with all three axis' using radians.
    fn euler_rad(x: f32, y: f32, z: f32) -> Quaternion<f32>;

    /// Create a new quaternion using a euler radian rotation of the x axis.
    fn euler_rad_x(x: f32) -> Quaternion<f32>;

    /// Create a new quaternion using a euler radian rotation of the y axis.
    fn euler_rad_y(y: f32) -> Quaternion<f32>;
    
    /// Create a new quaternion using a euler radian rotation of the z axis.
    fn euler_rad_z(z: f32) -> Quaternion<f32>;
}

/// An implementation of QuaternionExt.  See documentation for more info.
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