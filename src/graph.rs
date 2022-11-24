#![allow(non_snake_case)]
use std::collections::HashMap;

/// Implementation of Weighted Graph Data Structure
pub struct WeightedGraph {
    pub weighted_adjacency_matrix: Vec<Vec<f64>>,
}

impl WeightedGraph {
    /// Creates new WeightedGraph structure
    pub fn new(weighted_adjacency_matrix: Vec<Vec<f64>>) -> Self {
        WeightedGraph {
            weighted_adjacency_matrix,
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

    pub fn get_edges_hashmap(
        &self,
        n_cycles: Vec<usize>,
    ) -> HashMap<usize, Vec<Vec<(usize, usize)>>> {
        let mut edges_hashmap: HashMap<usize, Vec<Vec<(usize, usize)>>> = HashMap::new();
        for n in n_cycles.iter() {
            edges_hashmap.insert(*n, self.count_n_cycles(*n));
        }
        edges_hashmap
    } 

    pub fn count_n_cycles(&self, n: usize) -> Vec<Vec<(usize, usize)>> {
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
}

#[cfg(test)]
mod test {
    use super::WeightedGraph;
    #[test]
    fn check_n_cycles() {
        let graph: WeightedGraph = WeightedGraph::new(vec![
            vec![0.0, 1.0, 1.0],
            vec![1.0, 0.0, 1.0],
            vec![1.0, 1.0, 0.0],
        ]);

        let graph2: WeightedGraph = WeightedGraph::new(vec![
            vec![0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 0.0, 1.0, 0.0, 1.0],
            vec![0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 0.0, 1.0, 0.0, 1.0],
            vec![0.0, 1.0, 0.0, 1.0, 0.0],
        ]);
    }
}
