extern crate human_connectome;
use human_connectome::graph::WeightedGraph;
use human_connectome::stats;

fn main() {
    let mut graph_asd: WeightedGraph =
        WeightedGraph::new_from_all(human_connectome::DATA_DIR, "ASD");
    graph_asd.sparsify_matrix(0.2);
    stats::save_stats::save_graph_stats(&graph_asd, "ASD");

    let mut graph_td: WeightedGraph = WeightedGraph::new_from_all(human_connectome::DATA_DIR, "TD");
    graph_td.sparsify_matrix(0.2);
    stats::save_stats::save_graph_stats(&graph_td, "TD");
}
