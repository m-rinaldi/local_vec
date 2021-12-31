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

#[cfg(test)]
mod tests {
    use super::LocalVec;

    #[test]
    fn test_iter() {
        let vec = LocalVec::<_, 3>::from_array([1, 2, 3]);
        let mut iter = vec.into_iter();
        matches!(iter.next(), Some(1));
        matches!(iter.next(), Some(2));
        matches!(iter.next(), Some(3));
        matches!(iter.next(), None);
        matches!(iter.next(), None);
    }

    #[test]
    fn test_iter_empty() {
        let vec = LocalVec::<usize, 2>::new();
        let mut iter = vec.into_iter();
        matches!(iter.next(), None);
        matches!(iter.next(), None);
    }

    #[test]
    fn test_iter_zero_cap() {
        let vec = LocalVec::<usize, 0>::new();
        let mut iter = vec.into_iter();
        matches!(iter.next(), None);
        matches!(iter.next(), None);
    }
}