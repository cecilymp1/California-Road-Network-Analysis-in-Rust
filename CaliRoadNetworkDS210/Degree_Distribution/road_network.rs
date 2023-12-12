
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use petgraph::graphmap::UnGraphMap;


//Here we are setting up the public module function that will load and form the graph of the road network 
pub fn load_road_network_from_file<P: AsRef<Path>>(file_path: P) -> io::Result<UnGraphMap<usize, u32>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut graph = UnGraphMap::<usize, u32>::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let start: usize = parts[0].parse().unwrap();
            let end: usize = parts[1].parse().unwrap();
            graph.add_edge(start, end, 1);
        }
    }

    Ok(graph)
}