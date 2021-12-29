use crate::LocalVec;
use std::iter::Extend;

impl<T, const N: usize> Extend<T> for LocalVec<T, N> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        for elem in iter {
            self.push(elem);
        }
    }
}