So far, we've been taking inputs from Python, transforming them in Rust, and then returning them back to Python in another form. In Rust, these would generally be accomplished by reading from aliased borrows `&T`.

But sometimes, an `&T` isn't enough! Sometimes, you've gotta mutate in place with `&mut T`. 

This distinction doesn't exist in Python: all objects are effectively `Box<T>`, passed around by reference, and all references are non-exclusive mutable references. By default, PyO3 will try to maintain Rust's aliasing guarantees across the two languages by making reference-counted copies of Python data, operating on those copies rather than the original: the following code, for example:

```rust
use pyo3::prelude::*;

#[pyfunction]
fn example_function(
    example_vec: Vec<u32>,
) -> Vec<u32> {
    example_vec.iter().map(|i| 2 * i).collect()
}
```

Is operating on a copy of `example_vec`, not the original, and returns a newly-allocated `Vec<u32>`. Even if we alter its signature to be mutable:

```rust
use pyo3::prelude::*;

#[pyfunction]
fn example_function_mut(
    mut example_vec: Vec<u32>,
) -> Vec<u32> {
    example_vec.iter_mut().map(|i| 2 * *i).collect()
}
```

***
CHECK
***
It avoids allocating the second vector by directly mutating the first, but we're still working on and returning a copy. We're not mutating the original `example_vec` in Python memory.

But what if we really do want to mutate Python-held memory in-place?

That's where the most prominent of PyO3's custom smart pointers comes in: `Bound<'py, T>`. 

`Bound<'py, T>` can look pretty clunky and odd when you're first working with it, but it's actually quite a friendly and ergonomic fellow, bundling a lot of guarantees. 

`Bound<'py, T>` is attached to the lifetime of the Python interpreter, `'py`. A `Bound<'py, T>` element can only exist if the Python interpreter is running and this Rust code holds the Global Interpreter Lock (GIL). 

These guarantees allow us to safely and reliably interact with Python memory directly, allowing behavior and optimizations that would not be possible otherwise. 

How would we use this to mutate Python memory in place, as we tried to above? 

*** 
DOUBLE CHECK that this works as stated

```rust
use pyo3::prelude::*;

#[pyclass]
struct Foo {
    bar: u32
}

#[pyfunction]
fn example_function_mut(
    example_vec: Vec<Bound<'_, Foo>>,
) {
    // iterate through the pointers
    example_vec.iter().for_each(|i_handle| {
        // turn the Bound<'_, u32> into a PyRefMut<'_, u32>
        let mut i_ref = i_handle.borrow_mut();
        // act on the mutable reference directly, safely
        i_ref.bar*=2;
    });
}
```

Note that in this rendition, the `example_vec` argument doesn't even need to be listed as mut: neither the vector nor the smart pointers themselves are being mutated, just the memory they point to: yet we remain explicit by calling `.borrow_mut` on the handle.

You'll also note that we're using a Vec of smart pointers, which invites a great deal of pointer chasing, as opposed to working directly on arrays in contiguous memory. 

Because Python does not have native support for contiguous-memory arrays, we would need to be a bit more sophisticated: in practice, the solution would be to install the `numpy` crate and use its `PyArray` types to read from, mutate, and reate numpy arrays from Rust. Without `numpy`, we could also use a `Bound<'_ PyList>`, PyO3's built-in type for representing native Python lists. However, Python lists are themselves functionally arrays of smart pointers... so we shouldn't expect a real performance gain from this.





***
Why can't this be done on a raw Bound<'py, u32>????







