use super::WeightedGraph;
mod helper;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use helper::to_tuple;

impl WeightedGraph {
    pub fn load_edges(file_path: &str) -> Vec<(usize, usize)> {
        let file = File::open(file_path).expect("Failed to read file.");
        let reader = BufReader::new(file);
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for line in reader.lines() {
            if let Ok(val) = line {
                edges.push(to_tuple(val))
            }
        }
        edges
    }

    pub fn load_face_weights(
        file_path_3: &str,
        file_path_4: &str,
        file_path_5: &str,
    ) -> HashMap<usize, HashMap<(usize, usize), Vec<f64>>> {
        let mut face_weights: HashMap<usize, HashMap<(usize, usize), Vec<f64>>> = HashMap::new();
        let files: Vec<File> = vec![
            File::open(file_path_3).expect("Failed to read file."),
            File::open(file_path_4).expect("Failed to read file."),
            File::open(file_path_5).expect("Failed to read file."),
        ];
        let num_cycles: [usize; 3] = [3, 4, 5];

        for (f, n) in files.into_iter().zip(num_cycles) {
            let reader = BufReader::new(f);
            let mut h: HashMap<(usize, usize), Vec<f64>> = HashMap::new();
            for line in reader.lines() {
                if let Ok(val) = line {
                    let vals: Vec<&str> = val.split("    ").collect();
                    let edge: (usize, usize) = to_tuple(vals[0].to_string());
                    let weights: Vec<f64> = vals[1..]
                        .into_iter()
                        .map(|x| (**x).parse().unwrap())
                        .collect();
                    h.insert(edge, weights);
                }
            }
            face_weights.insert(n, h);
        }
        face_weights
    }

    pub fn load_face_edges(
        file_path_3: &str,
        file_path_4: &str,
        file_path_5: &str,
    ) -> HashMap<usize, Vec<Vec<(usize, usize)>>> {
        let mut face_edges: HashMap<usize, Vec<Vec<(usize, usize)>>> = HashMap::new();
        let files: Vec<File> = vec![
            File::open(file_path_3).expect("Failed to read file."),
            File::open(file_path_4).expect("Failed to read file."),
            File::open(file_path_5).expect("Failed to read file."),
        ];
        let num_cycles: [usize; 3] = [3, 4, 5];

        for (f, n) in files.into_iter().zip(num_cycles) {
            let reader = BufReader::new(f);
            let mut vals: Vec<Vec<(usize, usize)>> = Vec::new();
            for line in reader.lines() {
                if let Ok(val) = line {
                    let val: Vec<&str> = val.split("    ").collect();
                    let val: Vec<(usize, usize)> =
                        val.into_iter().map(|x| to_tuple(x.to_string())).collect();
                    vals.push(val);
                }
            }
            face_edges.insert(n, vals);
        }
        face_edges
    }

    pub fn load_parallel_edges(
        file_path_3: &str,
        file_path_4: &str,
        file_path_5: &str,
    ) -> HashMap<usize, HashMap<(usize, usize), Vec<(usize, usize)>>> {
        let mut parallel_edges: HashMap<usize, HashMap<(usize, usize), Vec<(usize, usize)>>> =
            HashMap::new();
        let files: Vec<File> = vec![
            File::open(file_path_3).expect("Failed to read file."),
            File::open(file_path_4).expect("Failed to read file."),
            File::open(file_path_5).expect("Failed to read file."),
        ];
        let num_cycles: [usize; 2] = [4, 5];

        for (f, n) in files.into_iter().zip(num_cycles) {
            let reader = BufReader::new(f);
            let mut h: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
            for line in reader.lines() {
                if let Ok(val) = line {
                    let val: Vec<&str> = val.split("    ").collect();
                    let val: Vec<(usize, usize)> =
                        val.into_iter().map(|x| to_tuple(x.to_string())).collect();
                    h.insert(val[0], val[1..].to_vec());
                }
            }
            parallel_edges.insert(n, h);
        }
        parallel_edges
    }

    pub fn load_incident_edges(file_path: &str) -> HashMap<usize, Vec<(usize, usize)>> {
        let mut incident_edges: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        let file = File::open(file_path).expect("Failed to read file.");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(val) = line {
                let val: Vec<&str> = val.split("    ").collect();
                let vertex: usize = val[0].parse().unwrap();
                let val: Vec<(usize, usize)> = val[1..]
                    .into_iter()
                    .map(|x| to_tuple(x.to_string()))
                    .collect();
                incident_edges.insert(vertex, val);
            }
        }
        incident_edges
    }
}
