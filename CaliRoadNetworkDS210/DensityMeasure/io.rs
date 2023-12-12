use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Function to read graph from file and populate nodes, edges, and adjacency_map
pub fn read_graph_from_file(filename: &str) -> io::Result<(HashSet<i32>, Vec<(i32, i32)>, HashMap<i32, HashSet<i32>>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut nodes = HashSet::new();
    let mut edges = Vec::new();
    let mut adjacency_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;
        }

        let parts: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .filter_map(Result::ok)
            .collect();

        if parts.len() == 2 {
            let (src, dst) = (parts[0], parts[1]);
            edges.push((src, dst));
            nodes.insert(src);
            nodes.insert(dst);

            adjacency_map.entry(src).or_default().insert(dst);
            adjacency_map.entry(dst).or_default().insert(src);
        }
    }

    Ok((nodes, edges, adjacency_map))
}
