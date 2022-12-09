use super::WeightedGraph;
use std::collections::HashMap;

impl WeightedGraph {
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
}
