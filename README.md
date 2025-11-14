# Accelerate Python with Rust

Workshop "Accelerate Python with Rust"

## Requirements

- uv https://docs.astral.sh/uv/
- rustup https://rustup.rs/
- Your IDE of choice

## Install

```
$ git clone https://github.com/astrojuanlu/workshop-accelerate-python-rust
$ cd workshop-accelerate-python-rust
$ uv sync
```

## Edit slides locally

```
$ uv run marimo edit slides.py
```

## Workshop

### 1. Basic setup with `maturin`

```
$ uvx maturin new -b pyo3 --src py-rust
$ cd py-rust
$ uv run python
>>> import py_rust
>>> py_rust.sum_as_string(2, 3)
```

### 2. Improve directory layout

```
$ cd py-rust
$ mkdir rust
$ mv Cargo.* rust/
$ mv src/ rust/
$ mkdir -p src/py_rust
$ touch src/py_rust/__init__.py src/py_rust/rust.py
$ # edit pyproject.toml to tweak module-name
$ # edit lib.rs to tweak module name
$ # edit rust.py to import functions explicitly
$ uv run python
>>> import py_rust.rust
>>> py_rust.rust.sum_as_string(2, 3)
'5'
```

### 3. Add linting and formatting tools

```
$ cd py-rust
$ uv add --dev ruff
$ # edit pyproject.toml to add more checks
$ ruff check --fix && ruff format
```

### 4. Add line count in Python

```
$ cd py-rust
$ uv add --dev ipython
$ # edit python.py
$ uv run ipython
In [1]: from pathlib import Path

In [2]: import py_rust.python

In [3]: p = Path.cwd() / ".venv"

In [4]: py_rust.python.count_lines_in_directory(p)
Out[4]: (847, 282939)

In [5]: %timeit py_rust.python.count_lines_in_directory(p)
35.3 ms ± 890 μs per loop (mean ± std. dev. of 7 runs, 10 loops each)
```

### 5. Add line count in Rust

```
$ cd py-rust
$ cd rust
$ cargo add walkdir
$ # edit line_count.rs
$ # edit lib.rs to include line_count module
$ # edit rust.py to explicitly import function
$ uv run ipython
In [1]: from pathlib import Path

In [2]: import py_rust.rust

In [3]: p = Path.cwd() / ".venv"

In [4]: py_rust.python.count_lines_in_directory(p.as_posix())
Out[4]: (847, 282939)

In [5]: %timeit py_rust.rust.count_lines_in_directory(p.as_posix())
21 ms ± 2.6 ms per loop (mean ± std. dev. of 7 runs, 10 loops each)
```

### 6. Parallelize the code in Rust!

```
$ cd py-rust
$ cd rust
$ cargo add rayon
$ # edit line_count.rs
$ uv run ipython
In [1]: from pathlib import Path

In [2]: p = Path.cwd() / ".venv"

In [3]: import py_rust.rust

In [4]: py_rust.rust.count_lines_in_directory(p.as_posix())
Out[4]: (847, 282939)

In [5]: %timeit py_rust.rust.count_lines_in_directory(p.as_posix())
6.09 ms ± 190 μs per loop (mean ± std. dev. of 7 runs, 100 loops each)
```
