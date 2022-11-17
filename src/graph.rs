#![allow(non_snake_case)]
// assumption that graph is undirected
pub struct WeightedGraph {
    pub weighted_adjacency_matrix: Vec<Vec<f64>>,
}

impl WeightedGraph {
    pub fn new(weighted_adjacency_matrix: Vec<Vec<f64>>) -> Self {
        WeightedGraph {
            weighted_adjacency_matrix,
        }
    }

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

    pub fn count_cycles(&self, n: usize) -> i32 {
        let graph: Vec<Vec<i32>> = self.get_adjacency_matrix();
        let V: usize = graph.len();
        let mut marked: Vec<bool> = vec![false; V];
        let mut count: i32 = 0;
        for i in 0..(V - (n - 1)) {
            count = Self::dfs(&graph, &mut marked, n - 1, i, i, count, V);
            marked[i] = true;
        }
        count / 2
    }

    fn dfs(
        adjacency_matrix: &Vec<Vec<i32>>,
        marked: &mut Vec<bool>,
        n: usize,
        vertex: usize,
        start: usize,
        mut count: i32,
        V: usize,
    ) -> i32 {
        marked[vertex] = true;

        if n == 0 {
            marked[vertex] = false;

            if adjacency_matrix[vertex][start] == 1 {
                count += 1;
                return count;
            } else {
                return count;
            }
        }
        for i in 0..V {
            if !marked[i] && adjacency_matrix[vertex][i] == 1 {
                count = Self::dfs(adjacency_matrix, marked, n - 1, i, start, count, V);
            }
        }
        marked[vertex] = false;
        count
    }
}
