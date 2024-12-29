# Creating PyO3 Project with uv

In this article, we are going to explore how to create a Rust library
which we can can from Python.


## Step 1: Create a Rust Library for Python

    uv tool run maturin new --name=numlib --bindings=pyo3 numlib
    cd numlib

The `numblib` directory now contains the following files

    ├── Cargo.toml
    ├── pyproject.toml
    └── src
        └── lib.rs

The content of the `Cargo.toml` file:

```toml
[package]
name = "numlib"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[lib]
name = "numlib"
crate-type = ["cdylib"]

[dependencies]
pyo3 = "0.23.3"
```

The content of the `src/lib.rs` file:

```rust
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn numlib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```


## Step 2: Create Function `factorial`

We now edit `src/lib.rs` to add function `factorial`. Notes

1. The triple slash provide documentation
2. We add a bunch of simple tests

```rust
// lib.rs
use pyo3::prelude::*;

/// Return the factorial of N
#[pyfunction]
fn factorial(n: u64) -> PyResult<u64> {
    if n == 0 || n == 1 {
        Ok(1)
    } else {
        Ok(n * factorial(n - 1).unwrap())
    }
}

/// Provide numerical functions
#[pymodule]
fn numlib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(factorial, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn test_0() {
        assert_eq!(factorial(0).unwrap(), 1);
    }

    #[test]
    fn test_1() {
        assert_eq!(factorial(1).unwrap(), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(factorial(2).unwrap(), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(factorial(3).unwrap(), 6)
    }

    #[test]
    fn test_6() {
        assert_eq!(factorial(6).unwrap(), 720);
    }
}
```

## Step 3: Create a Makefile

To automate the process of building, let us create a `Makefile` with
the following content:

```
.PHONY: lint test build clean

build: lint test
	uv tool run maturin build --release

test: lint
	cargo test

lint:
	cargo check
	cargo clippy
	cargo fmt

clean:
	cargo clean
```

Basically, we invoke this `Makefile` by typing 

```bash
make
```

at the shell prompt. This in turn will lint, test, then build the
project. The result will be in the `target/wheels` directory. We can
install the files in this directory into the Python virtual environment of
other projects. The specific steps are beyond the scope of this article.
