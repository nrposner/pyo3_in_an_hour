# Challenge 1: Defining Employees

Before anything else, we need to know who's in the office! We've defined an Employee struct in Rust:

```rust
// src/employees.rs
pub struct Employee { 
    id: u32,
    name: String, 
    email: String, 
    michaels_notes: String,
    inbox: HashSet<String>
}
```

And now we want to create a new Employee object in Python as follows:

```python
michael = Employee(1, "Michael", "mscott@dundermifflin.com", "the best")
```

Create a pymethod that instantiates an Employee class instance with the following values and an empty inbox.

## Hints

### 1. Creating a PyClass

Since Employee is a simple struct composed of simple standard library types, we can turn it into a python class using the `#[pyclass]` attribute macro.

```rust
#[pyclass]
pub struct Employee { 
```

### 2. Creating a Python constructor method

While creating a regular constructor on the Rust side is also useful, we'll need to provide a specialized Python-side constructor to get the behavior above. This is accomplished through the `__init__` dunder method. We can also mark a separate set of impl methods as available from Python using the `#[pymethods]` attribute macro, and use the `#[new]` attribute macro to mark it as a Python constructor.

```rust
#[pymethods]
impl Employee {
    #[new]
    pub fn __init__(...) -> Self { .. }
}
```

Note: since `__init__` is used for Python-side construction, this method can coexist with a Rust-native `Self::new()` constructor. They could even have totally different interfaces!







