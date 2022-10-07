use crate::LocalVec;

impl <T, const N: usize> Drop for LocalVec<T, N> {
    fn drop(&mut self) {
        // TODO check the dropping order
        // it should be the same as in a built-in array
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::LocalVec;
    use std::rc::Rc;

    #[derive(Clone)]
    struct Counter(Rc<()>);

    impl Counter {
        fn new() -> Self {
            Self(Rc::new(()))
        }

        fn count(&self) -> usize {
            Rc::strong_count(&self.0)
        }
    }

    #[test]
    fn test_drop() {
        let cnt = Counter::new();
        let mut buf = LocalVec::<_, 3>::new();

        assert_eq!(cnt.count(), 1);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 2);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 3);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 4);

        std::mem::drop(buf);
        assert_eq!(cnt.count(), 1);
    }

    #[test]
    fn test_drop_after_set_len() {
        let cnt = Counter::new();
        let mut buf = LocalVec::<_, 3>::new();

        assert_eq!(cnt.count(), 1);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 2);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 3);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 4);

        unsafe {
            buf.set_len(1);
        }

        std::mem::drop(buf);
        assert_eq!(cnt.count(), 3);
    }

    #[test]
    fn test_drop_after_into_array() {
        let cnt = Counter::new();
        let mut buf = LocalVec::<_, 3>::new();

        assert_eq!(cnt.count(), 1);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 2);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 3);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 4);

        let arr: [Counter; 3] = buf.try_into().unwrap();
        assert_eq!(cnt.count(), 4);

        std::mem::drop(arr);
        assert_eq!(cnt.count(), 1);
    }

    #[test]
    fn test_drop_after_take_array() {
        let cnt = Counter::new();
        let mut buf = LocalVec::<_, 3>::new();

        assert_eq!(cnt.count(), 1);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 2);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 3);

        buf.push(cnt.clone());
        assert_eq!(cnt.count(), 4);

        let arr = unsafe { buf.take_array() };
        assert_eq!(buf.len(), 0);
        assert_eq!(cnt.count(), 4);

        std::mem::drop(arr);
        assert_eq!(cnt.count(), 1);
    }
}
