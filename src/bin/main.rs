extern crate human_connectome;
use human_connectome::graph::WeightedGraph;
use human_connectome::stats;

fn main() {
    let mut graph_asd: WeightedGraph =
        WeightedGraph::new_from_all(human_connectome::DATA_DIR, "ASD");
    graph_asd.sparsify_matrix(0.2);

    let curvatures_asd = stats::curvature::load_and_calc_curvature(
        &graph_asd.weighted_adjacency_matrix,
        &(human_connectome::SAVE_DIR.to_owned() + "ASD/"),
    );

    let mut graph_td: WeightedGraph = WeightedGraph::new_from_all(human_connectome::DATA_DIR, "TD");
    graph_td.sparsify_matrix(0.2);

    let curvatures_td = stats::curvature::load_and_calc_curvature(
        &graph_td.weighted_adjacency_matrix,
        &(human_connectome::SAVE_DIR.to_owned() + "TD/"),
    );

    let curvature_diff: Vec<(String, String, f64)> = stats::curvature::get_curvature_diff(
        &(human_connectome::DATA_DIR.to_owned() + "region_names.txt"),
        &curvatures_asd,
        &curvatures_td,
    );

    // find mean and standard deviation of curvature differences
    let mut diffs: Vec<f64> = Vec::new();
    for d in curvature_diff.iter() {
        diffs.push(d.2);
    }
    let mu: f64 = stats::basic_stats::mean(&diffs);
    let sigma: f64 = stats::basic_stats::std_dev(&diffs);

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
