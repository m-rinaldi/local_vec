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
    struct CounterGuard(*mut u8);

    impl<'a> CounterGuard {
        pub fn new(cnt: &'a mut u8) -> CounterGuard {
            *cnt += 1;
            CounterGuard(cnt as *mut u8)
        }
    }

    impl<'a> Drop for CounterGuard {
        fn drop(&mut self) {
            unsafe {
                *self.0 -= 1;
            }
        }
    }

    use crate::LocalVec;

    #[test]
    fn test_drop() {
        let mut cnt = 0u8;
        let mut buf = LocalVec::<_, 3>::new();

        assert_eq!(cnt, 0);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 1);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 2);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 3);

        std::mem::drop(buf);
        assert_eq!(cnt, 0);
    }

    #[test]
    fn test_drop_after_set_len() {
        let mut cnt = 0u8;
        let mut buf = LocalVec::<_, 3>::new();

        assert_eq!(cnt, 0);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 1);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 2);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 3);

        unsafe {
            buf.set_len(1);
        }

        std::mem::drop(buf);
        assert_eq!(cnt, 2);
    }

    #[test]
    fn test_drop_after_into_array() {
        let mut cnt = 0u8;
        let mut buf = LocalVec::<_, 3>::new();

        assert_eq!(cnt, 0);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 1);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 2);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 3);

        let arr: [CounterGuard; 3] = buf.try_into().unwrap();
        assert_eq!(cnt, 3);

        std::mem::drop(arr);
        assert_eq!(cnt, 0);
    }

    #[test]
    fn test_drop_after_take_array() {
        let mut cnt = 0u8;
        let mut buf = LocalVec::<_, 3>::new();

        assert_eq!(cnt, 0);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 1);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 2);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 3);

        let arr = unsafe { buf.take_array() };
        assert_eq!(buf.len(), 0);
        assert_eq!(cnt, 3);

        std::mem::drop(arr);
        assert_eq!(cnt, 0);
    }
}
