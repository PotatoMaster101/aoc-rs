#![cfg(feature = "std")]

use std::fs::File;
use std::io;
use std::io::{read_to_string, BufRead};

/// Returns all lines from a file.
#[inline]
pub fn get_all_lines(filename: &str) -> io::Result<Vec<String>> {
    Ok(get_text(filename)?.lines().map(String::from).collect())
}

/// Returns an iterator over the lines in a file.
#[inline]
pub fn get_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Returns all text from a file.
#[inline]
pub fn get_text(filename: &str) -> Result<String, io::Error> {
    let file = File::open(filename)?;
    read_to_string(file)
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::fs::remove_file;
    use super::*;

    #[test]
    fn test_get_lines() {
        fs::write("test_get_lines.txt", "line 1\nline 2").unwrap();
        let sut: Vec<_> = get_lines("test_get_lines.txt")
            .unwrap()
            .map_while(Result::ok)
            .collect();

        assert_eq!(sut, ["line 1", "line 2"]);
        remove_file("test_get_lines.txt").unwrap();
    }

    #[test]
    fn test_get_all_lines() {
        fs::write("test_get_all_lines.txt", "line 1\nline 2").unwrap();
        let sut = get_all_lines("test_get_all_lines.txt").unwrap();
        assert_eq!(sut, ["line 1", "line 2"]);
        remove_file("test_get_all_lines.txt").unwrap();
    }

    #[test]
    fn test_get_text() {
        fs::write("test_get_text.txt", "line 1\nline 2").unwrap();
        let sut = get_text("test_get_text.txt").unwrap();
        assert_eq!(sut, String::from("line 1\nline 2"));
        remove_file("test_get_text.txt").unwrap();
    }
}
