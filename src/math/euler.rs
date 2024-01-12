use cgmath::*;

/// A trait to be applied to cgmath::Euler to make the creation of eulers easier.
pub trait EulerDegExt {
    /// Create a new euler using the given degrees on each axis.
    fn deg(x: f32, y: f32, z: f32) -> Euler<Deg<f32>>;

    /// Create a new euler from the given degrees on the x axix.
    fn deg_x(x: f32) -> Euler<Deg<f32>>;

    /// Create a new euler from the given degrees on the y axix.
    fn deg_y(y: f32) -> Euler<Deg<f32>>;

    /// Create a new euler from the given degrees on the z axix.
    fn deg_z(z: f32) -> Euler<Deg<f32>>;
}

/// An implementation of EulerDegExt.  See documenation for more info.
impl EulerDegExt for Euler<Deg<f32>> {
    fn deg(x: f32, y: f32, z: f32) -> Euler<Deg<f32>> { Euler { x: Deg(x), y: Deg(y), z: Deg(z) } }
    fn deg_x(x: f32) -> Euler<Deg<f32>> { Euler { x: Deg(x), y: Deg(0.0), z: Deg(0.0) } }
    fn deg_y(y: f32) -> Euler<Deg<f32>> { Euler { x: Deg(0.0), y: Deg(y), z: Deg(0.0) } }
    fn deg_z(z: f32) -> Euler<Deg<f32>> { Euler { x: Deg(0.0), y: Deg(0.0), z: Deg(z) } }
}

/// A trait to be applied to cgmath::Euler to make the creation of eulers easier.
pub trait EulerRadExt {
    /// Create a new euler using the given radians on each axis.
    fn rad(x: f32, y: f32, z: f32) -> Euler<Rad<f32>>;

    /// Create a new euler from the gievn degrees on the x axis.
    fn rad_x(x: f32) -> Euler<Rad<f32>>;

    /// Create a new euler from the gievn degrees on the y axis.
    fn rad_y(y: f32) -> Euler<Rad<f32>>;

    /// Create a new euler from the gievn degrees on the z axis.
    fn rad_z(z: f32) -> Euler<Rad<f32>>;
}

/// An implementation of EulerRadExt.  See documentation for more info.
impl EulerRadExt for Euler<Rad<f32>> {
    fn rad(x: f32, y: f32, z: f32) -> Euler<Rad<f32>> { Euler { x: Rad(x), y: Rad(y), z: Rad(z) } }
    fn rad_x(x: f32) -> Euler<Rad<f32>> { Euler { x: Rad(x), y: Rad(0.0), z: Rad(0.0) } }
    fn rad_y(y: f32) -> Euler<Rad<f32>> { Euler { x: Rad(0.0), y: Rad(y), z: Rad(0.0) } }
    fn rad_z(z: f32) -> Euler<Rad<f32>> { Euler { x: Rad(0.0), y: Rad(0.0), z: Rad(z) } }
}
