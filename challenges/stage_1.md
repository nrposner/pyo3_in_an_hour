# Stage 1

First, let's make the Paper struct and its basic methods accessible from Python.

At the bottom of the `lib.rs` file, you'll find this odd little function. 

```rust
#[pymodule]
fn dunder_mifflin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
```

This function constructs the `dunder_mifflin` python module: anything you want to publicly expose as part of the module, you add here. 

For example, if you had a Python class named ExampleClass, you could expose it as follows:

```rust
#[pymodule]
fn dunder_mifflin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ExampleClass>()?; // add this line!
    Ok(())
}
```

Then access it from Python as follows:

```python
import dunder_mifflin as dm
from dm import ExampleClass

example_class = ExampleClass.new(...)
```

## Creating a PyClass from Rust

You can turn an existing Rust struct or enum into a Python class using the `#[pyclass]` attribute macro.

```rust
use pyo3::prelude::*;

#[pyclass]
struct ExampleClass {}
```

You can also use the `name` and `module` fields to customize its appearance to Python-side users.

```rust
#[pyclass(name="MyCoolExampleClass", module="dunder_mifflin")]
```

Under the hood, this implements the pyo3::PyClass trait for your item. You could do this yourself, but for the basics like this, it's best to trust the macro!. 

## Implementing methods for a PyClass

In the same way, we can write methods for a PyClass.

We can create a block of python methods, separate from the regular Rust methods, with the `#[pymethods]` attribute macro:

```rust
use pyo3::prelude::*;

#[pyclass]
pub struct ExampleClass {}

// Rust-exclusive methods
impl ExampleClass {
    pub fn foo(&self) {
        println!("I've never head of Python")
    }
}

// a plural!
#[pymethods]
impl ExampleClass {
    pub fn bar(&self) {
        println!("Call me from Python!")
    }
}
```

Not all methods defined on the Rust-side for a PyClass need to also be available from Python! You can pick and choose, exposing only a more limited, curated API to the Python user and reserving the more finnicky, sensitive API for Rust-side development.


### Specific PyMethod types

PyO3 also provides some macros to specify more specific kinds of python methods

`#[new]`
`#[getter]`
`#[setter]`
`#[staticmethod]`
`#[classmethod]`
`#[classattr]`
`#[args]`





