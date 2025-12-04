# Calling Python from Rust

While most of our use cases for Rust-Python interop have us calling Rust code from Python in order to provide flexibility to the end user while taking advantage of Rust's performance and safety characteristics, there are some cases where we instead want to call Python from Rust. This can be done in a few ways: 

We can directly write and evaluate inline Python code: however, doing so is verbose and best reserved for very short statements.

```rust
// from the PyO3 User Guide
fn test_pycall() -> Result<(), String> {
    Python::attach(|py| {
        let activators = PyModule::from_code(py, CString::new(r#"
    def relu(x):
        return max(0.0, x)

    def leaky_relu(x, slope=0.01):
        return x if x >= 0 else x * slope
        "#).unwrap().as_c_str(), 
        CString::new("activators.py").unwrap().as_c_str(), 
        CString::new("activators").unwrap().as_c_str()).unwrap();

        let relu_result: f64 = activators.getattr("relu").unwrap().call1((-1.0,)).unwrap().extract().unwrap();
        assert_eq!(relu_result, 0.0);

        let kwargs = [("slope", 0.2)].into_py_dict(py);
        let lrelu_result: f64 = activators
            .getattr("leaky_relu").unwrap()
            .call((-1.0,), Some(&kwargs.unwrap())).unwrap()
            .extract().unwrap();
        assert_eq!(lrelu_result, -0.2);
    });

    Ok(())
}
```

A more flexible method is to import existing modules from the local environment. This allows Rust code to directly run third-party Python code, such as numpy, and capture its output. This does require that the relevant wheels are loaded in the environment at runtime. 

It also means that, since we can export our Rust code as a Python wheel into the local environment, we can call our own code in Python from Rust... from Python... from Rust... I haven't tried this, and don't recommend doing so, but it does seem like a new and innovative way to blow your stack.

```rust
...
```

Third, we can read Python code from a local file and execute it directly: this gives us some of the convenience of the inline method while being more readable and organized.

```rust
...
```


With these tools, the Python-Rust Orobouros is complete, and gives us another powerful tool in our toolbelt. Not only can we provide our developers with Python tools written in Rust, but our developers can now write Python code which we can execute in Rust!

## Exercise

This is very useful for Dunder Mifflin, since it means our Rust firmware can run Ai algorithms written in Rust. Let's take a look at one of these algorithms; I think Dwight was in charge of making some...

```python
def dwights_ai_decision_algorithm(paper: Paper, employees: list[Employee]) -> list[str]:
    if "do not shred" in paper.contents.lower():
        # we search for the names of employees in the paper
        employee_names = [employee.name.lower() for employee in employees]
        # and then send to all those mentioned
        present = employees[employee_names in paper.contents.lower()]
        return [employee.email for employee in present]
    else:
        paper.shred()
```

...

DWIIIIIIIIIGHT!!!

You know what, fine. We can work with this. 

Let's make a pyfunction that can be called from Python, executed in Rust, and uses the Python code defined locally in `assets/algorithms.py`


## Hints 

...







