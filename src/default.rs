use crate::{LocalVec, CopyLocalVec};

impl<T, const N: usize> Default for LocalVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy, const N: usize> Default for CopyLocalVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}