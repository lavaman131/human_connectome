#![allow(non_snake_case)]
use std::collections::HashMap;
// assumption that graph is undirected
pub struct WeightedGraph {
    pub weighted_adjacency_matrix: Vec<Vec<f64>>
}

impl WeightedGraph {
    pub fn new(weighted_adjacency_matrix: Vec<Vec<f64>>) -> Self {
        WeightedGraph {
            weighted_adjacency_matrix
        }
    }

    pub fn count_n_sided_polygons(&self) -> HashMap<i32, i32> {
        let mut num_sides: HashMap<i32, i32> = HashMap::from([
            (3, 0),
            (4, 0),
            (5, 0)
        ]);
        /* Algorithm:
            1. raise matrix to the power of n
            2. divide by 2*n since graph is undirected and every vertex is counted
        */
        
        for (k, _) in &num_sides.clone() {
            let mut A: Vec<Vec<f64>> = self.weighted_adjacency_matrix.clone();
            // convert to binary representation
            for i in 0..A.len() {
                for j in 0..A[i].len() {
                    A[i][j] = if (A[i][j] as f32) != 0.0 { 1.0 } else { 0.0 };
                }
            }
            A = Self::power_matrix(A, *k);
            let trace: f64 = Self::find_trace(&A);
            let mut factorial: i32 = 1;
            for i in 1..=*k {
                factorial *= i;
            }
            let num_polygons = trace / (factorial as f64);
            *num_sides.get_mut(k).unwrap() = num_polygons as i32;
            
        }
        num_sides
    }

    fn power_matrix(mut A: Vec<Vec<f64>>, power: i32) -> Vec<Vec<f64>> {
        // finds matrix A to the power of n
        let B: Vec<Vec<f64>> = A.clone();
        for _ in 1..power {
            A = Self::matmul(&A, &B);
        }
        A
    }

    fn matmul(A: &Vec<Vec<f64>>, B: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        // first check if matmul can happen
        assert_eq!(A[0].len(), B.len());

        // create zero matrix for matmul
        let mut C: Vec<Vec<f64>> = vec![vec![0.0_f64; A[0].len()]; A.len()];

        for i in 0..C.len() {
            for j in 0..C[0].len() {
                for k in 0..C[0].len() {
                    C[i][j] += A[i][k] * B[k][j];
                }
            }
        }
        C
    }

    fn find_trace(A: &Vec<Vec<f64>>) -> f64 {
        let mut trace: f64 = 0.0;
        for i in 0..A.len() {
            trace += A[i][i];
        }
        trace
    }
}