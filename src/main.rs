pub mod graph;
use graph::WeightedGraph;
pub mod utils;
use std::process;

fn main() {
    let matrix: Vec<Vec<f64>> =
        utils::matrix_parser("UCLA_Autism/ASD38D_DTI_connectivity_matrix_file.txt");
    let graph: WeightedGraph = WeightedGraph::new(matrix);
    if let Err(e) = utils::save_results(&graph.get_face_weights(vec![3, 4, 5])) {
        println!("Failed with error: {e}");
        process::exit(1);
    }
}
