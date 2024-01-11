use cgmath::*;

pub trait EulerDegExt {
    fn deg(x: f32, y: f32, z: f32) -> Euler<Deg<f32>>;
    fn deg_x(x: f32) -> Euler<Deg<f32>>;
    fn deg_y(y: f32) -> Euler<Deg<f32>>;
    fn deg_z(z: f32) -> Euler<Deg<f32>>;
}

impl EulerDegExt for Euler<Deg<f32>> {
    fn deg(x: f32, y: f32, z: f32) -> Euler<Deg<f32>> { Euler { x: Deg(x), y: Deg(y), z: Deg(z) } }
    fn deg_x(x: f32) -> Euler<Deg<f32>> { Euler { x: Deg(x), y: Deg(0.0), z: Deg(0.0) } }
    fn deg_y(y: f32) -> Euler<Deg<f32>> { Euler { x: Deg(0.0), y: Deg(y), z: Deg(0.0) } }
    fn deg_z(z: f32) -> Euler<Deg<f32>> { Euler { x: Deg(0.0), y: Deg(0.0), z: Deg(z) } }
}

pub trait EulerRadExt {
    fn rad(x: f32, y: f32, z: f32) -> Euler<Rad<f32>>;
    fn rad_x(x: f32) -> Euler<Rad<f32>>;
    fn rad_y(y: f32) -> Euler<Rad<f32>>;
    fn rad_z(z: f32) -> Euler<Rad<f32>>;
}

impl EulerRadExt for Euler<Rad<f32>> {
    fn rad(x: f32, y: f32, z: f32) -> Euler<Rad<f32>> { Euler { x: Rad(x), y: Rad(y), z: Rad(z) } }
    fn rad_x(x: f32) -> Euler<Rad<f32>> { Euler { x: Rad(x), y: Rad(0.0), z: Rad(0.0) } }
    fn rad_y(y: f32) -> Euler<Rad<f32>> { Euler { x: Rad(0.0), y: Rad(y), z: Rad(0.0) } }
    fn rad_z(z: f32) -> Euler<Rad<f32>> { Euler { x: Rad(0.0), y: Rad(0.0), z: Rad(z) } }
}
