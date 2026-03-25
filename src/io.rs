//! File and stdin reading utilities.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

/// Reads all lines from a file into a `Vec<String>`.
///
/// # Errors
/// Returns an error if the file cannot be opened or read.
///
/// # Example
/// ```no_run
/// use termz::io::read_lines;
/// let lines = read_lines("data.txt").unwrap();
/// ```
pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().collect()
}

/// Reads the entire contents of a file into a `String`.
///
/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

/// Reads a single line from stdin without printing a prompt.
///
/// Trims the trailing newline.
pub fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap_or(0);
    buf.trim_end_matches(['\n', '\r']).to_string()
}

/// Returns `true` if the file at `path` exists and is a regular file.
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}
