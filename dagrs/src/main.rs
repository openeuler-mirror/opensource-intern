extern crate bimap;
extern crate clap;
extern crate deno_core;
extern crate lazy_static;
extern crate yaml_rust;
#[macro_use]
extern crate log;
extern crate simplelog;

mod dag_engine;
mod error_handler;
mod graph;
mod task;

use std::fs::File;

use clap::Parser;
use dag_engine::DagEngine;
use simplelog::*;

use crate::task::Retval;

#[derive(Parser)]
#[clap(version)]
/// Command Line input
struct Args {
    /// YAML file path
    #[clap(short, long)]
    filepath: String,
    /// Log file path
    #[clap(short, long)]
    logpath: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut dagrs: DagEngine = DagEngine::new();
    let log_path = if let Some(s) = args.logpath {
        s
    } else {
        "./example.log".to_owned()
    };

    init_logger(&log_path);

    if let Err(e) = dagrs.run_from_yaml(&args.filepath) {
        error!("[Error] {}", e);
    }
}

/// Get a new logger, which will store logs into given path.
fn init_logger(path: &str) {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(path).unwrap(),
        ),
    ])
    .unwrap();
}

#[test]
fn test_runscript() {
    let res = DagEngine::new().run_from_yaml("test/test_dag1.yaml").unwrap();
    assert_eq!(res, true)
}

#[test]
fn test_dag() {
    let res = DagEngine::new()
        .run_from_yaml("test/test_dag2.yaml")
        .unwrap();
    assert_eq!(res, true)
}


#[test]
fn test_loop() {
    let res = DagEngine::new()
        .run_from_yaml("test/test_loop1.yaml")
        .unwrap();
    assert_eq!(res, false)
}

#[test]
fn test_complex_loop() {
    let res = DagEngine::new()
        .run_from_yaml("test/test_loop2.yaml")
        .unwrap();
    assert_eq!(res, false)
}

#[test]
fn test_format_error1() {
    use crate::error_handler::{DagError, FormatError};
    let res = DagEngine::new().run_from_yaml("test/test_error1.yaml");
    assert_eq!(
        res,
        Err(DagError::format_error(FormatError::NoName("a".into())))
    );
}

#[test]
fn test_format_error2() {
    use error_handler::{DagError, FormatError};
    let res = DagEngine::new().run_from_yaml("test/test_error2.yaml");
    assert_eq!(
        res,
        Err(DagError::format_error(FormatError::StartWordError))
    );
}

#[test]
fn test_rely_error() {
    use error_handler::{DagError, InnerError};
    let res = DagEngine::new().run_from_yaml("test/test_error3.yaml");
    assert_eq!(
        res,
        Err(DagError::inner_error(InnerError::RelyTaskIllegal(
            "任务1".into()
        )))
    );
}

#[test]
fn test_no_runscript() {
    use error_handler::{DagError, FormatError};
    let res = DagEngine::new().run_from_yaml("test/test_error4.yaml");
    assert_eq!(
        res,
        Err(DagError::format_error(FormatError::RunScriptError(
            "a".into()
        )))
    );
}

#[test]
fn test_prom1() {
    use crate::task::{TaskTrait, TaskWrapper};
    struct T1 {}
    impl TaskTrait for T1 {
        fn run(&self) -> Option<Retval> {
            println!("T1!");
            None
        }
    }

    struct T2 {}
    impl TaskTrait for T2 {
        fn run(&self) -> Option<Retval> {
            println!("T2!");
            None
        }
    }

    let mut t1 = TaskWrapper::new(T1 {}, "Task 1");
    let mut t2 = TaskWrapper::new(T2 {}, "Task 2");
    let t3 = TaskWrapper::new(T1 {}, "Task 3");

    t2.add_relys(&[&t1, &t3]);
    t1.add_relys(&[&t3]);

    let mut dag = DagEngine::new();
    dag.add_task(t1);
    dag.add_task(t2);
    dag.add_task(t3);

    dag.run().unwrap();
}

#[test]
fn test_prom2() {
    use crate::task::{RunScript, RunType, TaskTrait, TaskWrapper};
    struct T {
        run_script: RunScript,
    }

    impl TaskTrait for T {
        fn run(&self) -> Option<Retval> {
            Some(self.run_script.exec())
        }
    }

    let mut t1 = TaskWrapper::new(
        T {
            run_script: RunScript::new("echo T1", RunType::SH),
        },
        "Task 1",
    );
    let mut t2 = TaskWrapper::new(
        T {
            run_script: RunScript::new("echo T2", RunType::SH),
        },
        "Task 2",
    );
    let t3 = TaskWrapper::new(
        T {
            run_script: RunScript::new(r#"Deno.core.print("T3\n")"#, RunType::DENO),
        },
        "Task 3",
    );

    t2.add_relys(&[&t1, &t3]);
    t1.add_relys(&[&t3]);

    let mut dag = DagEngine::new();
    dag.add_task(t1);
    dag.add_task(t2);
    dag.add_task(t3);

    dag.run().unwrap();
}
