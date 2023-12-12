use std::fs::File;
use std::io::{self, BufRead, BufReader};

// This function reads edges from a given file and returns a Result wrapping a vector of tuples
// Each tuple represents an edge with two integer nodes
// Attempts to open the file at the provided file path and creates a buffered reader
pub fn read_edges_from_file(file_path: &str) -> io::Result<Vec<(i32, i32)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // create an empty vector to store edges
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line?;

        // Ignore lines that start with a hashtag in text file
        if line.starts_with('#') {
            continue;
        }

        let numbers: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .filter_map(Result::ok)
            .collect();
        // if there're exactly two numbers on a line then we add them as an edge to the edges vector
        if numbers.len() == 2 {
            edges.push((numbers[0], numbers[1]));
        }
    }

    Ok(edges)
}