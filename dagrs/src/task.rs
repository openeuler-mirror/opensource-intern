/*
 * @Author: Yinwhe
 * @Date: 2022-01-25 17:49:19
 * @LastEditors: Yinwhe
 * @LastEditTime: 2022-01-26 23:18:04
 * @Description: Task implementation
 * @Copyright: Copyright (c) 2022
 */

use crate::error_handler::DagError;
use yaml_rust::Yaml;

#[derive(Debug)]
pub struct Task {
    name: String,
    relys: Vec<String>,
}

impl Task {
    pub fn from_yaml(info: &Yaml) -> Result<Task, DagError> {
        let name = info["name"]
            .as_str()
            .ok_or(DagError::error("Task name not found"))?
            .to_owned();

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
