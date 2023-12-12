use petgraph::graphmap::UnGraphMap;
use std::io::{self, BufReader, BufRead};
use std::fs::File;

pub fn load_road_network_from_file(file_path: &str) -> io::Result<UnGraphMap<usize, u32>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut graph = UnGraphMap::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;
        }
        let nodes: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .filter_map(Result::ok)
            .collect();
        if nodes.len() == 2 {
            graph.add_edge(nodes[0], nodes[1], 1);
        }
    }
    Ok(graph)
}