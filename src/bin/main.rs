extern crate human_connectome;
pub use human_connectome::graph::WeightedGraph;
pub use human_connectome::stats;
use itertools::izip;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut graph_asd: WeightedGraph = WeightedGraph::new_from_all(
        "/Users/alilavaee/Documents/DS210/human_connectome/UCLA_Autism/",
        "ASD",
    );
    graph_asd.sparsify_matrix(0.2);

    let curvatures_asd = load_and_calc_curvature(
        &graph_asd.weighted_adjacency_matrix,
        "/Users/alilavaee/Documents/DS210/human_connectome/results/ASD/",
    );

    let mut graph_td: WeightedGraph = WeightedGraph::new_from_all(
        "/Users/alilavaee/Documents/DS210/human_connectome/UCLA_Autism/",
        "TD",
    );
    graph_td.sparsify_matrix(0.2);

    let curvatures_td = load_and_calc_curvature(
        &graph_td.weighted_adjacency_matrix,
        "/Users/alilavaee/Documents/DS210/human_connectome/results/TD/",
    );

    let mut curvature_diff: HashMap<String, f64> = get_curvature_diff(
        "/Users/alilavaee/Documents/DS210/human_connectome/region_names.txt",
        &curvatures_asd,
        &curvatures_td,
    );

    println!("All nonzero curvature differences:");
    for (k, v) in curvature_diff.iter() {
        if *v != 0.0 {
            println!("Region: {}\tCurvature Difference: {}", k, v);
        }
    }

    // find mean and standard deviation of curvature differences
    let diffs: Vec<f64> = curvature_diff.values().cloned().collect();
    let mu: f64 = stats::mean(&diffs);
    let sigma: f64 = stats::std_dev(&diffs);

    println!("Statistically significant curvature differences:");
    // filter out by at least 2 standard deviations (statistically significant) away from the mean
    curvature_diff.retain(|_, v| v.abs() >= (mu + 2.0 * sigma));
    for (k, v) in curvature_diff.iter() {
        println!("Region: {}\tCurvature Difference: {}", k, v);
    }
}

pub fn load_and_calc_curvature(
    weighted_adjacency_matrix: &Vec<Vec<f64>>,
    folder: &str,
) -> Vec<f64> {
    let edges: Vec<(usize, usize)> = WeightedGraph::load_edges(&(folder.to_owned() + "edges.txt"));

    let face_weights: HashMap<usize, HashMap<(usize, usize), Vec<f64>>> =
        WeightedGraph::load_face_weights(
            &(folder.to_owned() + "face_weights_3_cycles.txt"),
            &(folder.to_owned() + "face_weights_4_cycles.txt"),
            &(folder.to_owned() + "face_weights_5_cycles.txt"),
        );

    let parallel_edges: HashMap<usize, HashMap<(usize, usize), Vec<(usize, usize)>>> =
        WeightedGraph::load_parallel_edges(
            &(folder.to_owned() + "face_edges_3_cycles.txt"),
            &(folder.to_owned() + "face_edges_4_cycles.txt"),
            &(folder.to_owned() + "face_edges_5_cycles.txt"),
        );

    let incident_edges: HashMap<usize, Vec<(usize, usize)>> =
        WeightedGraph::load_incident_edges(&(folder.to_owned() + "incident_edges.txt"));

    let curvatures: Vec<f64> = WeightedGraph::calculate_curvature(
        &weighted_adjacency_matrix,
        &edges,
        &face_weights,
        &incident_edges,
        &parallel_edges,
    );

    curvatures
}

pub fn get_curvature_diff(
    curvature_labels_file_path: &str,
    curvatures_asd: &Vec<f64>,
    curvatures_td: &Vec<f64>,
) -> HashMap<String, f64> {
    let file = File::open(curvature_labels_file_path).expect("Failed to read file.");
    let reader = BufReader::new(file);
    let mut region_names: Vec<String> = Vec::new();
    for line in reader.lines() {
        if let Ok(val) = line {
            region_names.push(val);
        }
    }
    let mut curvature_diff: HashMap<String, f64> = HashMap::new();
    for (c_asd, c_td, label) in izip!(curvatures_asd, curvatures_td, region_names) {
        // typically_developing - autism_developed
        curvature_diff.insert(label, c_td - c_asd);
    }

    curvature_diff
}
