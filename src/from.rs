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