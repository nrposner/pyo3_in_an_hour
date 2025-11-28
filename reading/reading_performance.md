# Further Reading: High-Performance PyO3

So far we've been focused on just getting our applications working without caring too much about performance, assuming that the performance boost from using Rust will be big enough by itself. 

In many applications, this may even be the case, and we don't need to worry too much about the performance loss from copying data on the FFI. However, for applications requiring high performance, the cost of an undisciplined FFI crossing can significantly impact or even dwarf the cost of the internal function. 

In these cases, we need to care deeply about avoiding copies and runtime checks on each FFI call, and there are several strategies we can employ to this effect. See also the PyO3 User Guide's chapter on [Performance](https://pyo3.rs/v0.27.1/performance.html).

## Use `numpy` to minimize copying and take advantage of vectorization

Up to this point, we've stuck to vanilla Python and PyO3. But the core Python language and its built-in libraries do not provide a contiguous-memory `array` type, and Python `lists` do not fill that niche. This is why the `numpy` package is so ubiquitous in numeric Python libraries. 

It is thus very useful to interface with `numpy` directly from Rust: we can do this using the `numpy` crate, which provides a variety of useful, multidimensional array types such as PyArray and PyReadonlyArray. Operating on these types (give or take a little `unsafe`) can often provide comparable or even superior performance to C-Python pointer juggling, especially when iterating over many vectors simultaneously, while being considerably more concise and readable. See this [blog post](https://nrposner.com/blog/rust-is-2x-faster-than-rust-pyo3-edition/) for an example. 

## Avoid unnecessary attachment checks

Many things we do with PyO3 require attachment to the Python interpreter. According to the PyO3 documentation, attaching to the interpreter using `Python::attach` is 'effectively a no-op', but checking for an existing attachment still carries some cost. 

Luckily, even this small cost can be avoided through the use of `Bound`, hallowed be its name. Since having access to a `Bound<T>` implicitly requires attachment to the Python interpreter it is possible to manually extract a token that validates this attachment using the `.py()` method on `Bound`. See the relevant section in the PyO3 User Guide's Performance chapter for further details.

## Detaching from the interpreter

...

