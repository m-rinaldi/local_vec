# `local_vec`

A *fixed-capacity* vector whose elements stored *locally* – it can be on the [stack](#allocating-on-the-stack-or-the-heap).

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://app.travis-ci.com/m-rinaldi/local_vec.svg?branch=main)](https://app.travis-ci.com/m-rinaldi/local_vec)

---

`local_vec::LocalVec` is a *fixed-capacity* vector, i.e., its *size* or *length* increases and decreases as elements are pushed into and popped from the vector, respectively. However, its *capacity* remains always the same and must be determined at compile time.

---

### `LocalVec` vs `Vec`

`LocalVec`'s elements reside locally, i.e., inside it:

    use local_vec::LocalVec;
    let mut vec = LocalVec::<_, 4>::new();
    vec.push(3);
    vec.push(7);
    
`vec` contents in the code above are:

<p align="center">
  <img src="img/LocalVec.png">
</p>

That is, the `i32` values `3` and `7` are stored inside `vec`, not remotelly allocated on the heap.


In contrast, [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) allocates a buffer on the heap and contains a pointer to that buffer instead of the buffer itself:

    let mut vec = Vec::with_capacity(4);
    vec.extend([3, 7]);


The capacity of a `LocalVec` must be determined at compile-time as a constant argument thanks to *const generics*.

---

### Allocating on the Stack or the Heap

Technically, the elements `LocalVec` contains are *locally* stored in the `LocalVec` itself. Whether these elements are on the stack or not, depends on whether the `LocalVec` itself is allocated on the stack. For example:

# TODO


`vec` is allocated on the heap and so are the elements it contains. They are stored inside `vec` itself, though.


