use crate::LocalVec;

impl<T, const N: usize> std::ops::Index<usize> for LocalVec<T, N> {
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

impl<T, const N: usize> std::ops::IndexMut<usize> for LocalVec<T, N> {
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
    use crate::LocalVec;

    #[test]
    fn test_index() {
        let mut vec = LocalVec::<_, 3>::new();

        vec.push(0);
        vec.push(1);
        vec.push(2);

        assert_eq!(vec[0], 0);
        assert_eq!(vec[1], 1);
        assert_eq!(vec[2], 2);
    }

    #[test]
    fn test_index_mut() {
        let mut vec = LocalVec::<_, 3>::new();

        vec.push(0);
        vec.push(1);
        vec.push(2);

        assert_eq!(vec[0], 0);
        assert_eq!(vec[1], 1);
        assert_eq!(vec[2], 2);

        vec[0] += 1;
        assert_eq!(vec[0], 1);

        vec[1] += 1;
        assert_eq!(vec[1], 2);

        vec[2] += 1;
        assert_eq!(vec[2], 3);
    }
}



