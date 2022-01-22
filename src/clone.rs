use crate::LocalVecImpl;

impl<T: Clone, const N: usize> Clone for LocalVecImpl<T, N> {
    fn clone(&self) -> Self {
        let mut cloned = Self::new();
        for i in 0..self.len() {
            cloned.push(self[i].clone());
        }
        debug_assert_eq!(cloned.len(), self.len());
        cloned
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let vec = LocalVecImpl::<_, 3>::from_array([1, 2, 3]);
        let cloned = vec.clone();
        assert_eq!(vec, cloned);
    }
}