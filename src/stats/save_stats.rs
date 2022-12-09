use std::{collections::HashMap, process};

use crate::graph::WeightedGraph;

pub fn save_graph_stats(graph: &WeightedGraph, patient_code: &str) {
    let edges_hashmap: HashMap<usize, Vec<Vec<(usize, usize)>>> =
        graph.get_edges_hashmap(vec![3, 4, 5]);

    if let Err(e) = graph.save_edges(&(crate::SAVE_DIR.to_owned() + patient_code)) {
        println!("Failed with error: {e}");
        process::exit(1);
    }

    if let Err(e) =
        graph.save_face_weights(&(crate::SAVE_DIR.to_owned() + patient_code), &edges_hashmap)
    {
        println!("Failed with error: {e}");
        process::exit(1);
    }
    if let Err(e) =
        graph.save_face_edges(&(crate::SAVE_DIR.to_owned() + patient_code), &edges_hashmap)
    {
        println!("Failed with error: {e}");
        process::exit(1);
    }
    if let Err(e) =
        graph.save_parallel_edges(&(crate::SAVE_DIR.to_owned() + patient_code), &edges_hashmap)
    {
        println!("Failed with error: {e}");
        process::exit(1);
    }

    if let Err(e) = graph.save_incident_edges(&(crate::SAVE_DIR.to_owned() + patient_code)) {
        println!("Failed with error: {e}");
        process::exit(1);
    }
}
