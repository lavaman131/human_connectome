pub mod save_data {
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::{BufWriter, Write};
    use std::path::Path;

    pub fn save_face_weights(
        folder: &str,
        face_weights: &HashMap<usize, HashMap<(usize, usize), Vec<f64>>>,
    ) -> std::io::Result<()> {
        if let false = Path::new(folder).exists() {
            fs::create_dir(folder).expect("Failed to create folder.");
        }
        for (k, d) in face_weights.iter() {
            let file = File::create(Path::new(
                (String::from(folder) + "/face_weights_" + &k.to_string() + "_cycles.txt").as_str(),
            ))
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

    // file with edges in a face
    pub fn save_face_edges(
        folder: &str,
        face_edges: &HashMap<usize, Vec<Vec<(usize, usize)>>>,
    ) -> std::io::Result<()> {
        if let false = Path::new(folder).exists() {
            fs::create_dir(folder).expect("Failed to create folder.");
        }
        for (k, d) in face_edges.iter() {
            let file = File::create(Path::new(
                (String::from(folder) + "/face_edges_" + &k.to_string() + "_cycles.txt").as_str(),
            ))
            .expect("Failed to create file.");
            let mut writer = BufWriter::new(file);
            for face in d.iter() {
                let weights = face
                    .iter()
                    .map(|x| format!("{:?}", x))
                    .collect::<Vec<String>>()
                    .join(" ");
                writeln!(&mut writer, "{:?}", weights).expect("Failed to write line.");
            }
        }
        Ok(())
    }

    // loop through faces and find parallel edges to a given edge
    // parallel edge is when two edges have no common vertices
    pub fn save_parallel_edges(
        folder: &str,
        face_edges: &HashMap<usize, Vec<Vec<(usize, usize)>>>,
    ) -> std::io::Result<()> {
        if let false = Path::new(folder).exists() {
            fs::create_dir(folder).expect("Failed to create folder.");
        }
        for (k, d) in face_edges.iter() {
            if *k != 3 {
                let file = File::create(Path::new(
                    (String::from(folder) + "/parallel_edges_" + &k.to_string() + ".txt").as_str(),
                ))
                .expect("Failed to create file.");
                let mut writer = BufWriter::new(file);
                for face in d.iter() {
                    for i in 0..face.len() {
                        let mut parallel_edges: Vec<(usize, usize)> = Vec::new();
                        for j in 0..face.len() {
                            if i != j {
                                // check for parallel edge
                                if face[i].0 != face[j].0
                                    && face[i].1 != face[j].1
                                    && face[i].0 != face[j].1
                                    && face[i].1 != face[j].0
                                {
                                    parallel_edges.push(face[j]);
                                }
                            }
                        }
                        parallel_edges.insert(0, face[i]);
                        let parallel_edges = parallel_edges
                            .iter()
                            .map(|x| format!("{:?}", x))
                            .collect::<Vec<String>>()
                            .join(" ");
                        writeln!(&mut writer, "{:?}", parallel_edges)
                            .expect("Failed to write line.");
                    }
                }
            }
        }
        Ok(())
    }

    pub fn save_incident_edges(
        folder: &str,
        incident_edges: &HashMap<usize, Vec<(usize, usize)>>,
    ) -> std::io::Result<()> {
        let file = File::create(Path::new(
            (String::from(folder) + "/incident_edges_" + ".txt").as_str(),
        ))
        .expect("Failed to create file.");
        let mut writer = BufWriter::new(file);
        for (k, v) in incident_edges {
            let mut v = v
                .iter()
                .map(|x| format!("{:?}", x))
                .collect::<Vec<String>>();
            v.insert(0, k.to_string());
            writeln!(&mut writer, "{:?}", v.join(" ")).expect("Failed to write line.");
        }
        Ok(())
    }
}

pub mod load_data {
    use serde_json;
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    pub fn matrix_parser(path: &str) -> Vec<Vec<f64>> {
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
        connectivity_matrix
    }
}
