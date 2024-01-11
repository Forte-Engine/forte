use cgmath::Quaternion;
use forte::math::quaternion::QuaternionExt;

fn main() {
    let quat = Quaternion::euler_deg_x(1.0);
    println!("{:?}", quat);
}