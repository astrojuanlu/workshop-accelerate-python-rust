use pyo3::prelude::*;

mod line_count;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_py_rust")]
mod py_rust {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    /// Count lines in Python files in a directory
    #[pyfunction]
    fn count_lines_in_directory(directory: &str) -> PyResult<(usize, usize)> {
        Ok(super::line_count::count_lines_in_directory(&directory))
    }
}
