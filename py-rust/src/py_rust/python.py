"""Count lines of code in Python files."""

import typing as t
from pathlib import Path


def count_lines(filepath: Path) -> int:
    """Count lines in a single file."""
    try:
        with open(filepath, encoding="utf-8") as f:
            return sum(1 for _ in f)
    except (OSError, UnicodeDecodeError):
        return 0


def filter_by_extension(paths: t.Iterator[Path], extension: str) -> list[Path]:
    """Filter paths by file extension."""
    return [p for p in paths if p.suffix == extension]


def count_lines_in_files(filepaths: list[Path]) -> int:
    """Count total lines across multiple files."""
    return sum(count_lines(f) for f in filepaths)


def count_lines_in_directory(directory: Path) -> tuple[int, int]:
    """Count total lines in all .py files in directory."""
    py_files = filter_by_extension(Path(directory).rglob("*"), ".py")
    total_lines = count_lines_in_files(py_files)

    return len(py_files), total_lines
