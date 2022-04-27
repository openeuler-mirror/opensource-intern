use super::{Retval, RunScript, RunType, TaskTrait, TaskWrapper, Inputval};
use crate::error_handler::{DagError, YamlFormatError, RunningError};
use std::{collections::HashMap, fs::File, io::Read};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
/// Task Struct for YAML file.
struct YamlTaskInner {
    /// Running Script
    run: RunScript,
}

/// Task struct for YAML file.
#[derive(Debug)]
pub struct YamlTask {
    /// Task's id in yaml file.
    ///
    /// Be careful that `yaml_id` is different from [`TaskWrapper`]'s id.
    yaml_id: String,
    /// Task's name.
    name: String,
    /// Record tasks' `yaml_id` that shall be executed after this task.
    relys: Vec<String>,
    /// A field shall be wrapper into [`TaskWrapper`] later.
    inner: YamlTaskInner,
}

impl TaskTrait for YamlTaskInner {
    fn run(&self, input: Inputval) -> Retval {
        // TODO
        Retval::empty()
    }
}

impl YamlTask {
    /// Parse a task from yaml.
    ///
    /// # Example
    /// ```
    /// let task = Task::parse_one(id, yaml);
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
    fn parse_one(id: &str, info: &Yaml) -> Result<YamlTask, DagError> {
        // Get name first
        let name = info["name"]
            .as_str()
            .ok_or(DagError::format_error(YamlFormatError::NoName(id.to_owned())))?
            .to_owned();

        // Get run script
        let run = &info["run"];

        let executor = match run["type"].as_str().ok_or(DagError::format_error(
            YamlFormatError::RunScriptError(id.into()),
        ))? {
            "sh" => RunType::SH,
            "deno" => RunType::DENO,
            _ => {
                return Err(DagError::format_error(YamlFormatError::RunScriptError(
                    id.into(),
                )))
            }
        };

        let run_script =
            run["script"]
                .as_str()
                .ok_or(DagError::format_error(YamlFormatError::RunScriptError(
                    id.into(),
                )))?;

        // relys can be empty
        let mut relys = Vec::new();
        if let Some(rely_tasks) = info["rely"].as_vec() {
            rely_tasks
                .iter()
                .map(|rely_task_id| relys.push(rely_task_id.as_str().unwrap().to_owned()))
                .count();
        }

        let inner = YamlTaskInner {
            run: RunScript::new(run_script, executor),
        };

        Ok(YamlTask {
            yaml_id: id.to_string(),
            name,
            relys,
            inner,
        })
    }

    /// Parse all tasks from yaml file.
    ///
    /// # Example
    /// ```
    /// let tasks = YamlTask::parse_tasks("test/test_dag.yaml")?;
    /// ```
    fn parse_tasks(filename: &str) -> Result<Vec<Self>, DagError> {
        let mut yaml_cont = String::new();

        let mut yaml_file = File::open(filename)?;
        yaml_file.read_to_string(&mut yaml_cont)?;

        // Parse Yaml
        let yaml_tasks = YamlLoader::load_from_str(&yaml_cont)?;
        let yaml_tasks = yaml_tasks[0]["dagrs"]
            .as_hash()
            .ok_or(DagError::format_error(YamlFormatError::StartWordError))?;

        let mut tasks = Vec::new();
        // Read tasks
        for (v, w) in yaml_tasks {
            let id = v.as_str().unwrap();
            let task = YamlTask::parse_one(id, w)?;

            tasks.push(task);
        }

        Ok(tasks)
    }

    /// Parse all tasks from yaml file into format recognized by dagrs.
    ///
    /// # Example
    /// ```
    /// let tasks = YamlTask::from_yaml(filename)?;
    /// ```
    ///
    /// Used in [`crate::DagEngine`].
    pub fn from_yaml(filename: &str) -> Result<Vec<TaskWrapper>, DagError> {
        let tasks = YamlTask::parse_tasks(filename)?;
        let mut res = Vec::new();
        let mut temp_hash_yaml2id = HashMap::new();
        let mut temp_hash_id2rely = HashMap::new();

        // Wrap tasks
        tasks
            .into_iter()
            .map(|t| {
                let task = TaskWrapper::new(t.inner, &t.name);
                temp_hash_id2rely.insert(task.get_id(), t.relys);
                temp_hash_yaml2id.insert(t.yaml_id, task.get_id());
                res.push(task);
            })
            .count();

        // Add Dependency
        for task in &mut res {
            let mut relys = Vec::new();
            for rely in &temp_hash_id2rely[&task.get_id()] {
                // Rely task existence check
                if !temp_hash_yaml2id.contains_key(rely) {
                    return Err(DagError::running_error(RunningError::RelyTaskIllegal(
                        task.get_name(),
                    )));
                }
                relys.push(temp_hash_yaml2id[rely])
            }
            task.add_relys_by_ids(&relys)
        }

        Ok(res)
    }
}
