use std::convert::From;
use crate::LocalVec;

impl<T, const N: usize> From<LocalVec<T, N>> for [T; N] {
    fn from(mut local_vec: LocalVec<T, N>) -> Self {
        let arr: [T; N] = unsafe {
            local_vec.set_len(0);
            std::mem::transmute_copy(&local_vec.buf)
        };

        arr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_into_array() {
        let mut vec = LocalVec::<_, 4>::new();
        vec.push(0);
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let arr: [_; 4] = vec.into();
        assert_eq!(arr[0], 0);
        assert_eq!(arr[1], 1);
        assert_eq!(arr[2], 2);
        assert_eq!(arr[3], 3);
    }
}