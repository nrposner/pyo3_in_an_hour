Now that we've gotten our feet wet with creating classes, let's do it again with a slightly more involved example:

```rust
pub struct Paper {
    paper_size: PaperSize,
    #[pyo3(get)]
	contents: String,
}

pub struct PaperSize { 
    width: u32, 
    height: u32, 
}
```

The `#[pyo3(get)]` decorator automatically generates a getter methods for the `contents` field, saving us some boilerplate. 

With what we learned before, we should be able to trivially make a constructor that satisfies this test:

```python
Paper(6, 8, "IMPORTANT MEMO")
```

Unfortunately, Dwight has some very odd ideas about page dimensions: he insists that negative width is valid because it should just 'be wide in the other direction' and will try to pass in negative integers for the dimensions of a paper.

```python
Paper(-6, 8, "NEW OFFICE RULES")
```

If we build this like we did `Employee`, this will fail: we need to make a judgment call here:

Either determine that the creation of a Paper can fail, and if so return an error to Python, or silently correct the value under the hood.

## Hints

### 1. Fallible Constructors

The #[new] `__init__` builder in PyO3 doesn't just support returning `Self`: it also supports returning `PyResult<Self>`! PyResult can return as an error any kind of Python exception, without needing to specify which ones or create an enum.

### 2. Python Exceptions in Rust

PyO3 makes Python exceptions (which are divided into Warnings and Errors) available using the `pyo3::exceptions` module. `PyTypeError`, `PyValueError`, and `PyWarning` are particularly useful!


