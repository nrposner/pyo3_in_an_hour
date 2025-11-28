Some of the most basic, useful things we can do are to define functions and classes in Rust, and then make them available from Python. 

In the case of functions, this is generally because we want access to speed, reliability, or low-level control that is not normally available in native Python. In the case of classes, this is often because we want fine-grained control over memory or want to leverage Rust's type system. 

There are numerous advantages to developing these internals in Rust, but we still want to be able to access and use them from Python. 

We'll start by demonstrating how to make Rust's structs and enums available as Python classes, and then make standalone Rust functions available as well.

# Uses of Python Classes in Rust

Though both are commonly used in their respective languages to group related data together and implement method functions direclty on that grouped data, Rust structs and Python classes share few other similarities: where Rust structs are static, strongly typed, and provide locality of memory, Python classes make no such guarantees: unless special steps are taken, any field of a Python class can be modified at any time, including having its type changed, and new fields can be added to it at will. We'll cover the steps used to prevent this in [[[# 4. Defensive Programming in Rust-Python]]]. 

Python classes can also abstract over a great deal of implicit, ergonomic behavior: iteration, containment, display logic, binary operations, and more can be specially defined by the class author, in much the same way that a Rust developer will derive, and sometimes custom-define trait implementations like Debug/Display and std::ops. We will cover the steps we can take to make a Rust-defined object into an ergonomic, Pythonic class in [[[# 3. Customization and Ergonomics]]]

# Defining Python Classes in Rust

```rust
use pyo3::prelude::*;

#[pyclass]
struct ExampleStruct {
    foo: u32,
    bar: String,
    baz: Vec<bool>
}

#[pyclass]
enum ExampleEnum {
    Foo(u32),
    Bar(String),
    Baz(Vec<bool>)
}
```

With PyO3, Rust structs and enums can be marked as Python classes simply by using the `#[pyclass]` attribute macro. For simple structs and enums like those above, which are composed entirely of standard library types with Python equivalents, this is all that is necessary.

Under the hood, PyO3 is deriving the `FromPyObject` and `IntoPyObject` traits: this can be done trivially for most classes which are composed of simple types with equivalents in Python, but may need to be implemented manually in other cases.

Note that, at present, PyO3 does not support deriving FromPyObject or IntoPyObject for empty structs or enum variants.



