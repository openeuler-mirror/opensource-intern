/*
 * @Author: Yinwhe
 * @Date: 2022-01-25 18:26:14
 * @LastEditors: Yinwhe
 * @LastEditTime: 2022-01-26 23:23:07
 * @Description: DAG Engine implementation
 * @Copyright: Copyright (c) 2022
 */

use crate::{error_handler::DagError, graph::Graph, task::Task};
use std::{collections::HashMap, fs::File, io::Read};
use yaml_rust::YamlLoader;

pub struct DagEngine {
    tasks: HashMap<String, Task>,
    rely_graph: Graph,
}

impl DagEngine {
    /// Allocate a new DagEngine
    pub fn new() -> DagEngine {
        DagEngine {
            tasks: HashMap::new(),
            rely_graph: Graph::new(),
        }
    }

    /// Do its job
    pub fn run(&mut self, tasks_list: &str) -> Result<bool, DagError> {
        self.read_tasks(tasks_list)?;
        self.create_graph()?;
        Ok(self.check_dag())
    }

    /// Read tasks into engine throuh yaml
    fn read_tasks(&mut self, filename: &str) -> Result<(), DagError> {
        let mut yaml_cont = String::new();

        let mut yaml_file = File::open(filename)?;
        yaml_file.read_to_string(&mut yaml_cont)?;

        // Parse Yaml
        let yaml_tasks = YamlLoader::load_from_str(&yaml_cont)?;
        let yaml_tasks = yaml_tasks[0]["dagrs"]
            .as_hash()
            .ok_or(DagError::error("format error"))?;

        // Read tasks
        for (v, w) in yaml_tasks {
            let id = v.as_str().ok_or(DagError::error("task id error"))?;
            let task = Task::from_yaml(w)?;

            self.tasks.insert(id.to_owned(), task);
        }

        Ok(())
    }

    /// create rely map between tasks
    fn create_graph(&mut self) -> Result<(), DagError> {
        let size = self.tasks.len();
        self.rely_graph.set_graph_size(size);

        // Add Node (create id - index mapping)
        self.tasks
            .iter()
            .map(|(n, _)| self.rely_graph.add_node(n))
            .count();

        // Form Graph
        for (id, task) in &self.tasks {
            let index = self.rely_graph.find_index_by_id(id).unwrap();

            task.get_rely_list()
                .iter()
                .map(|t| {
                    self.rely_graph
                        .add_edge(index, self.rely_graph.find_index_by_id(t).unwrap())
                })
                .count();
        }

        Ok(())
    }

    /// Check whether it's DAG or not
    fn check_dag(&self) -> bool {
        self.rely_graph.topo_sort()
    }
}