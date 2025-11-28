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

Is operating on a copy of `example_vec`, not the original, and returns a newly-allocated `Vec<u32>`. Even if we alter its signature to be mutable and modify the local binding in place:

```rust
use pyo3::prelude::*;

#[pyfunction]
fn example_function_mut(
    mut example_vec: Vec<u32>,
) -> Vec<u32> {
    example_vec.iter_mut().for_each(|x| *x *= 2);
    example_vec
}
```

we still end up copying the Python data when it crosses into Rust, and copying the Rust data when it crosses back into Python: this *does not* mutate the original Python data.

But what if we really do want to mutate Python-held memory in-place?

That's where the most prominent of PyO3's custom smart pointers comes in: `Bound<'py, T>`. 

`Bound<'py, T>` can look pretty clunky and odd when you're first working with it, but it's actually quite a friendly and ergonomic fellow, bundling a lot of guarantees. 

`Bound<'py, T>` is attached to the lifetime of the Python interpreter, `'py`. A `Bound<'py, T>` element can only exist if the Python interpreter is running and this Rust code holds the Global Interpreter Lock (GIL). 

These guarantees allow us to safely and reliably interact with Python memory directly, allowing behavior and optimizations that would not be possible otherwise. 

How would we use this to mutate Python memory in place, as we tried to above? 

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

A few notes on this implementation:

First, the `example_vec` argument doesn't even need to be listed as mut: neither the vector nor the smart pointers themselves are being mutated, just the memory they point to: yet we remain explicit by calling `.borrow_mut` on the handle.

Second, you'll note that this still isn't zero-copy: we've avoided a second internal vector allocation, and we no longer need to copy the returned data on the Rust->Python crossing (since we're no longer returning anything at all), but we're still copying all those smart pointers from a Python list into a newly-allocated Rust vector. In addition, the fact that we're operating on smart pointers means that we're not getting the benefit of cache locality, and we lose a lot of performance on pointer-chasing.

Because Python does not have native support for contiguous-memory arrays, we would need to be a bit more sophisticated: in practice, the solution would be to install the `numpy` crate and use its `PyArray` types to read from, mutate, and create numpy arrays from Rust. Without `numpy`, we could also use a `PyList`, PyO3's built-in type for representing native Python lists. However, Python lists are themselves functionally arrays of smart pointers, so we still would not get the benefits of contiguous memory from it, only the benefit of avoiding data copying when entering the function.

## Exercise

Now let's put this into practice in our codebase:

```rust
// src/paper.rs
#[pymethods]
impl Paper {
    pub fn fax(
        &self,
        employees: Vec<Bound<'_, Employee>>, 
        emails_opt: Option<Vec<String>>
    )
}
```

We'll create a `fax` pymethod on the Paper struct with the above signature. We'll also use the `signature` attribute to make the optional `emails` input into a key-word argument. 

For those employees whose emails are provided (or if no emails are provided, for all employees), modify the employee data in place using the `employee.send()` method in `employees.rs`. This will add the contents of the `Paper` to the employee's inbox. Make sure that it satisfries both the following interfaces

```python
# send to employees with listed emails
paper.fax(employees, emails)

# send to all employees
paper.fax(employees)
```






