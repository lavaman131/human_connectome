#![allow(non_snake_case)]
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

/// Implementation of Weighted Graph Data Structure
pub struct WeightedGraph {
    pub weighted_adjacency_matrix: Vec<Vec<f64>>,
}

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

    pub fn sparsify_matrix(&mut self, thresh: f64) {
        for i in 0..self.weighted_adjacency_matrix.len() {
            for j in 0..self.weighted_adjacency_matrix[i].len() {
                if self.weighted_adjacency_matrix[i][j].abs() < thresh {
                    self.weighted_adjacency_matrix[i][j] = 0.0;
                }
            }
        }
    }

    /// Returns binary adjacency matrix representation
    pub fn get_adjacency_matrix(&self) -> Vec<Vec<i32>> {
        let num_rows: usize = self.weighted_adjacency_matrix.len();
        let num_cols: usize = self.weighted_adjacency_matrix[0].len();
        let mut adjacency_matrix: Vec<Vec<i32>> = vec![vec![0; num_cols]; num_rows];
        for i in 0..num_rows {
            for j in 0..num_cols {
                if self.weighted_adjacency_matrix[i][j].abs() > 0.0 {
                    adjacency_matrix[i][j] = 1;
                }
            }
        }
        adjacency_matrix
    }

    /// Returns adjacency list representation
    pub fn get_adjacency_list(&self) -> Vec<Vec<usize>> {
        let num_rows: usize = self.weighted_adjacency_matrix.len();
        let num_cols: usize = self.weighted_adjacency_matrix[0].len();
        let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; num_rows];
        for u in 0..num_rows {
            for v in 0..num_cols {
                if self.weighted_adjacency_matrix[u][v].abs() > 0.0 {
                    adjacency_list[u].push(v);
                }
            }
        }
        adjacency_list
    }

    pub fn get_incident_edges(&self) -> HashMap<usize, Vec<(usize, usize)>> {
        let adj_list: Vec<Vec<usize>> = self.get_adjacency_list();
        let mut incident_edges: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        for u in 0..adj_list.len() {
            for v in 0..adj_list[u].len() {
                incident_edges.entry(v).or_insert(Vec::new()).push((u, v));
            }
        }
        incident_edges
    }

    pub fn get_edges_hashmap(
        &self,
        n_cycles: Vec<usize>,
    ) -> HashMap<usize, Vec<Vec<(usize, usize)>>> {
        let mut edges_hashmap: HashMap<usize, Vec<Vec<(usize, usize)>>> = HashMap::new();
        for n in n_cycles.iter() {
            edges_hashmap.insert(*n, self.find_n_cycles(*n));
        }
        edges_hashmap
    }

    pub fn get_face_weights(
        &self,
        edges_hashmap: &HashMap<usize, Vec<Vec<(usize, usize)>>>,
    ) -> HashMap<usize, HashMap<(usize, usize), Vec<f64>>> {
        let mut face_weights: HashMap<usize, HashMap<(usize, usize), Vec<f64>>> = HashMap::new();
        for n in edges_hashmap.keys() {
            let mut h: HashMap<(usize, usize), Vec<f64>> = HashMap::new();
            for cycle in edges_hashmap[n].iter() {
                let mut face_sum: f64 = 0.0;
                for (u, v) in cycle.iter() {
                    face_sum += self.weighted_adjacency_matrix[*u][*v];
                }
                for (u, v) in cycle.iter() {
                    h.entry((*u, *v)).or_insert(Vec::new()).push(face_sum);
                }
            }
            face_weights.insert(*n, h);
        }
        face_weights
    }

    fn find_n_cycles(&self, n: usize) -> Vec<Vec<(usize, usize)>> {
        let graph: Vec<Vec<i32>> = self.get_adjacency_matrix();
        let V: usize = graph.len();

        // validate input
        if V < n {
            return Vec::new();
        }

        let mut marked: Vec<bool> = vec![false; V];
        let mut edges_list: Vec<(usize, usize)> = Vec::new();
        let mut valid_edges_list: Vec<Vec<(usize, usize)>> = Vec::new();
        for i in 0..(V - (n - 1)) {
            Self::dfs(
                &graph,
                &mut marked,
                n - 1,
                i,
                i,
                V,
                &mut edges_list,
                &mut valid_edges_list,
            );
            marked[i] = true;
        }
        // only return non-duplicate edges (doesn't matter but choose the first cycle to keep)
        let mut duplicates_removed: Vec<Vec<(usize, usize)>> = Vec::new();
        for i in valid_edges_list.into_iter().step_by(2) {
            duplicates_removed.push(i);
        }
        return duplicates_removed;
    }

    // helper function that finds all cycles in a graph of V vertices
    fn dfs(
        graph: &Vec<Vec<i32>>,
        marked: &mut Vec<bool>,
        n: usize,
        u: usize,
        start: usize,
        V: usize,
        edges_list: &mut Vec<(usize, usize)>,
        valid_edges_list: &mut Vec<Vec<(usize, usize)>>,
    ) {
        // mark the current node
        marked[u] = true;

        // if length of path is equal to n
        if n == 0 {
            // if there is an edge from current vertex to starting vertex
            if graph[u][start] == 1 {
                edges_list.push((u, start));
                valid_edges_list.push(edges_list.clone());
                edges_list.pop();
            }
        }
        // if cycle length not reached
        else {
            // Recur for all the vertices adjacent to current vertex
            for v in 0..V {
                // consider current vertex only if it is adjacent to parent and
                // current vertex is not marked
                if graph[u][v] == 1 && !marked[v] {
                    edges_list.push((u, v));
                    Self::dfs(
                        graph,
                        marked,
                        n - 1,
                        v,
                        start,
                        V,
                        edges_list,
                        valid_edges_list,
                    );
                    edges_list.pop();
                }
            }
        }

        // unmark current vertex
        marked[u] = false;
    }

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

    fn to_tuple(input: String) -> (usize, usize) {
        // expects input like "(0, 0)"
        // splits like ["", "0", "", "0"]
        let input = input.trim().to_string();
        let list: Vec<&str> = input.split(['(', ',', ' ', ')']).collect();
        let a: usize = list[1].parse().expect("Failed to convert to number.");
        let b: usize = list[list.len() - 2]
            .parse()
            .expect("Failed to convert to number.");
        (a, b)
    }

    pub fn load_edges(file_path: &str) -> Vec<(usize, usize)> {
        let file = File::open(file_path).expect("Failed to read file.");
        let reader = BufReader::new(file);
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for line in reader.lines() {
            if let Ok(val) = line {
                edges.push(Self::to_tuple(val))
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
                    let edge: (usize, usize) = Self::to_tuple(vals[0].to_string());
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
                    let val: Vec<(usize, usize)> = val
                        .into_iter()
                        .map(|x| Self::to_tuple(x.to_string()))
                        .collect();
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
                    let val: Vec<(usize, usize)> = val
                        .into_iter()
                        .map(|x| Self::to_tuple(x.to_string()))
                        .collect();
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
                    .map(|x| Self::to_tuple(x.to_string()))
                    .collect();
                incident_edges.insert(vertex, val);
            }
        }
        incident_edges
    }

    pub fn calculate_curvature(
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
}
