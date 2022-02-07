//! Graph stores dependency relations

use bimap::BiMap;

#[derive(Debug)]
/// Graph Struct
pub struct Graph {
    /// Record node id and it's index
    nodes: BiMap<String, usize>,
    /// Adjacency list
    adj: Vec<Vec<usize>>,
    /// Node's indegree, used for topo sort
    indegree: Vec<usize>,
}

impl Graph {
    /// Allocate an empty graph
    /// 
    /// # Example
    /// ```
    /// let g = Grapg::new();
    /// ```
    pub fn new() -> Graph {
        Graph {
            nodes: BiMap::new(),
            adj: Vec::new(),
            indegree: Vec::new(),
        }
    }

    /// Set graph size, size is the number of tasks
    /// 
    /// # Example
    /// ```
    /// let size = 10; // 10 nodes
    /// g.set_graph_size(size);
    /// ```
    pub fn set_graph_size(&mut self, size: usize) {
        self.adj.resize(size, Vec::new());
        self.indegree.resize(size, 0)
    }

    /// Add a node into the graph
    /// 
    /// This operation will create a mapping between ID and its index.
    /// 
    /// # Example
    /// ```
    /// g.add_node("Node1");
    /// ```
    /// **Note:** `id` won't get repeated in dagrs,
    /// since yaml parser will overwrite its info if a task's ID repeats.
    pub fn add_node(&mut self, id: &str) {
        let index = self.nodes.len();
        self.nodes.insert(id.to_owned(), index);
    }

    /// Add an edge into the graph
    /// 
    /// ```Example
    /// g.add_edge(0, 1);
    /// ```
    /// Above operation adds a arrow from node 0 to node 1,
    /// which means task 0 shall be executed before task 1.
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].push(w);
        self.indegree[w] += 1;
    }

    /// Find a task's index by its ID
    pub fn find_index_by_id(&self, id: &str) -> Option<usize> {
        self.nodes.get_by_left(id).map(|i| i.to_owned())
    }

    /// Find a task's ID by its index
    pub fn find_id_by_index(&self, index: usize) -> Option<String> {
        self.nodes.get_by_right(&index).map(|n| n.to_string())
    }

    /// Get number of nodes in grapg
    pub fn get_node_num(&self) -> usize {
        self.nodes.len()
    }

    /// Do topo sort in graph, returns true if DAG
    /// 
    /// # Example
    /// ```
    /// g.topo_sort();
    /// ```
    /// This operation will judge whether graph is a DAG or not, 
    /// returns true if yes, and false if no.
    /// 
    /// This function has output, if graph is a DAG, it will print a possible execution sequence,
    /// or it will print `Loop Detected`.
    /// 
    /// **Note**: this function can only be called after graph's initialization (add nodes and edges, etc.) is done.
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
