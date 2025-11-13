use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Count lines in a single file
fn count_lines(filepath: &Path) -> usize {
    match File::open(filepath) {
        Ok(file) => BufReader::new(file).lines().count(),
        Err(_) => 0,
    }
}

/// Filter paths by extension
fn filter_by_extension(paths: Vec<PathBuf>, extension: &str) -> Vec<PathBuf> {
    paths
        .into_iter()
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some(extension))
        .collect()
}

/// Count total lines across multiple files
fn count_lines_in_files(filepaths: &[PathBuf]) -> usize {
    filepaths.iter().map(|p| count_lines(p)).sum()
}

/// Scan directory and count lines of code in Python files
pub fn count_lines_in_directory(directory: &str) -> (usize, usize) {
    let all_files: Vec<PathBuf> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .collect();

    let py_files = filter_by_extension(all_files, "py");
    let total_lines = count_lines_in_files(&py_files);

    (py_files.len(), total_lines)
}
