use crate::error_handler::DagError;

use super::{ExecState, Retval, Inputval};
use deno_core::{JsRuntime, RuntimeOptions};
use lazy_static::lazy_static;
use std::process::Command;
use std::sync::Mutex;
use std::thread;

/// Task Trait.
///
/// Any struct implements this trait can be added into dagrs.
pub trait TaskTrait {
    fn run(&self, input: Inputval) -> Retval;
}

/// Wrapper for task that impl [`TaskTrait`].
/// 
/// Since task will be executed in seperated threads, `send` is needed.
pub struct TaskWrapper {
    id: usize,
    name: String,
    rely_list: Vec<usize>,
    inner: Mutex<Box<dyn TaskTrait + Send>>,
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
    pub fn new(task: impl TaskTrait + 'static + Send, name: &str) -> Self {
        TaskWrapper {
            id: ID_ALLOCATOR.lock().unwrap().alloc(),
            name: name.to_owned(),
            rely_list: Vec::new(),
            inner: Mutex::new(Box::new(task)),
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
    pub fn rely_on(&mut self, relys: &[&TaskWrapper]) {
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

    pub fn get_inner(&self) -> &Mutex<Box<dyn TaskTrait + Send>> {
        &self.inner
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
    pub fn exec(&self) -> Result<String, DagError> {
        match self.executor {
            RunType::SH => self.run_sh(),
            RunType::DENO => self.run_deno(),
        }
    }

    fn run_sh(&self) -> Result<String, DagError> {
        let res = Command::new("sh")
            .arg("-c")
            .arg(&self.script)
            .output()
            .map(|output| format!("{}", String::from_utf8(output.stdout).unwrap()));
        
        res.map_err(|err| err.into())
    }

    fn run_deno(&self) -> Result<String, DagError> {
        let script = self.script.clone();
        let handle = thread::spawn(move || {
            let output = JsRuntime::new(RuntimeOptions {
                ..Default::default()
            })
            .execute_script("", &script);
        });

        // TODO
        unimplemented!()
    }
}
