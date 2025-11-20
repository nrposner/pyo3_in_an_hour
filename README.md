
Structure


Put up link to repo and things to download and have ready (a recent version of rustc and cargo, a recent version of the Python interpreter, and then install pyo3 and maturin).

Leave that up on the following slides to make sure people can continue to follow them??

A few slides on why Python-Rust interop

A lot of it is tool creation: Python is a very convenient environment in which to do a lot of things with a lot of tools, and recently a lot of them are being written in Rust: 



polars supplanting pandas, qdrant supplanting pyspark??

uv replacing basically the entire Python environment ecosystem, and thank god for that


And on top of that, the recent Pre-PEP to start introducing Rust into CPython. 
That, and the NumPy maintainers are started to rumble in the same direction.

Rust is here, and it's here to stay. It's a great time to get into Rust-Python interop.


Today's workshop is going to introduce Rust-Python interop using PyO3. It's not the only option, and if you're doing low-level interoperation like adding Rust into CPython or NumPy, you'll need to get a little more hands on. But for 95% of cases, it's the right choice: PyO3 lets you bind Rust and Python code very simply and distribute it just as easily.




# Sections

## Intro: Who you are

## Make PyClasses available
And discuss common patterns and pitfalls: if you're in this room, chances are you're familiar with the deficiencies of object-oriented design patterns. Just because you CAN use classes, doesn't mean you should use them for everything!

Also note that, even though the pyclass is a struct on your end, it acts like a regular Python class on the other end, which means it can be near-arbitrarily modified.

Use `__x` instead of `x` for the names of fields to prevent their arbitrary modification, and then provide getters for the same.

Also use __slots__ to prevent arbitrary addition of fields

With this, you can mostly replicate the experience of using structs in Python


Actually, one of the later challenges should be Dwight trying to just arbitrarily change pyclass contents, and you need to stop him from doing that.

## Make PyFunctions available

## Customize PyClasses with dunder methods for ergonomics

## Call Python from Rust!

## Optimize Python-Rust memory calls 
??









