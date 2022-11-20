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
                if self.weighted_adjacency_matrix[i][j] > 0.0 {
                    adjacency_matrix[i][j] = 1;
                }
            }
        }
        adjacency_matrix
    }

    pub fn get_face_weights(&self, n_cycles: Vec<usize>) -> HashMap<usize, HashMap<(usize, usize), Vec<f64>>> {
        let mut face_weights: HashMap<usize, HashMap<(usize, usize), Vec<f64>>> = HashMap::new();
        for n in n_cycles.iter() {
            let edges_list: Vec<Vec<(usize, usize)>> = self.count_n_cycles(*n).1;
            let mut h: HashMap<(usize, usize), Vec<f64>> = HashMap::new();
            for cycle in edges_list.iter() {
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

    /// Counts number of n-length cycles
    /// https://www.geeksforgeeks.org/cycles-of-length-n-in-an-undirected-and-connected-graph/
    pub fn count_n_cycles(&self, n: usize) -> (i32, Vec<Vec<(usize, usize)>>) {
        let graph: Vec<Vec<i32>> = self.get_adjacency_matrix();
        let V: usize = graph.len();

        // validate input
        if V < n {
            return (0, Vec::new());
        }

        let mut marked: Vec<bool> = vec![false; V];
        let mut count: i32 = 0;
        let mut edges_list: Vec<(usize, usize)> = Vec::new();
        let mut valid_edges_list: Vec<Vec<(usize, usize)>> = Vec::new();
        for i in 0..(V - (n - 1)) {
            count = Self::dfs(&graph, &mut marked, n - 1, i, i, count, V, &mut edges_list, &mut valid_edges_list);
            marked[i] = true;
        }
        if count != 0 {
            // only return non-duplicate edges (doesn't matter but choose the first cycle to keep)
            let mut duplicates_removed: Vec<Vec<(usize, usize)>> = Vec::new();
            for i in valid_edges_list.into_iter().step_by(2) {
                duplicates_removed.push(i);
            }
            return (count / 2, duplicates_removed);
        }
        return (count, Vec::new());
    }

    /// DFS helper function for finding all (n-1) length paths
    fn dfs(
        adjacency_matrix: &Vec<Vec<i32>>,
        marked: &mut Vec<bool>,
        n: usize,
        vertex: usize,
        start: usize,
        mut count: i32,
        V: usize,
        edges_list: &mut Vec<(usize, usize)>,
        valid_edges_list: &mut Vec<Vec<(usize, usize)>>
    ) -> i32 {
        marked[vertex] = true;
        if n == 0 {
            marked[vertex] = false;
            if adjacency_matrix[vertex][start] == 1 {
                edges_list.push((vertex, start));
                edges_list.insert(0, (edges_list[edges_list.len() - 1].1, edges_list[0].0));
                if edges_list[0].0 == edges_list[0].1 {
                    edges_list.remove(0);
                }
                valid_edges_list.push(edges_list.clone());
                edges_list.clear();
                count += 1;
                return count;
            } else {
                return count;
            }
        }
        for i in 0..V {
            if !marked[i] && adjacency_matrix[vertex][i] == 1 {
                edges_list.push((vertex, i));
                count = Self::dfs(
                    adjacency_matrix,
                    marked,
                    n - 1,
                    i,
                    start,
                    count,
                    V,
                    edges_list,
                    valid_edges_list
                );
            }
        }
        marked[vertex] = false;
        return count;
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
        assert_eq!(1, graph.count_n_cycles(3).0);

        let graph2: WeightedGraph = WeightedGraph::new(vec![
            vec![0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 0.0, 1.0, 0.0, 1.0],
            vec![0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 0.0, 1.0, 0.0, 1.0],
            vec![0.0, 1.0, 0.0, 1.0, 0.0],
        ]);
        assert_eq!(3, graph2.count_n_cycles(4).0);
    }
}
