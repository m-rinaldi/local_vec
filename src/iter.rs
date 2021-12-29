use crate::LocalVec;
use std::iter::{Iterator, IntoIterator};

impl<T, const N: usize> IntoIterator for LocalVec<T, N> {
    type Item = T;

    type IntoIter = LocalVecIter<T, N>;

    fn into_iter(self) -> LocalVecIter<T, N> {
        let mut vec = self;
        vec.reverse();
        
        LocalVecIter {
            elems: vec,
        }
    }
}

pub struct LocalVecIter<T, const N: usize> {
    elems: LocalVec<T, N>,
}

impl<T, const N: usize> Iterator for LocalVecIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.elems.pop()
    }
}