use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // Read in graph from file
    let file = File::open("california_roads.txt").unwrap();
    let reader = BufReader::new(file);

    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        // Ignore comment lines
        if line.starts_with('#') {
            continue;
        }

        let numbers: Vec<_> = line
            .split_whitespace()
            .map(|s| i32::from_str_radix(s, 10))
            .filter_map(Result::ok)
            .collect();

        if numbers.len() == 2 {
            let (src, dst) = (numbers[0], numbers[1]);
            edges.push((src, dst));
        }
    }

    // Calculate degree centrality
    let mut degree_counts = HashMap::new();
    for &(src, dst) in &edges {
        *degree_counts.entry(src).or_insert(0) += 1;
        *degree_counts.entry(dst).or_insert(0) += 1;
    }

    // Sort degree counts by degree
    let mut degree_counts_vec: Vec<_> = degree_counts.into_iter().collect();
    degree_counts_vec.sort_by(|a, b| a.0.cmp(&b.0));

    // Print degree centrality
    println!("Degree centrality:");
    for (node, degree) in degree_counts_vec.iter().rev() {
        println!("Node {}: {}", node, degree);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    // Provide a method to calculate degree centrality to facilitate testing
    fn calculate_degree_centrality(edges: &[(i32, i32)]) -> HashMap<i32, i32> {
        let mut degree_counts = HashMap::new();
        for &(src, dst) in edges {
            *degree_counts.entry(src).or_insert(0) += 1;
            *degree_counts.entry(dst).or_insert(0) += 1;
        }
        degree_counts
    }

    #[test]
    fn test_degree_centrality_with_sample_data() {
        // Sample graph represented as a list of edges
        let edges = vec![(1, 2), (2, 3), (3, 1), (4, 2)];
        let degree_counts = calculate_degree_centrality(&edges);

        // Expected degrees for each node
        let mut expected_counts = HashMap::new();
        expected_counts.insert(1, 2);
        expected_counts.insert(2, 3);
        expected_counts.insert(3, 2);
        expected_counts.insert(4, 1);
        
        assert_eq!(degree_counts, expected_counts);
    }

    #[test]
    fn test_degree_centrality_empty_graph() {
        let edges = vec![];
        let degree_counts = calculate_degree_centrality(&edges);

        let expected_counts: HashMap<i32, i32> = HashMap::new();

        assert_eq!(degree_counts, expected_counts);
    }
}

