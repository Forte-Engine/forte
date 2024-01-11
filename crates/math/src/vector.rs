use cgmath::*;

pub trait VectorExt<T: BaseFloat> {
    fn length_sq(&self) -> T;
    fn length(&self) -> T;
    fn euler(&self) -> Euler<Deg<T>>;
}

impl <T: BaseFloat> VectorExt<T> for Vector3<T> {
    fn length(&self) -> T { T::sqrt(self.length_sq()) }
    fn length_sq(&self) -> T { self.x * self.x + self.y * self.y + self.z * self.z }
    fn euler(&self) -> Euler<Deg<T>> { Euler { x: Deg(self.x), y: Deg(self.y), z: Deg(self.z) } }
}