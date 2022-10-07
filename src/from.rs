use crate::LocalVec;
use std::convert::{From, TryFrom};
use std::mem::MaybeUninit;

impl<T, const N: usize> From<LocalVec<T, N>> for [MaybeUninit<T>; N] {
    fn from(mut local_vec: LocalVec<T, N>) -> Self {
        local_vec.take_uninit_array()
    }
}

#[derive(Debug)]
pub struct NotFull;

impl std::fmt::Display for NotFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("array is not full")
    }
}

impl std::error::Error for NotFull {}

impl<T, const N: usize> TryFrom<LocalVec<T, N>> for [T; N] {
    type Error = NotFull;

    fn try_from(mut local_vec: LocalVec<T, N>) -> Result<Self, Self::Error> {
        if !local_vec.is_full() {
            return Err(NotFull);
        }
        // Safety: checked for is_full before.
        Ok(unsafe { local_vec.take_array() })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_into_array() {
        let mut vec = LocalVec::<u32, 4>::new();
        vec.push(0);
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let arr: [u32; 4] = vec.try_into().unwrap();
        assert_eq!(arr[0], 0);
        assert_eq!(arr[1], 1);
        assert_eq!(arr[2], 2);
        assert_eq!(arr[3], 3);
    }
}
