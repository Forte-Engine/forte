pub trait VecExt<T> {
    fn for_each<F>(&self, callback: F) where F: Fn(&T);
    fn for_each_mut<F>(&mut self, callback: F) where F: Fn(&mut T);
}

impl <T> VecExt<T> for Vec<T> {
    fn for_each<F>(&self, f: F) where F: Fn(&T) { self.iter().for_each(f); }
    fn for_each_mut<F>(&mut self, f: F) where F: Fn(&mut T) { self.iter_mut().for_each(f); }
}
