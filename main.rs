use std::io;
pub mod average_distance;
pub mod road_network;
pub mod application;

fn main() -> io::Result<()> {
    application::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_average_distance() {
        let mut graph = UnGraphMap::<usize, u32>::new();
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 4, 1);
        graph.add_edge(4, 5, 1);

        let avg_distance = average_distance::calculate_average_distance(&graph, 1000, 2).unwrap();
        assert!(avg_distance > 0.0);
    }

    #[test]
    fn test_calculate_average_distance_with_insufficient_nodes() {
        let graph = UnGraphMap::<usize, u32>::new();
        assert_eq!(average_distance::calculate_average_distance(&graph, 1000, 2), None);
    }
}
