//! Dag Engine is dagrs's main body

use crate::{
    error_handler::{DagError, FormatErrorMark},
    graph::Graph,
    task::Task,
};
use std::{collections::HashMap, fs::File, io::Read};
use yaml_rust::YamlLoader;

/// dagrs's function is wrapped in DagEngine struct
pub struct DagEngine {
    /// Store all tasks' infos
    tasks: HashMap<String, Task>,
    /// Store dependency relations
    rely_graph: Graph,
}

impl DagEngine {
    /// Allocate a new DagEngine
    /// 
    /// # Example
    /// ```
    /// let dagrs = DagEngine::new();
    /// ```
    /// This function is usually used with `run`.
    pub fn new() -> DagEngine {
        DagEngine {
            tasks: HashMap::new(),
            rely_graph: Graph::new(),
        }
    }

    /// Do dagrs's job
    /// 
    /// # Example
    /// ```
    /// let dagrs = DagEngine::new();
    /// dagrs.run("test/test_dag.yaml");
    /// ```
    pub fn run(&mut self, tasks_list: &str) -> Result<bool, DagError> {
        self.read_tasks(tasks_list)?;
        self.create_graph()?;
        Ok(self.check_dag())
    }

    /// Read tasks into engine throuh yaml
    /// 
    /// # Example
    /// ```
    /// let yaml_tasks = dagrs.read_task("test/test.yaml");
    /// ```
    /// This operation will read all info in yaml file into `dagrs.tasks` if no error occurs.
    fn read_tasks(&mut self, filename: &str) -> Result<(), DagError> {
        let mut yaml_cont = String::new();

        let mut yaml_file = File::open(filename)?;
        yaml_file.read_to_string(&mut yaml_cont)?;

        // Parse Yaml
        let yaml_tasks = YamlLoader::load_from_str(&yaml_cont)?;
        let yaml_tasks = yaml_tasks[0]["dagrs"]
            .as_hash()
            .ok_or(DagError::format_error("", FormatErrorMark::StartWordError))?;

        // Read tasks
        for (v, w) in yaml_tasks {
            let id = v.as_str().unwrap(); // .ok_or(DagError::form("task id error"))?;
            let task = Task::from_yaml(id, w)?;

            self.tasks.insert(id.to_owned(), task);
        }

        Ok(())
    }

    /// create rely map between tasks
    /// 
    /// # Example
    /// ```
    /// dagrs.create_graph();
    /// ```
    /// This operation will initialize `dagrs.rely_graph` if no error occurs.
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

            for rely_task_id in task.get_rely_list() {
                let rely_index = self
                    .rely_graph
                    .find_index_by_id(&rely_task_id)
                    .ok_or(DagError::format_error(id, FormatErrorMark::RelyIDIllegal))?;

                self.rely_graph.add_edge(index, rely_index);
            }
        }

        Ok(())
    }

    /// Check whether it's DAG or not
    /// 
    /// # Example
    /// ```
    /// dagrs.check_dag();
    /// ```
    /// This opeartions will judge the graph and give out a execution sequence if possible.
    fn check_dag(&self) -> bool {
        self.rely_graph.topo_sort()
    }
}