/// A trait to be implemented for vectors to make our lives easier.
pub trait VecExt<T> {
    /// Loop through each element of the vector and call the callback on each using a reference.
    fn for_each<F>(&self, callback: F) where F: Fn(&T);

    /// Loop throuh each element of the vector and call the callback on each using a mutable reference.
    fn for_each_mut<F>(&mut self, callback: F) where F: Fn(&mut T);
}

/// An implementation of VecExt for all vectors.
impl <T> VecExt<T> for Vec<T> {
    fn for_each<F>(&self, f: F) where F: Fn(&T) { self.iter().for_each(f); }
    fn for_each_mut<F>(&mut self, f: F) where F: Fn(&mut T) { self.iter_mut().for_each(f); }
}
