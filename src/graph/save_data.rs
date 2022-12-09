use crate::graph::WeightedGraph;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufWriter,
    io::Write,
    path::Path,
};

impl WeightedGraph {
    pub fn save_edges(&self, folder: &str) -> std::io::Result<()> {
        if let false = Path::new(folder).exists() {
            fs::create_dir_all(folder).expect("Failed to create folder.");
        }
        let file = File::create(Path::new((String::from(folder) + "/edges.txt").as_str()))
            .expect("Failed to create file.");
        let mut writer = BufWriter::new(file);
        for i in 0..self.weighted_adjacency_matrix.len() {
            for j in 0..self.weighted_adjacency_matrix[i].len() {
                writeln!(&mut writer, "{:?}", (i, j)).expect("Failed to write line.");
            }
        }
        Ok(())
    }

    pub fn save_face_weights(
        &self,
        folder: &str,
        edges_hashmap: &HashMap<usize, Vec<Vec<(usize, usize)>>>,
    ) -> std::io::Result<()> {
        let face_weights: HashMap<usize, HashMap<(usize, usize), Vec<f64>>> =
            self.get_face_weights(edges_hashmap);
        if let false = Path::new(folder).exists() {
            fs::create_dir_all(folder).expect("Failed to create folder.");
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
                    .join("    ");
                writeln!(&mut writer, "{:?}    {}", key, weights).expect("Failed to write line.");
            }
        }
        Ok(())
    }

    // file with edges in a face
    pub fn save_face_edges(
        &self,
        folder: &str,
        edges_hashmap: &HashMap<usize, Vec<Vec<(usize, usize)>>>,
    ) -> std::io::Result<()> {
        if let false = Path::new(folder).exists() {
            fs::create_dir_all(folder).expect("Failed to create folder.");
        }
        for (k, d) in edges_hashmap.iter() {
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
                    .join("    ");
                writeln!(&mut writer, "{}", weights).expect("Failed to write line.");
            }
        }
        Ok(())
    }

    // loop through faces and find parallel edges to a given edge
    // parallel edge is when two edges have no common vertices
    pub fn save_parallel_edges(
        &self,
        folder: &str,
        edges_hashmap: &HashMap<usize, Vec<Vec<(usize, usize)>>>,
    ) -> std::io::Result<()> {
        if let false = Path::new(folder).exists() {
            fs::create_dir_all(folder).expect("Failed to create folder.");
        }
        for (k, d) in edges_hashmap.iter() {
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
                            .join("    ");
                        writeln!(&mut writer, "{}", parallel_edges).expect("Failed to write line.");
                    }
                }
            }
        }
        Ok(())
    }

    pub fn save_incident_edges(&self, folder: &str) -> std::io::Result<()> {
        let incident_edges: HashMap<usize, Vec<(usize, usize)>> = self.get_incident_edges();
        if let false = Path::new(folder).exists() {
            fs::create_dir_all(folder).expect("Failed to create folder.");
        }
        let file = File::create(Path::new(
            (String::from(folder) + "/incident_edges" + ".txt").as_str(),
        ))
        .expect("Failed to create file.");
        let mut writer = BufWriter::new(file);
        for (k, v) in incident_edges {
            let mut v = v
                .iter()
                .map(|x| format!("{:?}", x))
                .collect::<Vec<String>>()
                .join("    ");
            v = k.to_string() + "    " + &v;
            writeln!(&mut writer, "{}", v).expect("Failed to write line.");
        }
        Ok(())
    }
}
