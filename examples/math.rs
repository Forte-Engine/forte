use cgmath::Quaternion;
use forte_engine::math::quaternion::QuaternionExt;

fn main() {
    let quat = Quaternion::euler_deg_x(1.0);
    println!("{:?}", quat);
}