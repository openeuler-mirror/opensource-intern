//! Dag Engine is dagrs's main body

use crate::{
    error_handler::{DagError, InnerError},
    graph::Graph,
    task::{TaskWrapper, YamlTask},
};
use std::collections::HashMap;

/// dagrs's function is wrapped in DagEngine struct
pub struct DagEngine {
    /// Store all tasks' infos
    tasks: HashMap<usize, TaskWrapper>,
    /// Store dependency relations
    rely_graph: Graph,
}

impl DagEngine {
    /// Allocate a new DagEngine.
    ///
    /// # Example
    /// ```
    /// let dagrs = DagEngine::new();
    /// ```
    pub fn new() -> DagEngine {
        DagEngine {
            tasks: HashMap::new(),
            rely_graph: Graph::new(),
        }
    }

    /// Do dagrs's job.
    ///
    /// # Example
    /// ```
    /// let dagrs = DagEngine::new();
    /// dagrs.add_task(task1);
    /// dagrs.add_task(task2);
    /// dagrs.run("test/test_dag.yaml");
    /// ```
    ///
    /// Here `task1` and `task2` are user defined task wrapped in [`TaskWrapper`].
    ///
    /// **Note:** This method must be called after all tasks have been added into dagrs.
    pub fn run(&mut self) -> Result<bool, DagError> {
        self.create_graph()?;
        Ok(self.check_dag())
    }

    /// Do dagrs's job from yaml file.
    ///
    /// # Example
    /// ```
    /// let dagrs = DagEngine::new();
    /// dagrs.run_from_yaml("test/test_dag.yaml");
    /// ```
    ///
    /// This method is similar to `run`, but read tasks from yaml file,
    /// thus no need to add tasks mannually.
    pub fn run_from_yaml(&mut self, filename: &str) -> Result<bool, DagError> {
        self.read_tasks(filename)?;
        self.run()
    }

    /// Add new task into dagrs
    ///
    /// # Example
    /// ```
    /// let dagrs = DagEngine::new();
    /// dagrs.add_task(task1);
    /// dagrs.add_task(task2);
    /// dagrs.run("test/test_dag.yaml");
    /// ```
    ///
    /// Here `task1` and `task2` are user defined task wrapped in [`TaskWrapper`].
    pub fn add_task(&mut self, task: TaskWrapper) {
        self.tasks.insert(task.get_id(), task);
    }

    /// Read tasks into engine throuh yaml
    ///
    /// # Example
    /// ```
    /// let yaml_tasks = dagrs.read_task("test/test.yaml");
    /// ```
    /// This operation will read all info in yaml file into `dagrs.tasks` if no error occurs.
    fn read_tasks(&mut self, filename: &str) -> Result<(), DagError> {
        let tasks = YamlTask::from_yaml(filename)?;
        tasks.into_iter().map(|t| self.add_task(t)).count();
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
            .map(|(&n, _)| self.rely_graph.add_node(n))
            .count();

        // Form Graph
        for (&id, task) in self.tasks.iter() {
            let index = self.rely_graph.find_index_by_id(&id).unwrap();

            for rely_task_id in task.get_rely_list() {
                // Rely task existence check
                let rely_index = self.rely_graph.find_index_by_id(&rely_task_id).ok_or(
                    DagError::inner_error(InnerError::RelyTaskIllegal(task.get_name())),
                )?;

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
        if let Some(seq) = self.rely_graph.topo_sort() {
            let seq = seq
                .into_iter()
                .map(|index| self.rely_graph.find_id_by_index(index).unwrap())
                .collect();
            self.print_seq(&seq);
            seq.iter()
                .map(|id| {
                    println!("Executing Task[name: {}]", self.tasks[id].get_name());
                    self.tasks[id].run();
                })
                .count();
            true
        } else {
            println!("Loop Detect");
            false
        }
    }

    /// Print possible execution sequnces.
    ///
    /// # Example
    /// ```
    /// if let Some(seq) = self.rely_graph.topo_sort() {
    ///     self.print_seq(&seq);
    ///     ...
    /// }
    /// ```
    fn print_seq(&self, seq: &Vec<usize>) {
        print!("[Start]");
        seq.iter()
            .map(|id| print!(" -> {}", self.tasks[id].get_name()))
            .count();
        println!(" -> [End]");
    }
}
