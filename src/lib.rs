use std::mem::MaybeUninit;


#[derive(Debug)]
/// A fixed-capacity vector that directly stores its elements  
pub struct StackVec<T, const N: usize> {
    buf: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> StackVec<T, N> {
    pub fn new() -> Self {
        let buf: [MaybeUninit<T>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        StackVec {
            buf,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == self.len
    }

    pub fn is_full(&self) -> bool {
        self.capacity() == self.len
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        N
    }

    /// It panics if the vector is full
    pub fn push(&mut self, val: T) {
        if self.len >= N {
            panic!("capacity excedeed");
        }

        // there is still room available
        self.buf[self.len] = MaybeUninit::new(val);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let val = unsafe {
            std::mem::transmute_copy(&self.buf[self.len-1])
        };
        self.len -= 1;
        Some(val)
    }

    pub fn clear(&mut self) {
        while let Some(_) = self.pop() {
        }
        debug_assert_eq!(self.len, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::StackVec;

    #[test]
    fn test_new() {
        let vec = StackVec::<u32, 4>::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 4);
    }

    #[test]
    #[should_panic]
    fn test_push_on_full() {
        let mut vec = StackVec::<_, 1>::new();
        vec.push(0);

        assert!(vec.is_full());
        // this one should panic
        vec.push(0);
    }

    #[test]
    fn test_push() {
        let mut vec = StackVec::<_, 3>::new();

        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);

        vec.push(0);
        assert_eq!(vec.len(), 1);

        vec.push(0);
        assert_eq!(vec.len(), 2);

        vec.push(0);
        assert_eq!(vec.len(), 3);
        assert!(vec.is_full());
    }

    #[test]
    fn test_pop_on_empty() {
        let mut vec = StackVec::<_, 1>::new();
        assert!(vec.is_empty());
        matches!(vec.pop(), None);

        vec.push(0);
        // not empty anymore
        matches!(vec.pop(), Some(1));

        // empty again
        matches!(vec.pop(), None);
    }

    #[test]
    fn test_clear() {
        let mut vec = StackVec::<_,3>::new();
        vec.clear();
        assert!(vec.is_empty());

        vec.push(0);
        assert!(!vec.is_empty());

        vec.push(0);
        vec.push(0);
        assert!(vec.is_full());

        vec.clear();
        assert!(vec.is_empty());
    }
}
