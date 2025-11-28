# Rust-Python in 1 Hour
## Introduction to Rust-Python Interop with PyO3

Welcome! If you're reading this, you're either attending the RustNYC 2025 Unconference, or you're following along with this workshop independently. Either way, I'm glad to have you!

This workshop is a guided introduction to interoperation between Rust and Python programs using `PyO3` bindings and the `maturin` build system. It is intended for people comfortable with the Rust programming language and at least some experience with Python.

## Structure

This repository is structured as follows:
- A `src/` directory containing the base Rust code
- An `exercises/` directory containing the explanations and instructions for each section
- A `python/` directory containing the test cases for each section
- More detailed explanations of certain topics in the `reading/` directory

### Setup
To work through the directory, please follow these steps:
1. `git clone` this repostory to your local environment.
2. Install the following requirements locally:
    - Rust >= v1.83
    - Python >= 3.8
    - the [`maturin` build system](https://www.maturin.rs/installation.html)
    - (Recommended) `uv` for virtual environment management in Python
3. In the cloned repository, initialize a Python virtual environment using `uv` or your tool of choice and follow these steps or their equivalents:
    - `uv venv .venv`
    - `source .venv.bin/activate`
    - `uv pip install -U pip maturin`
4. Start from `intro.md` in the `exercises/` directory, and then proceed to the first stage, `1_basics.md`.
5. For each stage, modify the code in `src/` to pass the tests located in the `python/` directory. Build the Rust-Python code using `uv run maturin develop` (or equivalent).
6. To run the tests for each stage, run the equivalent file in the `python` directory, e.g. `python3 python/1_basics.py`

If you're in the room live with us, feel free to ask questions and for help at any point. If you're doing this later on your own, and have further questions, please raise Issues on this repository, or direct questions to nicolasposner@gmail.com.

Good luck, and have fun!
