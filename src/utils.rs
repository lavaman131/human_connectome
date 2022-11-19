use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn matrix_parser(path: &str) -> Vec<Vec<f64>> {
    let file = File::open(path).expect("Failed to read file.");
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
    connectivity_matrix
}

pub fn save_results(
    edge_weights: &HashMap<usize, HashMap<(usize, usize), Vec<f64>>>,
) -> std::io::Result<()> {
    for (k, d) in edge_weights.iter() {
        let file = File::create(String::from("face_weights_") + &k.to_string() + "_cycles.txt")
            .expect("Failed to create file.");
        let mut writer = BufWriter::new(file);
        for (key, val) in d.iter() {
            let weights = val
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(&mut writer, "{:?} {}", key, weights).expect("Failed to write line.");
        }
    }
    Ok(())
}
