use super::*;
use petgraph::graphmap::UnGraphMap;
use petgraph::graph::NodeIndex;

#[test]
fn test_select_city_with_valid_k() {
    let mut graph = UnGraphMap::new();
        for i in 0..100 {
            graph.add_node(i);
        }
        let cities = select_city(&graph, 10);
        assert_eq!(cities.len(), 10);
        // mnaking sure that all cities are unique
        let mut cities_set = std::collections::HashSet::new();
        for city in &cities {
            assert!(cities_set.insert(city.index()));
        }
}

#[test]
fn test_precompute_city_distances() {
    let mut graph = UnGraphMap::new();
    graph.add_edge(0, 1, 1);
    graph.add_edge(1, 2, 2);

    let cities = vec![NodeIndex::new(0), NodeIndex::new(1)];
    let distances = precompute_city_distances(&graph, &cities);
    assert_eq!(distances[&NodeIndex::new(0)][&NodeIndex::new(1)], 1.0);
    assert_eq!(distances[&NodeIndex::new(1)][&NodeIndex::new(2)], 2.0);
    // distance from a city to itself is 0
    assert_eq!(distances[&NodeIndex::new(0)][&NodeIndex::new(0)], 0.0);}

#[test]
fn test_approximate_shortest_path() {
    let mut graph = UnGraphMap::<usize, u32>::new();
    graph.add_edge(0, 1, 7);
    graph.add_edge(0, 2, 9);
    graph.add_edge(1, 2, 10);
    graph.add_edge(1, 3, 15);
    graph.add_edge(2, 3, 11);
    graph.add_edge(2, 5, 2);
    graph.add_edge(3, 4, 6);
    graph.add_edge(4, 5, 9);

    let cities = vec![
        NodeIndex::new(0),
        NodeIndex::new(1),
        NodeIndex::new(2),
        NodeIndex::new(3),
        NodeIndex::new(4),
        NodeIndex::new(5),
    ];
    let city_distances = precompute_city_distances(&graph, &cities);

    let start_node = NodeIndex::new(0);
    let end_node = NodeIndex::new(4);
    let alpha = 1.0;
    let approx_distance = approximate_shortest_path(start_node, end_node, &city_distances, alpha);

    // We expect to find the shortest path to be 0 -> 2 -> 5 -> 4 which is 20
    let expected_distance = 20.0;
    assert!((approx_distance - expected_distance).abs() < f64::EPSILON);
}