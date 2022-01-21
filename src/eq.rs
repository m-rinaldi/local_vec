use crate::{LocalVec, LenType};

// TODO generalize further for comparing LocalVec values of different capacities
impl<T: PartialEq, const N: LenType> PartialEq for LocalVec<T, N> {
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

impl<T: Eq, const N: LenType> Eq for LocalVec<T, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_equal() {
        let vec = LocalVec::<_, 3>::from_array([1, 2, 3]);
        let cloned = vec.clone();
        assert!(vec == cloned);
    }

    #[test]
    fn test_eq_not_equal() {
        let a = LocalVec::<_, 3>::from_array([1, 2, 3]);
        let b = LocalVec::<_, 3>::from_array([1, 2, 5]);
        assert!(a != b);
    }
    
    #[test]
    fn test_eq_diff_len() {
        let a = LocalVec::<_, 4>::from_array([1, 2, 3]);
        let b = LocalVec::<_, 4>::from_array([1, 2, 3, 4]);
        assert!(a != b);
    }
}