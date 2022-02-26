extern crate clap;
extern crate lazy_static;
extern crate yaml_rust;
extern crate bimap;

mod dag_engine;
mod error_handler;
mod graph;
mod task;

use clap::Parser;
use dag_engine::DagEngine;

#[derive(Parser)]
#[clap(version)]
/// Command Line input
struct Args {
    /// YAML file path
    #[clap(short, long)]
    filepath: String,
}


fn main() {
    let args = Args::parse();
    let mut dagrs: DagEngine = DagEngine::new();

    if let Err(e) = dagrs.run_from_yaml(&args.filepath) {
        println!("[Error] {}", e);
    }
}

#[test]
fn test_dag() {
    let res = DagEngine::new().run_from_yaml("test/test_dag.yaml").unwrap();
    assert_eq!(res, true)
}


#[test]
fn test_loop() {
    let res = DagEngine::new().run_from_yaml("test/test_loop1.yaml").unwrap();
    assert_eq!(res, false)
}

#[test]
fn test_complex_loop() {
    let res = DagEngine::new().run_from_yaml("test/test_loop2.yaml").unwrap();
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
        Err(DagError::inner_error(InnerError::RelyTaskIllegal("任务1".into())))
    );
}

#[test]
fn test_no_runscript() {
    use error_handler::{DagError, FormatError};
    let res = DagEngine::new().run_from_yaml("test/test_error4.yaml");
    assert_eq!(
        res,
        Err(DagError::format_error(FormatError::NoRunScript("a".into())))
    );
}

#[test]
fn test_prom() {
    use crate::task::{TaskTrait, TaskWrapper};

    struct T1 {}
    impl TaskTrait for T1 {
        fn run(&self) {
            println!("T1!");
        }
    }

    struct T2 {}
    impl TaskTrait for T2 {
        fn run(&self) {
            println!("T2!");
        }
    }

    let mut t1 = TaskWrapper::new(T1{}, "Task 1");
    let mut t2 = TaskWrapper::new(T2{}, "Task 2");
    let mut t3 = TaskWrapper::new(T1{}, "Task 3");

    t2.add_relys(&[&t1, &t3]);
    t1.add_relys(&[&t3]);


    let mut dag = DagEngine::new();
    dag.add_task(t1);
    dag.add_task(t2);
    dag.add_task(t3);

    dag.run();
}