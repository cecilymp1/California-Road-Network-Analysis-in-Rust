mod graph;
mod io;
mod application;

fn main() {
    application::run("california_roads.txt").unwrap();
}

#[cfg(test)]
mod tests {
    use super::graph::*;

    #[test]
    fn test_calculate_density_empty() {
        let nodes = HashSet::new();
        let edges = Vec::new();
        let density = calculate_density(&nodes, &edges);
        assert!(density.is_nan());
    }

    #[test]
    fn test_calculate_density() {
        let nodes = HashSet::from_iter(vec![1, 2, 3, 4]);
        let edges = vec![(1, 2), (2, 3), (3, 4), (4, 1), (1, 3), (2, 4)];
        let density = calculate_density(&nodes, &edges);
        let expected_density = 1.0; // Complete graph should have a density of 1
        assert_eq!(density, expected_density);
    }

    #[test]
    fn test_compute_densest_subgraph() {
        let mut adjacency_map: HashMap<i32, HashSet<i32>> = HashMap::new();
        adjacency_map.insert(1, HashSet::from_iter(vec![2, 3]));
        adjacency_map.insert(2, HashSet::from_iter(vec![1, 3]));
        adjacency_map.insert(3, HashSet::from_iter(vec![1, 2]));

        let densest_subgraph = compute_densest_subgraph(&adjacency_map);
        assert_eq!(densest_subgraph, HashSet::from_iter(vec![1, 2, 3]));
    }

    #[test]
    fn test_edges_in_subgraph() {
        let mut adjacency_map: HashMap<i32, HashSet<i32>> = HashMap::new();
        adjacency_map.insert(1, HashSet::from_iter(vec![2]));
        adjacency_map.insert(2, HashSet::from_iter(vec![1, 3]));
        adjacency_map.insert(3, HashSet::from_iter(vec![2]));

        let subgraph = HashSet::from_iter(vec![1, 2]);
        let edges = edges_in_subgraph(&subgraph, &adjacency_map);
        let expected_edges = vec![(1, 2), (2, 1)];
        assert_eq!(edges.contains(&(1, 2)), true);
        assert_eq!(edges.contains(&(2, 1)), true);
        assert_eq!(edges.len(), expected_edges.len());
    }
}