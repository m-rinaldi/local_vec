use crate::LocalVec;
use std::iter::{Iterator, IntoIterator};

impl<T, const N: usize> IntoIterator for LocalVec<T, N> {
    type Item = T;

    type IntoIter = LocalVecIter<T>;

    fn into_iter(self) -> LocalVecIter<T> {
        // prevent the compiler from dropping the elements the vector
        // was holding as the onwership must be handed over to the iterator
        let this = std::mem::ManuallyDrop::new(self);

        let begin = this.as_ptr();

        // TODO what if T is a zero-sized type?
        // SAFETY: one byte past the end of the same allocated object
        let end = unsafe {
            begin.add(this.len()) as *const T
        };

        LocalVecIter { 
            ptr: begin,
            end,
        }
    }
}

pub struct LocalVecIter<T> {
    ptr: *const T,
    end: *const T,
}

impl<T> Iterator for LocalVecIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.ptr == self.end {
            // end already reached
            return None
        }

        // TODO
        if std::mem::size_of::<T>() == 0 {
            unimplemented!("zero-size types not yet supported")
        }

        let cur = self.ptr;
        
        // SAFETY: end not yet reached
        self.ptr = unsafe {
            self.ptr.offset(1)
        };

        Some(unsafe { std::ptr::read(cur) })
    }
}

impl<T> Drop for LocalVecIter<T> {
    fn drop(&mut self) {
        // we pay the price even if T doesn't implement Drop
        let _ = self.last();
    }
}