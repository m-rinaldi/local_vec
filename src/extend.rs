use crate::LocalVec;
use std::iter::Extend;

impl<T, const N: usize> Extend<T> for LocalVec<T, N> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        for elem in iter {
            self.push(elem);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend() {
        let mut vec = LocalVec::<_, 4>::new();
        let arr = [1, 2, 3, 4];
        vec.extend(arr);

        matches!(vec.pop(), Some(4));
        matches!(vec.pop(), Some(3));
        matches!(vec.pop(), Some(2));
        matches!(vec.pop(), Some(1));
        matches!(vec.pop(), None);
    }


    #[test]
    #[should_panic]
    fn test_extend_exceeding() {
        let mut vec = LocalVec::<_, 3>::new();
        let arr = [1, 2, 3, 4];
        vec.extend(arr);
    }
}
