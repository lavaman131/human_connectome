extern crate human_connectome;
pub use human_connectome::graph::WeightedGraph;
use std::collections::HashMap;
use std::{process, vec};


fn main() {
    let mut graph_asd: WeightedGraph = WeightedGraph::new_from_all(
        human_connectome::DATA_DIR,
        "ASD",
    );
    graph_asd.sparsify_matrix(0.2);
    save_graph_stats(&graph_asd, "ASD");

    let mut graph_td: WeightedGraph = WeightedGraph::new_from_all(
        human_connectome::DATA_DIR,
        "TD",
    );
    graph_td.sparsify_matrix(0.2);
    save_graph_stats(&graph_td, "TD");
}

fn save_graph_stats(graph: &WeightedGraph, patient_code: &str) {
    let edges_hashmap: HashMap<usize, Vec<Vec<(usize, usize)>>> =
        graph.get_edges_hashmap(vec![3, 4, 5]);

    if let Err(e) = graph.save_edges(
        &(human_connectome::SAVE_DIR.to_owned() + patient_code),
    ) {
        println!("Failed with error: {e}");
        process::exit(1);
    }

    if let Err(e) = graph.save_face_weights(
        &(human_connectome::SAVE_DIR.to_owned() + patient_code),
        &edges_hashmap,
    ) {
        println!("Failed with error: {e}");
        process::exit(1);
    }
    if let Err(e) = graph.save_face_edges(
        &(human_connectome::SAVE_DIR.to_owned() + patient_code),
        &edges_hashmap,
    ) {
        println!("Failed with error: {e}");
        process::exit(1);
    }
    if let Err(e) = graph.save_parallel_edges(
        &(human_connectome::SAVE_DIR.to_owned() + patient_code),
        &edges_hashmap,
    ) {
        println!("Failed with error: {e}");
        process::exit(1);
    }

    if let Err(e) = graph.save_incident_edges(
        &(human_connectome::SAVE_DIR.to_owned() + patient_code),
    ) {
        println!("Failed with error: {e}");
        process::exit(1);
    }
}
