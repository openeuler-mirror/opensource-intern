//! Task Implementations, used to store task infos

use std::{collections::HashMap, fs::File, io::Read, process::exit};

use crate::error_handler::{DagError, FormatErrorMark};
use yaml_rust::{Yaml, YamlLoader};

/// Struct that implement this trait can be taken as a task accepted by DagEngine.
pub trait TaskTrait where Self:Sized {
    /// Get the ID of a task.
    fn get_ID(&self) -> String;
    /// Get the dependency list of a task.
    fn get_rely_list(&self) -> &Vec<String>;
    /// Parse all tasks from file and form a hash map (ID to task struct mapping).
    fn from_file(filename: &str) -> HashMap<String, Self>;
}

/// Task Struct
#[derive(Debug)]
pub struct Task {
    /// Task ID, must be unique
    pub ID: String,
    /// Task Name, can repeat
    pub name: String,
    /// Dependency relations, store relied tasks' ID
    pub relys: Vec<String>, 
}

impl TaskTrait for Task {
    /// Get the ID of a task
    /// 
    /// # Example
    /// ```
    /// let id = task.get_ID();
    /// println!("{}", id);
    /// ```
    fn get_ID(&self) -> String {
        self.ID.to_owned()
    }

    /// Get dependency tasks list
    /// 
    /// # Example
    /// Usually used like:
    /// ```
    /// let relys = tasks.get_rely_list();
    /// for rely_task in relys{
    ///     ...
    /// }
    /// ```
    fn get_rely_list(&self) -> &Vec<String> {
        &self.relys
    }

    /// Read all tasks from file, and return a hash map recording ID to Task Struct
    /// 
    /// # Example
    /// ```
    /// let tasks = Task::from_file("test/test_dag.yaml")
    /// ```
    fn from_file(filename: &str) -> HashMap<String, Self> {
        let res = Task::read_tasks(filename);
        if let Err(e) = res {
            println!("[Error] {}", e);
            exit(0);
        } else {
            res.unwrap()
        }
    }


}

impl Task {
    /// Parse Task from Yaml
    /// 
    /// # Example
    /// ```
    /// let task = Task::from_yaml(id, yaml);
    /// ```
    /// Here `id` and `yaml` comes from:
    /// ```
    /// let yaml_tasks = YamlLoader::load_from_str(&yaml_cont)?;
    /// let yaml_tasks = yaml_tasks[0]["dagrs"]
    /// .as_hash()
    /// .ok_or(DagError::format_error("", FormatErrorMark::StartWordError))?;
    /// 
    /// for(id, yaml) in yaml_tasks{
    ///     ...
    /// }
    /// ```
    fn from_yaml(id: &str, info: &Yaml) -> Result<Task, DagError> {
        // Get name first

        let name = info["name"]
            .as_str()
            .ok_or(DagError::format_error(id, FormatErrorMark::NoName))?
            .to_owned();

        // relys can be empty
        let mut relys = Vec::new();
        if let Some(rely_tasks) = info["rely"].as_vec() {
            for rely_task_id in rely_tasks {
                let rely_task_id = rely_task_id
                    .as_str()
                    .ok_or(DagError::format_error(id, FormatErrorMark::RelyIDIllegal))?
                    .to_owned();
                relys.push(rely_task_id)
            }
        }

        Ok(Task {
            ID: id.into(),
            name,
            relys,
        })
    }

    /// Read all tasks from yaml file.
    fn read_tasks(filename: &str) -> Result<HashMap<String, Self>, DagError> {
        let mut yaml_cont = String::new();

        let mut yaml_file = File::open(filename)?;
        yaml_file.read_to_string(&mut yaml_cont)?;

        // Parse Yaml
        let yaml_tasks = YamlLoader::load_from_str(&yaml_cont)?;
        let yaml_tasks = yaml_tasks[0]["dagrs"]
            .as_hash()
            .ok_or(DagError::format_error("", FormatErrorMark::StartWordError))?;

        let mut tasks = HashMap::new();
        // Read tasks
        for (v, w) in yaml_tasks {
            let id = v.as_str().unwrap(); // .ok_or(DagError::form("task id error"))?;
            let task = Task::from_yaml(id, w)?;

            tasks.insert(id.to_owned(), task);
        }

        Ok(tasks)
    }
}