extern crate human_connectome;
pub use human_connectome::graph::WeightedGraph;
pub use human_connectome::stats;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut graph_asd: WeightedGraph =
        WeightedGraph::new_from_all(human_connectome::DATA_DIR, "ASD");
    graph_asd.sparsify_matrix(0.2);

    let curvatures_asd = load_and_calc_curvature(
        &graph_asd.weighted_adjacency_matrix,
        &(human_connectome::SAVE_DIR.to_owned() + "ASD/"),
    );

    let mut graph_td: WeightedGraph = WeightedGraph::new_from_all(human_connectome::DATA_DIR, "TD");
    graph_td.sparsify_matrix(0.2);

    let curvatures_td = load_and_calc_curvature(
        &graph_td.weighted_adjacency_matrix,
        &(human_connectome::SAVE_DIR.to_owned() + "TD/"),
    );

    let curvature_diff: Vec<(String, String, f64)> = get_curvature_diff(
        &(human_connectome::DATA_DIR.to_owned() + "region_names.txt"),
        &curvatures_asd,
        &curvatures_td,
    );

    // find mean and standard deviation of curvature differences
    let mut diffs: Vec<f64> = Vec::new();
    for d in curvature_diff.iter() {
        diffs.push(d.2);
    }
    let mu: f64 = stats::mean(&diffs);
    let sigma: f64 = stats::std_dev(&diffs);

    // filter out by at least 2 standard deviations (statistically significant) away from the mean
    let mut filtered_curvature_diffs: Vec<(String, String, f64)> = Vec::new();
    for c_diff in curvature_diff.iter() {
        if c_diff.2.abs() >= (mu + 2.0 * sigma) {
            filtered_curvature_diffs.push(c_diff.clone());
        }
    }

    filtered_curvature_diffs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    println!("Top 10 Statistically significant curvature differences:");

    println!("5 Most Negative Curvatures:");
    for c_diff in filtered_curvature_diffs[..5].iter() {
        println!(
            "Regions: {:?}\tCurvature Difference: {}",
            (&c_diff.0, &c_diff.1),
            c_diff.2
        );
    }

    println!("5 Most Positive Curvatures:");
    for c_diff in filtered_curvature_diffs[filtered_curvature_diffs.len() - 5..].iter() {
        println!(
            "Regions: {:?}\tCurvature Difference: {}",
            (&c_diff.0, &c_diff.1),
            c_diff.2
        );
    }
}

pub fn load_and_calc_curvature(
    weighted_adjacency_matrix: &Vec<Vec<f64>>,
    folder: &str,
) -> HashMap<(usize, usize), f64> {
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

    let curvatures: HashMap<(usize, usize), f64> = WeightedGraph::calculate_curvature(
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
    curvatures_asd: &HashMap<(usize, usize), f64>,
    curvatures_td: &HashMap<(usize, usize), f64>,
) -> Vec<(String, String, f64)> {
    let file = File::open(curvature_labels_file_path).expect("Failed to read file.");
    let reader = BufReader::new(file);
    let mut region_names: Vec<String> = Vec::new();
    for line in reader.lines() {
        if let Ok(val) = line {
            region_names.push(val);
        }
    }

    let mut curvature_diff: Vec<(String, String, f64)> = Vec::new();
    for k in curvatures_asd.keys() {
        // typically_developing - autism_developed
        curvature_diff.push((
            region_names[k.0].clone(),
            region_names[k.1].clone(),
            curvatures_td[k] - curvatures_asd[k],
        ));
    }

    curvature_diff
}
