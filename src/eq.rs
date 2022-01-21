use crate::{LocalVecImpl, LenType};

// TODO generalize further for comparing LocalVec values of different capacities
impl<T: PartialEq, const N: LenType> PartialEq for LocalVecImpl<T, N> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        for i in 0..self.len() {
            if self[i] != other[i] {
                return false;
            }
        }

        true
    }
}

impl<T: Eq, const N: LenType> Eq for LocalVecImpl<T, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_equal() {
        let vec = LocalVecImpl::<_, 3>::from_array([1, 2, 3]);
        let cloned = vec.clone();
        assert!(vec == cloned);
    }

    #[test]
    fn test_eq_not_equal() {
        let a = LocalVecImpl::<_, 3>::from_array([1, 2, 3]);
        let b = LocalVecImpl::<_, 3>::from_array([1, 2, 5]);
        assert!(a != b);
    }
    
    #[test]
    fn test_eq_diff_len() {
        let a = LocalVecImpl::<_, 4>::from_array([1, 2, 3]);
        let b = LocalVecImpl::<_, 4>::from_array([1, 2, 3, 4]);
        assert!(a != b);
    }
}