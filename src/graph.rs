mod algorithms;
mod creation_routines;
mod extract_data;
mod load_data;
mod process_graph;
mod save_data;

/// Weighted Graph Data Structure
pub struct WeightedGraph {
    pub weighted_adjacency_matrix: Vec<Vec<f64>>,
}
