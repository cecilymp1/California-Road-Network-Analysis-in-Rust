# DS210- California Road Network Project

- Author: Cecily Wang/Munoz-Pastrana 
- Collaborators: Help from CDS OH and tutoring
- Class: DS210, Professor Kontothanassis
- Date: December 12, 2023


This project aims to analyze routes within a road network represented by a graph data structure where intersections and endpoints are nodes, and the roads connecting these are undirected edges. In this project, I am using the California Road Network dataset from the Stanford Large Network Dataset Collection (SNAP) to find the shortest paths from randomly selected locations and compute measures for the network such as degree centrality, average distance, degree distribution, and graph densities. 

# IMPORTANT:
### Make sure to do this before running!!!
- Download the 'california_roads.txt.zip' and place the unzipped file '"california_roads.txt'" exactly spelled like that in each measure folder (Centrality_measure, Average_DistanceMeasure, Degree_Distribution, DensityMeasure, Shortest_Distance_Measure) to make sure that the program runs.
- 
### Features: 
- average distance between pre-computed cities
- graph creation
- io to read file
- shortest path-Dijkstra
- centrality and centrality distributions
- graph densities
  
### How to Run the Project
Make sure to have Rust and Cargo installed.
Under `Dependencies` in the .toml file we add in: 

petgraph = "0.6"
rayon = "1.5.0"
rand = "0.8"
itertools = "0.10.1"

Clone/download the project.
Navigate to the project directory and run cargo build to compile the project.
You can then run the project using cargo run.
## *Again, be sure to have the “california_roads.txt” file in the same directory for all features/files that calculate different measures of the network.* 

