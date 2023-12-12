mod network;
mod centrality;

fn main() {
    //reading in the text file and turning it into a graph and grabbing the edges
    let edges = network::read_edges_from_file("california_roads.txt").unwrap();
    // Calculate the degree centrality for each node in the graph using the edges
    let degree_counts = centrality::calculate_degree_centrality(&edges);

    // Converting the hashmap of degree centrality counts into a vector for sorting
    let mut degree_counts_vec: Vec<_> = degree_counts.into_iter().collect();
    // sorting the degree counts vector by node ID in ascending order
    degree_counts_vec.sort_by(|a, b| a.0.cmp(&b.0));

    // printing the degree centrality of each node, in descending order of centrality
    println!("Degree centrality:");
    for (node, degree) in degree_counts_vec.iter().rev() {
        println!("Node {}: {}", node, degree);
    }
}