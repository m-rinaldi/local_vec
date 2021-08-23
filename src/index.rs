use crate::StackVec;

impl<T, const N: usize> std::ops::Index<usize> for StackVec<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        if idx >= self.len {
            panic!("out of bounds access");
        }

        let ptr = self.buf.index(idx).as_ptr();

        unsafe {
            &*ptr
        }
    }
}

impl<T, const N: usize> std::ops::IndexMut<usize> for StackVec<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if idx >= self.len {
            panic!("out of bounds access");
        }

        let ptr = self.buf.index_mut(idx).as_mut_ptr();

        unsafe {
            &mut *ptr
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StackVec;

    #[test]
    fn test_index() {
        let mut vec = StackVec::<_, 3>::new();

        vec.push(0);
        vec.push(1);
        vec.push(2);

        assert_eq!(vec[0], 0);
        assert_eq!(vec[1], 1);
        assert_eq!(vec[2], 2);
    }
}



