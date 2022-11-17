pub mod graph;
use graph::WeightedGraph;

fn main() {
    let graph: WeightedGraph = WeightedGraph::new(vec![
        vec![0.0, 1.0, 0.0, 1.0, 0.0],
        vec![1.0, 0.0, 1.0, 0.0, 1.0],
        vec![0.0, 1.0, 0.0, 1.0, 0.0],
        vec![1.0, 0.0, 1.0, 0.0, 1.0],
        vec![0.0, 1.0, 0.0, 1.0, 0.0],
    ]);
    println!("{:?}", &graph.count_cycles(4));
}
