# Stage 1

Python classes are a convenient way to wrap around Rust structs and impl methods, but this isn't Java: we've got free-floating functions!

We can add functions to our module just like we added functions earlier, with the `wrap_pyfunction!()` macro to help us out.

```rust
#[pymodule]
fn dunder_mifflin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(example_function, m)?)?;
    Ok(())
}
```

Then access it from Python as follows:

```python
import dunder_mifflin as dm
from dm import ExampleClass

example_class = ExampleClass.new(...)
```

## Creating a PyFunction from Rust


You can make an existing Rust function available from Python using the `#[pyfunction]` attribute macro.

```rust
use pyo3::prelude::*;

#[pyfunction]
fn example_function() {}
```

As with PyClasses, this does not need to be a public function so long as it's in the same file as the python module.



## Writing Effective PyFunctions

The key concern when writing PyFunctions in Rust, beyond the usual considerations for writing Rust functions, is how receive inputs from Python and how to return outputs to it.

If you're dealing with common primitive types shared by both Python and Rust, such as strings, integers and floats, booleans, and simple collections, then you can just pass them around directly. Much like Python itself, it's not the most optimized way to do this, but it's easy.


```rust
use pyo3::prelude::*;

#[pyfunction]
fn example_function(
    example_int: i32,
    example_vec: Vec<u32>,
    multiply: bool,
) -> Vec<f64> {
    example_vec.iter().map(|u| {
        if multiply {
            u as f64 * example_int as f64
        } else {
            u as f64 / example_int as f64
        }
    }).collect()
}
```

In the above snippet, we show that it's trivial to intake primitives like numerics and booleans, and even collections like vecs, since they all have analogous object types in Python: `bool`s, arbitrarily sized `int`s and `float`s, and the python `list`[1]. We can also trivially output the resulting `Vec<f64>`, because PyO3 can trivially transform all its constituent types into analogous Python objects:

[1] Those of you who know how Python's `list`s work may be pointing out that `list` and `Vec` are totally different data structures, and you're right. Unfortunately, Python does not have a built-in `array` type for contiguous, single-type memory allocation, which makes reading from and creating lists expensive and wasteful, but they're the closest thing to a `Vec` in vanilla Python. In real life, almost everyone will use `numpy.array`s, but for the purpose of this lesson, we're focusing on vanilla Python only, warts and all. Don't worry, we'll take a more critical look and discuss memory optimization strategies later!



