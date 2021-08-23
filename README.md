# `stack_vec`

A *fixed-capacity* vector allocated on the stack.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://app.travis-ci.com/m-rinaldi/stack_vec.svg?branch=main)](https://app.travis-ci.com/m-rinaldi/stack_vec)

---

`stack_vec::StackVec` is a *fixed-capacity* vector, i.e., its *size* or *length* increases and decreases as elements are pushed into and popped from the vector, respectively. However, its *capacity* remains always the same.

`StackVec`'s elements reside inside it:

    use stack_vec::StackVec;
    let mut vec = StackVec::<_, 4>::new();
    vec.push(3);
    vec.push(7);
    
`vec` contents in the code above are graphically represented as:

<p align="center">
  <img src="img/StackVec.png">
</p>


In contrast, [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) allocates a buffer on the heap and contains a pointer to that buffer instead of the buffer itself.


The capacity of a `StackVec` must be determined at compile-time as a constant argument.

