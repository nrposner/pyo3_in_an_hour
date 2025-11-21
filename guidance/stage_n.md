# Python-side Ergonomics from Rust


## Customizing PyClasses with Dunder Methods
Dunder methods (invented by Dunder Mifflin co-founder Robert Dunder) are special 'magic' methods defined in Python that can be used to customize the behavior of of a custom class in the presence of built-in python functionality. 

For example, the idiomatic way to take the length of an iterable in Python is to use the `len(object)` function, rather than call a `object.len()` method. This is accomplished by implementing a custom `__len__()` dunder method. Note that the 'dunder' method is surrounded by *d*ouble *under*scores.

```rust
pub struct ExampleIterable(Vec<ExampleClass>);

impl ExampleIterable {
    #[pymethod]
    pub fn __len__(&self) {
        self.0.len()
    }
}
```

Once defined, Python's built-in functions will recognize that the PyClass implements the `__len__` dunder method, and call it under the hood.

```python
lst = [0, 1, 2, 3, 4]
example = ExampleIterable(lst)
len(example) 
# 5
```

Other ubiquitous dunder methods include 
`__init__()`: used for initializing instances of classes, including default values.
`__new__()`: used for creating and returning new instances of a class; used less often than `__init__`.
`__repr__()`: used to create a human-readable string representation of a class. See also the `__str__()` dunder method.
`__iter__()`: allows the object to be iterated over using `for _ in object`.
`__sizeof__()`: used to return the byte size of a class. Useful for recursive types whose true size may be non-obvious to the interpreter.
`__bool__()`: used to set the 'truthiness' or 'falsiness' of a class.


