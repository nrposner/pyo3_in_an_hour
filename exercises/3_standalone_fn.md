Now that we've gotten some experience creating basic Python classes from Rust, let's do the same for freestanding functions.

```rust
// src/machine.rs
pub fn search_paper(paper: Paper, keys: Vec<String>, any: bool) -> Vec<bool> { ... }
```

We have a function that takes in a Paper struct, a Vector of Strings, and a boolean. We can make this function callable from Python by adding the `#[pyfunction]` decorator on top and adding it to our `PyModule`.

Is that it?

Sort of. 

In this case, that is sufficient, because all these types implement the `FromPyObject` trait: `Paper` implements the trait because we've already defined it as a `#[pyclass]`, so being able to pass it between Rust and Python comes free. `Vec<T>`, `String`, and `bool`, being common standard library types, also have implementations that allow us to coerce Python types into them: `str` becomes `String`, any kind of ordered iterable can become a `Vec`, and booleans need no introduction. 

The core concerns when writing `pyfunction`s in Rust, beyond the regular concerns of writing Rust functions, lies in managing the input and return types, as well as handling any in-place mutation of Python objects, as we saw in the last section. This is especially important when writing high-performance functions that minimize copying; but for our purposes at the moment, we can take the simpler and slightly less performant route. 

Our real problem comes here: our users want our functions to act like Python functions in ways Rust doesn't allow for:

```python
results = search_paper(paper, ["lorem", "ipsum"]) # implied any=False
```

Python users are accustomed to functions with keyword arguments (kwargs) allowing them to use default variables without having to fill them in. Rust does not support this.

In Rust, we'd have to take our lumps and just manually pass in `false` most of the time. But we want to provide our Python user with the full ergonomics that Python allows for: the choice to use Python functions written in Rust should not carry a cost to quality of life or ergonomics.

We can accomplish this using PyO3's `signature` extension. 

```rust
#[pyfunction(signature = (bar, baz=42, zap=true))]
pub fn foo(bar: String, baz: i32, zap: bool) {...}
```

This extension of the `#[pyfunction]` attribute macro allows the author to define the Python-visible signature of the `pyfunction`, including default values for kwargs!

Just note that, as in Python, kwargs can only go after regular, required arguments, and that unlike Python, the default values must be of the same type as declared for that variable.

## Exercise

Let's implement this interface for our `search_paper` function, with `any` being false by default, such that we satisfy both of the following calls:

```python
# using only positional arguments
search_paper(p, ["score", "Michael"])

# using the optional keyword argument
search_paper(p, ["dunder", "Michael"], any=True)
```

## Further exercise

In addition to defining methods that operate on instances of classes (equivalent to `&self` and `&mut self` methods in Rust) and defining freestanding functions, we can also define methods that do not require an instance of a class. 

In Rust, this would simply be done by creating a struct or enum method that doesn't take the `self` argument or variants on it, and then calling it using a fully qualified path.

But to be accessible from Python, we must mark this method with the `#[staticmethod]` decorator.

`#[staticmethod]` also has a close cousin, `#[classmethod]`, which has slightly different behavior: it passes the *type* of the class (not the instance itself, just the type) as the implicit first argument. This can occasionally be useful, but for the most part is marginal behavior.

