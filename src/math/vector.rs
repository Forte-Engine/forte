use cgmath::*;

/// A trait with a set of functions to make working with cgmath vectors easier.
pub trait VectorExt<T: BaseFloat> {
    /// Get the squared length of the vector.
    fn length_sq(&self) -> T;

    /// Get the length of the vector.
    fn length(&self) -> T;

    /// Convert this vector a euler rotation.
    fn euler(&self) -> Euler<Deg<T>>;
}

/// An implementation of VectorExt for cgmath::Vector3.  See documentation for more information.
impl <T: BaseFloat> VectorExt<T> for Vector3<T> {
    fn length(&self) -> T { T::sqrt(self.length_sq()) }
    fn length_sq(&self) -> T { self.x * self.x + self.y * self.y + self.z * self.z }
    fn euler(&self) -> Euler<Deg<T>> { Euler { x: Deg(self.x), y: Deg(self.y), z: Deg(self.z) } }
}