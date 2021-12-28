use crate::LocalVec;
use std::ops::Deref;

impl<T, const N: usize> Deref for LocalVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
           std::slice::from_raw_parts(self.as_ptr(), self.len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let arr = [33; 3];
        let vec = LocalVec::<_, 8>::from_array(arr);
        let slc = vec.deref();
        assert_eq!(slc.len(), 3);
        assert_eq!(slc, arr);
    }

    #[test]
    fn test_deref_zero_size() {
        let vec = LocalVec::<u8, 0>::new();
        let slc = vec.deref();
        assert_eq!(slc.len(), 0);
    }
}