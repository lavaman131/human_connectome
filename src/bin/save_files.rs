extern crate human_connectome;
pub use human_connectome::graph::WeightedGraph;
pub use human_connectome::utils;
use std::collections::HashMap;
use std::{process, vec};

fn main() {
    let matrix: Vec<Vec<f64>> = utils::load_data::matrix_parser(
        "/Users/alilavaee/Documents/DS210/human_connectome/UCLA_Autism/ASD38D_DTI_connectivity_matrix_file.txt",
    );
    let graph: WeightedGraph = WeightedGraph::new(matrix);
    let edges_hashmap: HashMap<usize, Vec<Vec<(usize, usize)>>> =
        graph.get_edges_hashmap(vec![3, 4, 5]);
    let incident_edges: HashMap<usize, Vec<(usize, usize)>> = graph.get_incident_edges();

    if let Err(e) = utils::save_data::save_face_weights(
        "/Users/alilavaee/Documents/DS210/human_connectome/results",
        &graph.get_face_weights(&edges_hashmap),
    ) {
        println!("Failed with error: {e}");
        process::exit(1);
    }
    if let Err(e) = utils::save_data::save_face_edges(
        "/Users/alilavaee/Documents/DS210/human_connectome/results",
        &edges_hashmap,
    ) {
        println!("Failed with error: {e}");
        process::exit(1);
    }
    if let Err(e) = utils::save_data::save_parallel_edges(
        "/Users/alilavaee/Documents/DS210/human_connectome/results",
        &edges_hashmap,
    ) {
        println!("Failed with error: {e}");
        process::exit(1);
    }

    if let Err(e) = utils::save_data::save_incident_edges(
        "/Users/alilavaee/Documents/DS210/human_connectome/results",
        &incident_edges,
    ) {
        println!("Failed with error: {e}");
        process::exit(1);
    }
}
