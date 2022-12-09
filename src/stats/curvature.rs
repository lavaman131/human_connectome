use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::graph::WeightedGraph;

fn calculate_curvature(
    weighted_adjacency_matrix: &Vec<Vec<f64>>,
    edges: &Vec<(usize, usize)>,
    face_weights: &HashMap<usize, HashMap<(usize, usize), Vec<f64>>>,
    incident_edges: &HashMap<usize, Vec<(usize, usize)>>,
    parallel_edges: &HashMap<usize, HashMap<(usize, usize), Vec<(usize, usize)>>>,
) -> HashMap<(usize, usize), f64> {
    // store curvatures for every edge
    let mut curvatures: HashMap<(usize, usize), f64> = HashMap::new();
    // for all edges
    for edge in edges.iter() {
        // store sum in var a
        let mut a: f64 = 0.0;
        // store sum in var b
        let mut b: f64 = 0.0;
        // store sum in var c
        let mut c: f64 = 0.0;
        for n_cycles in 3..=5 {
            if let None = face_weights.get(&n_cycles).unwrap().get(&edge) {
                continue;
            }
            // for all faces that a given edge (3, 4, and 5 cycles) is a part of
            for face_weight in face_weights
                .get(&n_cycles)
                .unwrap()
                .get(edge)
                .unwrap()
                .iter()
            {
                // take w(edge)
                let w_e: f64 = weighted_adjacency_matrix[edge.0][edge.1];
                // a += weight of edge / weight of the face edge is a part of
                a += (w_e / face_weight).abs();

                // for all parallel edges in that face
                if n_cycles != 3 {
                    if let Some(p_edges) = parallel_edges.get(&n_cycles).unwrap().get(edge) {
                        for p_e in p_edges.iter() {
                            // take w(parallel edge)
                            let w_p_e: f64 = weighted_adjacency_matrix[p_e.0][p_e.1];
                            c += ((w_e * w_p_e).abs()).sqrt() / face_weight.abs();
                        }
                    }
                }
            }
            // for vertices in edge (2)
            for v in [edge.0, edge.1].iter() {
                // sum of the weight of incident edges divided by the by number of indicent edges -> w(vertex)
                let mut w_v = 0.0;
                for list_incident_edge in incident_edges.get(v).iter() {
                    for incident_edge in list_incident_edge.iter() {
                        w_v += (weighted_adjacency_matrix[incident_edge.0][incident_edge.1]).abs();
                    }
                    w_v /= list_incident_edge.len() as f64;
                }
                // then divide w(vertex) by edge w(edge) in outermost loop
                b += w_v / weighted_adjacency_matrix[edge.0][edge.1].abs();
            }
        }
        curvatures.insert(
            *edge,
            weighted_adjacency_matrix[edge.0][edge.1].abs() * (a + b - c),
        );
    }
    curvatures
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

    let curvatures: HashMap<(usize, usize), f64> = calculate_curvature(
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
