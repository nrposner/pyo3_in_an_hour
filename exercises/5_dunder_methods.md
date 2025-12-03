Now that we've got the basics of classes and functions down, let's work on making our objects more ergonomic to use from Python.

We've already seen the `__init__` dunder method used for class initialization, but there are many, many more useful methods that allow us to make `pyclass`es easy to use from Python while preserving the performance and safety benefits of a Rust implementation under the hood.

For a non-exhaustive summary of the most useful and common such methods, see `reading/dunder_methods.md`. For this exercise we will focus on just three: `__len__` `__contain__` and `__getitem__`.

## Length

How do we get the length of an iterable? In languages like Rust, we would use a method like `.len()`. However, this is not the idiom in Python, which prefers to use built-in functions like `len()`. 

That looks simple enough--but think about it for a moment, and it means that this function would need to be generic over any iterable type, including types defined by the user that do not subclass from built-in types! How does Python manage?

Well, it's a bit of a switcheroo. Under the hood, the `len(T)` function checks `T` for a dunder method named `__len__()`. If present, it calls the method and returns it output, outputting an error otherwise. 

## Containment and Access

The `__contains__` and `__getitem__` methods define how the class behaves with the `in` keyword and index syntax, respectively.

For example, instead of using our `search_paper` function from the last section, we might want to use the simpler, more Pythonic expression when we're checking for just one element:
```python
"lorem" in paper
```

By defining a custom `__contains__` method on `Paper` that checks for the existence of a string in the contents field of the class, we can use this build-in keyword instead of importing an external function: if we expand the implementation to allow for iterables of strings, we can make it even more expressive: however, note that, unlike a separate function, we cannot pass additional parameters like `any` into it.

Likewise, `__getitem__` integrates with Python's index syntax:
```python
paper[3] # extracts the 4th element of paper
paper[-2] # extracts the second-to-last element of paper
```

By defining a custom `__getitem__` method that splits whitespace and returns the nth element of the contents, field, we could get those words directly instead of having to extract `.contents()` manually. 

## Exercises

For this test case, go ahead and define several dunder methods on `Paper` to pass the following test cases:

```python
paper = Paper(6, 8, "Would I rather be feared or loved? Easy. Both. I want people to be afraid of how much they love me.")

# use __len__ to return the number of words in the contents field
assert(len(paper)==21)

# use __contains__
assert("love" in paper)

# use __getitem__ to return the index elements of the contents field
assert(paper[4]=="feared")
# let's support negative indices counting from the end as well!
assert(paper[-2]=="love")
```

## Hints

### Type signatures

```rust
fn __len__(&self) -> usize 
fn __contains__(&self, key: String) -> bool
fn __getitem__(&self, idx: i64) -> PyResult<String> 
```

### Supporting negative indices

In addition to regular, zero-indexed positive indices for accessing slice elements, idiomatic Python also supports negative indices, which count backwards from the end: -1 is the final element, -2 is the second to last, etc.

In the type signatures above, we've suggested that the index should be an i64 rather than a u64, precisely to enable this behavior. However, it's not the only way: we might also consider accepting a `&Bound<'_, PyAny>` type, which we could then attempt to manually extract into an unsigned integer, and if that should fail, to extract it into a signed integer and count from the end.  

## Further Exercises

### Expanding `__getitem__`

Our test of `__getitem__` only requires that we support integer indices, but Python also supports slice indices, such as `[0:4]` to get the first five items, as well as supporting step size, such as `[0:4:2]`. 

How would we expand our method to also allow for slice and step indices?

Hint: instead of expecting an i64, we can take in a `PyAny` value and attempt to extract values of different types: PyO3 provides a `PySlice` type for exactly this purpose!

### Implementing `__iter__` and `__next__`

Other common and useful dunder methods include `__iter__` and `__next__`, which allow a class to operate like an iterable: this allows us to loop over it using `for`, along with other conveniences!

Take a look at the PyO3 documentation for `__iter__` and `__next__`, and try to implement them on your own!
