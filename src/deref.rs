use crate::LocalVec;

impl<T, const N: usize> std::ops::Deref for LocalVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}
