use std::collections::HashMap;


pub fn calculate_degree_centrality(edges: &[(i32, i32)]) -> HashMap<i32, i32> {
    let mut degree_counts = HashMap::new();
    for &(src, dst) in edges {
        *degree_counts.entry(src).or_insert(0) += 1;
        *degree_counts.entry(dst).or_insert(0) += 1;
    }
    degree_counts
}




//Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree_centrality_with_sample_data() {
        let edges = vec![(1, 2), (2, 3), (3, 1), (4, 2)];
        let degree_counts = calculate_degree_centrality(&edges);

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