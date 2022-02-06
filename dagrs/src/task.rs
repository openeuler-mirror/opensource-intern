/*
 * @Author: Yinwhe
 * @Date: 2022-01-25 17:49:19
 * @LastEditors: Yinwhe
 * @LastEditTime: 2022-02-06 13:40:58
 * @Description: Task implementation
 */

use crate::error_handler::DagError;
use yaml_rust::Yaml;

#[derive(Debug)]
pub struct Task {
    name: String,
    relys: Vec<String>,
}

impl Task {
    /// Parse Task from Yaml
    pub fn from_yaml(info: &Yaml) -> Result<Task, DagError> {
        // Get name first
        let name = info["name"]
            .as_str()
            .ok_or(DagError::error("Task name not found"))?
            .to_owned();

        // relys can be empty
        let mut relys = Vec::new();
        if let Some(rely_tasks) = info["rely"].as_vec() {
            for rely_task_id in rely_tasks {
                let id = rely_task_id
                    .as_str()
                    .ok_or(DagError::error("Rely tasks id error"))?
                    .to_owned();
                relys.push(id)
            }
        }

        Ok(Task { name, relys })
    }

    pub fn get_rely_list(&self) -> &Vec<String> {
        &self.relys
    }
}
