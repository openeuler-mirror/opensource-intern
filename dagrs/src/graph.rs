/*
 * @Author: Yinwhe
 * @Date: 2022-01-25 17:46:18
 * @LastEditors: Yinwhe
 * @LastEditTime: 2022-01-26 23:26:05
 * @Description: Graph records dependency relations between tasks
 * @Copyright: Copyright (c) 2022
 */

use bimap::BiMap;

#[derive(Debug)]
pub struct Graph {
    nodes: BiMap<String, usize>, // Record node id and it's index
    adj: Vec<Vec<usize>>, // Adjacency list
    indegree: Vec<usize>, // Node's indegree, used for topo sort
}

impl Graph {
    /// Allocate an empty graph
    pub fn new() -> Graph {
        Graph {
            nodes: BiMap::new(),
            adj: Vec::new(),
            indegree: Vec::new(),
        }
    }

    /// Set graph size, size is the number of tasks
    pub fn set_graph_size(&mut self, size: usize) {
        self.adj.resize(size, Vec::new());
        self.indegree.resize(size, 0)
    }

    /// Add a node into the graph, repetitive add can cause errors
    pub fn add_node(&mut self, id: &str) {
        let index = self.nodes.len();
        self.nodes.insert(id.to_owned(), index);
    }

    /// Add an edge into the graph
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].push(w);
        self.indegree[w] += 1;
    }

    pub fn find_index_by_id(&self, id: &str) -> Option<usize> {
        self.nodes.get_by_left(id).map(|i| i.to_owned())
    }

    pub fn find_id_by_index(&self, index: usize) -> Option<String> {
        self.nodes.get_by_right(&index).map(|n| n.to_string())
    }

    pub fn get_node_num(&self) -> usize {
        self.nodes.len()
    }

    /// Do topo sort in graph, returns true if DAG
    pub fn topo_sort(&self) -> bool {
        let mut queue = Vec::new();
        let mut indegree = self.indegree.clone();
        let mut count = 0;
        let mut sequence = String::new();

        indegree
            .iter()
            .enumerate()
            .map(|(index, &degree)| {
                if degree == 0 {
                    queue.push(index)
                }
            })
            .count();

        while !queue.is_empty() {
            let v = queue.pop().unwrap();

            sequence.push_str(&format!(" -> {}", self.find_id_by_index(v).unwrap()));
            count += 1;

            self.adj[v]
                .iter()
                .map(|&index| {
                    indegree[index] -= 1;
                    if indegree[index] == 0 {
                        queue.push(index)
                    }
                })
                .count();
        }

        if count < self.get_node_num() {
            println!("LOOP Detected");
            false
        } else {
            println!("[Start]{} -> [End]", sequence);
            true
        }
    }
}