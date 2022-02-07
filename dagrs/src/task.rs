//! Task Implementations, used to store task infos

use crate::error_handler::{DagError, FormatErrorMark};
use yaml_rust::Yaml;

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
    pub fn from_yaml(id: &str, info: &Yaml) -> Result<Task, DagError> {
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
    pub fn get_rely_list(&self) -> &Vec<String> {
        &self.relys
    }
}
