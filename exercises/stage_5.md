Now that we've got the basics of classes and functions down, let's work on making our objects more ergonomic to use from Python.

We've already seen the `__init__` dunder method used for class initialization, but there are many, many more useful methods that allow us to make `pyclass`es easy to use from Python while preserving the performance and safety benefits of a Rust implementation under the hood.

A brief, *very* non-exhaustive overview of some of the most useful and common such methods follows.

## Displaying Object Information

### In String form
The `__str__` and `__repr__` dunder methods allow us to customize how the 

If they are not implemented, `__str__` will default to ...




and `__repr__` will default to printing the hexademical location of that object in memory.



`__str__`: `fn __str__(&self) -> String`, somewhat analogous to Rust's `Display` trait. 

`__repr__`: `fn __repr__(&self) -> String`, somewhat analogous to Rust's `Debug` trait.

### In Numerical form

`__len__` and `__sizeof__` allow us to customize how the type communicates its size in memory and dimensions to the user. This can be especially useful for types which contain and organize other data.

`__len__`: `fn __len__(&self) -> N`
`__sizeof__`: `fn __sizeof__(&self) -> N`




where N is any numeric type
??????

## Iteration and Access

Similarly to how Rust types can implement Iterator-related traits in order to act like iterators, Python classes can implement several dunder methods that allow them to behave like iterables.

The `__iter__`, and `__next__` ...




...


At the same time, `__contains__` and `__getitem__` methods define how the class behaves with the `in` keyword and index syntax, respectively.

For example, instead of using our `search_paper` function from the last section, we might want to use the simpler, more Pythonic expression:
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




For this test case, go ahead and define several dunder methods on `Paper` to pass the following test cases:


```python
paper = Paper(6, 8, "Would I rather be feared or loved? Easy. Both. I want people to be afraid of how much they love me.")

# use __contains__
assert("love" in paper)

# use __len__ to return the number of words in the contents field
assert(len(paper)==21)

# use __getitem__ to return the index elements of the contents field
assert(paper[4]=="feared")
assert(paper[-2]=="love")

```



other test cases for iter and next???

or maybe this is enough
maybe we shouldn't cover sizeof here??



for paper
str representation should be the contents themselves, 
but the repr should be the contents, size, and other struct data, pretty-printed
length of a paper should be the number of words in its contents
but the sizeof of a paper should be its surface area in square inches... because reasons


