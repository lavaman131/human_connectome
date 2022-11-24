extern crate human_connectome;
pub use human_connectome::graph::WeightedGraph;
pub use human_connectome::utils;
use std::collections::HashMap;
use std::{process, vec};

fn main() {
    let matrix: Vec<Vec<f64>> =
        utils::load_data::matrix_parser("../../UCLA_Autism/ASD62B_DTI_connectivity_matrix_file.txt");
    let graph: WeightedGraph = WeightedGraph::new(matrix);
    let edges_hashmap: HashMap<usize, Vec<Vec<(usize, usize)>>> =
        graph.get_edges_hashmap(vec![3, 4, 5]);

    if let Err(e) =
        utils::save_data::save_face_weights("../../results", &graph.get_face_weights(&edges_hashmap))
    {
        println!("Failed with error: {e}");
        process::exit(1);
    }
    if let Err(e) = utils::save_data::save_face_edges("../../results", &edges_hashmap) {
        println!("Failed with error: {e}");
        process::exit(1);
    }

    if let Err(e) = utils::save_data::save_parallel_edges("../../results", &edges_hashmap) {
        println!("Failed with error: {e}");
        process::exit(1);
    }
}
