#![allow(non_snake_case)]
use super::WeightedGraph;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

impl WeightedGraph {
    /// Creates new WeightedGraph structure from vector
    pub fn new_from_vec(weighted_adjacency_matrix: Vec<Vec<f64>>) -> Self {
        WeightedGraph {
            weighted_adjacency_matrix,
        }
    }
    /// Creates new WeightedGraph structure from txt file
    pub fn new_from_file(path: &str) -> Self {
        let file = File::open(Path::new(path)).expect("Failed to read file.");
        let reader = BufReader::new(file);
        let mut connectivity_matrix: Vec<Vec<f64>> = Vec::new();
        for line in reader.lines() {
            if let Ok(val) = line {
                connectivity_matrix.push(
                    val.trim()
                        .split_whitespace()
                        .map(|x| x.parse::<f64>().unwrap())
                        .collect::<Vec<f64>>(),
                );
            }
        }
        for i in 0..connectivity_matrix.len() {
            for j in 0..connectivity_matrix[i].len() {
                if connectivity_matrix[i][j].is_infinite() || connectivity_matrix[i][j].is_nan() {
                    connectivity_matrix[i][j] = 0.0;
                }
            }
        }
        WeightedGraph {
            weighted_adjacency_matrix: connectivity_matrix,
        }
    }

    pub fn new_from_all(folder: &str, patient_code: &str) -> Self {
        let paths = fs::read_dir(folder).expect("Invalid path.");
        let mut graph: WeightedGraph = Self::new_from_vec(vec![vec![0.0; 264]; 264]);
        let mut num_files = 0;
        for p in paths {
            if let Ok(path) = p {
                let f = path
                    .path()
                    .to_str()
                    .expect("Failed to make path a string.")
                    .to_owned();
                let stem: Vec<&str> = f.split("/").collect();
                let stem: &str = stem[stem.len() - 1];
                if stem.starts_with(patient_code) && stem.contains("connectivity_matrix") {
                    num_files += 1;
                    let other_graph: WeightedGraph = Self::new_from_file(&f);
                    for i in 0..graph.weighted_adjacency_matrix.len() {
                        for j in 0..graph.weighted_adjacency_matrix[i].len() {
                            graph.weighted_adjacency_matrix[i][j] +=
                                other_graph.weighted_adjacency_matrix[i][j];
                        }
                    }
                }
            }
        }
        if num_files == 0 {
            panic!("No suitable files found.");
        }
        for i in 0..graph.weighted_adjacency_matrix.len() {
            for j in 0..graph.weighted_adjacency_matrix[i].len() {
                graph.weighted_adjacency_matrix[i][j] /= num_files as f64;
            }
        }
        graph
    }
}
