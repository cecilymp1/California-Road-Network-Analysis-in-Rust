use crate::average_distance;
use crate::road_network;
use std::io;



pub fn run() -> io::Result<()> {
    let graph = road_network::load_road_network_from_file("california_roads.txt")?;
    // I chose 3000 for MC iterations because I wanted to be able to get a good idea of shortest distance for such a large txt file. 
    let monte_carlo_iterations = 3000; 
    let sample_size = 10; // Sample this many end nodes for each start node

    match average_distance::calculate_average_distance(&graph, monte_carlo_iterations, sample_size) {
        Some(avg_distance) => println!("The average distance is {}", avg_distance),
        None => println!("Could not compute the average distance."),
    }

    Ok(())
}