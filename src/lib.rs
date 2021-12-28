use std::mem::MaybeUninit;

mod drop;
mod index;
mod from;
mod deref;
mod iter;

#[derive(Debug)]
/// A fixed-capacity vector that directly stores its elements  
pub struct LocalVec<T, const N: usize> {
    buf: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> LocalVec<T, N> {
    pub fn new() -> Self {
        let buf: [MaybeUninit<T>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        LocalVec {
            buf,
            len: 0,
        }
    }

    // TODO implement From<[T; N]> on top of this?
    pub fn from_array<const M: usize>(arr: [T; M]) -> Self {
        // TODO check at compile time
        assert!(M <= N, "can't store {} elements with a capacity of {}", M, N);

        // TODO rewrite with Extend::extends()
        let mut vec = Self::new();
        for elem in arr {
            vec.push(elem);
        }

        vec
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

    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    pub fn capacity(&self) -> usize {
        N
    }

    /// panics if the vector is full
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

    #[must_use = "consider using clear() instead"]
    /// steal the elements stored
    pub fn take_array(&mut self) -> [T; N] {
        let arr: [T; N] = unsafe {
            self.set_len(0);
            std::mem::transmute_copy(&self.buf)
        };
        arr
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        if N == 0 {
            return std::ptr::null();
        }

        let ptr = self.buf[0].as_ptr();
        assert!(!ptr.is_null());
        ptr
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        if N == 0 {
            return std::ptr::null_mut();
        }

        let ptr = self.buf[0].as_mut_ptr();
        assert!(!ptr.is_null());
        ptr
    }
}

#[cfg(test)]
mod tests {
    use super::LocalVec;

    #[test]
    fn test_new() {
        let vec = LocalVec::<u32, 4>::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 4);
    }

    #[test]
    #[should_panic]
    fn test_push_on_full() {
        let mut vec = LocalVec::<_, 1>::new();
        vec.push(0);

        assert!(vec.is_full());
        // this one should panic
        vec.push(0);
    }

    #[test]
    fn test_push() {
        let mut vec = LocalVec::<_, 3>::new();

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
        let mut vec = LocalVec::<_, 1>::new();
        assert!(vec.is_empty());
        matches!(vec.pop(), None);

        vec.push(0);
        // not empty anymore
        matches!(vec.pop(), Some(1));

        // empty again
        matches!(vec.pop(), None);
    }

    #[test]
    fn test_push_and_pop() {
        let mut vec = LocalVec::<_, 4>::new();
        assert!(vec.is_empty());
        matches!(vec.pop(), None);

        vec.push(0);
        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), Some(0));
    }

    #[test]
    fn test_clear() {
        let mut vec = LocalVec::<_,3>::new();
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

    #[test]
    #[should_panic]
    fn test_from_capacity_exceeding_array() {
        let arr = [0; 4];
        let _ = LocalVec::<_,3>::from_array(arr);
    }

    #[test]
    fn test_from_array() {
        let arr = [0; 4];
        let vec = LocalVec::<_, 4>::from_array(arr);

        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_from_smaller_array() {
        let arr = [0; 4];
        let vec = LocalVec::<_, 6>::from_array(arr);

        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_set_len() {
        let arr = [7; 4];
        let mut vec = LocalVec::<_, 6>::from_array(arr);

        assert_eq!(vec.len(), 4);
        unsafe {
            vec.set_len(1);
        }
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_take_array() {
        let arr = [7; 4];
        let mut vec = LocalVec::<_, 6>::from_array(arr);
        assert_eq!(vec.len(), 4);
        let _ = vec.take_array();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_as_ptr() {
        let arr = [0xff; 3];
        let vec = LocalVec::<_, 8>::from_array(arr);
        let ptr = vec.as_ptr();
        assert_eq!(ptr, &vec[0] as *const i32);
    }

    #[test]
    fn test_as_mut_ptr() {
        let arr = [0xff; 3];
        let mut vec = LocalVec::<_, 8>::from_array(arr);
        let ptr = vec.as_mut_ptr();
        assert_eq!(ptr, &mut vec[0] as *mut i32);
    }

    #[test]
    fn test_as_ptr_zero_size() {
        let vec = LocalVec::<u8, 0>::new();
        let ptr = vec.as_ptr();
        assert!(ptr.is_null());
    }

    #[test]
    fn test_as_mut_ptr_zero_size() {
        let mut vec = LocalVec::<u8, 0>::new();
        let ptr = vec.as_mut_ptr();
        assert!(ptr.is_null());
    }
}
