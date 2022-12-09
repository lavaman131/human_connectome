#![allow(non_snake_case)]
use super::WeightedGraph;

impl WeightedGraph {
    pub fn find_n_cycles(&self, n: usize) -> Vec<Vec<(usize, usize)>> {
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
