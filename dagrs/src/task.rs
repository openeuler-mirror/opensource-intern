//! Task Implementations, used to store task infos

use crate::error_handler::{DagError, FormatError, InnerError};
use deno_core::{JsRuntime, RuntimeOptions};
use lazy_static::lazy_static;
use std::process::Command;
use std::sync::Mutex;
use std::{collections::HashMap, fs::File, io::Read};
use yaml_rust::{Yaml, YamlLoader};


/// Can be used to run a script cmd or file.
#[derive(Debug)]
pub struct RunScript {
    script: String,
    executor: RunType,
}

/// Run script type, now a script can be run in `sh` or embeded `deno`
#[derive(Debug)]
pub enum RunType {
    SH,
    DENO,
}

/// Return value for task, not supported now.
pub struct Retval {
    pub success: bool,
    pub value: String,
}

impl RunScript {
    /// Generate a new run script
    /// 
    /// # Example
    /// ```
    /// let r = RunScript::new("echo Hello!", RunType::SH);
    /// r.exec();
    /// ```
    pub fn new(script: &str, executor: RunType) -> Self {
        Self {
            script: script.to_owned(),
            executor,
        }
    }

    /// Execute the script.
    /// 
    /// # Example
    /// ```
    /// let r = RunScript::new("echo Hello!", RunType::SH);
    /// r.exec();
    /// ```
    pub fn exec(&self) -> Retval {
        match self.executor {
            RunType::SH => self.run_sh(),
            RunType::DENO => self.run_deno(),
        }
    }

    fn run_sh(&self) -> Retval {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&self.script)
            .output()
            .unwrap();
        // Reprint result
        print!("{}", String::from_utf8(output.stdout).unwrap());

        Retval {
            success: output.status.success(),
            value: "".into(),
        }
    }

    fn run_deno(&self) -> Retval {
        let output = JsRuntime::new(RuntimeOptions {
            ..Default::default()
        })
        .execute_script("", &self.script);

        match output {
            Ok(val) => Retval {
                success: true,
                value: format!("{:?}", val),
            },
            Err(e) => Retval {
                success: false,
                value: format!("{}", e),
            },
        }
    }
}

/// Task Trait.
///
/// Any struct implements this trait can be added into dagrs.
pub trait TaskTrait {
    fn run(&self) -> Option<Retval>;
}

/// Wrapper for task that impl [`TaskTrait`].
pub struct TaskWrapper {
    id: usize,
    name: String,
    rely_list: Vec<usize>,
    inner: Box<dyn TaskTrait>,
}

impl TaskWrapper {
    /// Allocate a new TaskWrapper.
    ///
    /// # Example
    /// ```
    /// let t = TaskWrapper::new(Task{}, "Demo Task")
    /// ```
    ///
    /// `Task` is a struct that impl [`TaskTrait`].
    ///
    /// **Note:** This method will take the ownership of struct that impl [`TaskTrait`].
    pub fn new(task: impl TaskTrait + 'static, name: &str) -> Self {
        TaskWrapper {
            id: ID_ALLOCATOR.lock().unwrap().alloc(),
            name: name.to_owned(),
            rely_list: Vec::new(),
            inner: Box::new(task),
        }
    }

    #[allow(unused)]
    /// Tasks that shall be executed after this one.
    ///
    /// # Example
    /// ```
    /// let mut t1 = TaskWrapper::new(T1{}, "Task 1");
    /// let mut t2 = TaskWrapper::new(T2{}, "Task 2");
    /// t2.add_relys(&[&t1]);
    /// ```
    /// In above code, `t2` will be executed before `t1`.
    pub fn add_relys(&mut self, relys: &[&TaskWrapper]) {
        self.rely_list.extend(relys.iter().map(|t| t.get_id()))
    }

    /// Tasks that shall be executed after this one.
    ///
    /// # Example
    /// ```
    /// let mut t1 = TaskWrapper::new(T1{}, "Task 1");
    /// let mut t2 = TaskWrapper::new(T2{}, "Task 2");
    /// t2.add_relys_by_ids(&[t1.get_id()]);
    /// ```
    /// Similar to `add_relys`, but this method tasks `id` rather than a task.
    ///
    /// In above code, `t2` will be executed before `t1`.
    pub fn add_relys_by_ids(&mut self, relys: &[usize]) {
        self.rely_list.extend(relys.iter())
    }

    pub fn get_rely_list(&self) -> Vec<usize> {
        self.rely_list.clone()
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn run(&self) -> Option<Retval> {
        self.inner.run()
    }
}

/// IDAllocator for TaskWrapper
struct IDAllocator {
    id: usize,
}

impl IDAllocator {
    pub fn alloc(&mut self) -> usize {
        self.id += 1;

        // Return values
        self.id - 1
    }
}

lazy_static! {
    /// Instance of IDAllocator
    static ref ID_ALLOCATOR: Mutex<IDAllocator> = Mutex::new(IDAllocator { id: 0 });
}

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
    fn run(&self) -> Option<Retval> {
        Some(self.run.exec())
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
            .ok_or(DagError::format_error(FormatError::NoName(id.to_owned())))?
            .to_owned();

        // Get run script
        let run = &info["run"];

        let executor = match run["type"].as_str().ok_or(DagError::format_error(
            FormatError::RunScriptError(id.into()),
        ))? {
            "sh" => RunType::SH,
            "deno" => RunType::DENO,
            _ => {
                return Err(DagError::format_error(FormatError::RunScriptError(
                    id.into(),
                )))
            }
        };

        let run_script =
            run["script"]
                .as_str()
                .ok_or(DagError::format_error(FormatError::RunScriptError(
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
            .ok_or(DagError::format_error(FormatError::StartWordError))?;

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
                    return Err(DagError::inner_error(InnerError::RelyTaskIllegal(
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
