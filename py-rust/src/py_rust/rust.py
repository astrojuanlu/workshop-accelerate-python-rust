# See https://www.maturin.rs/project_layout
from . import _py_rust
from ._py_rust import sum_as_string

__all__ = ["sum_as_string"]

__doc__ = _py_rust.__doc__
