use super::WeightedGraph;
impl WeightedGraph {
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
}
