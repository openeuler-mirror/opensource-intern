extern crate bimap;
extern crate clap;
extern crate deno_core;
extern crate lazy_static;
extern crate yaml_rust;
#[macro_use]
extern crate log;
extern crate anymap;
extern crate crossbeam;
extern crate simplelog;

mod dag_engine;
mod error_handler;
mod graph;
mod task;

pub use dag_engine::DagEngine;
pub use task::TaskTrait;

use simplelog::*;
use std::{
    env,
    fs::{create_dir, File},
};

pub fn init_log(logpath: Option<&str>) {
    let logpath = if let Some(s) = logpath {
        s.to_owned()
    } else {
        if let Ok(home) = env::var("HOME") {
            create_dir(format!("{}/.dagrs", home)).unwrap_or(());
            format!("{}/.dagrs/dagrs.log", home)
        } else {
            "./dagrs.log".to_owned()
        }
    };

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
            File::create(logpath).unwrap(),
        ),
    ])
    .unwrap();
}


#[test]
fn test_prom1() {
    use crate::task::{Inputval, Retval, TaskTrait, TaskWrapper};
    struct T1 {}
    impl TaskTrait for T1 {
        fn run(&self, _input: Inputval) -> Retval {
            println!("T1!");
            Retval::new(1i32)
        }
    }

    struct T2 {}
    impl TaskTrait for T2 {
        fn run(&self, input: Inputval) -> Retval {
            let val_from_t1 = input.get::<i32>(0);
            println!("T2, receive: {:?}", val_from_t1);
            Retval::empty()
        }
    }

    let t1 = TaskWrapper::new(T1 {}, "Task 1");
    let mut t2 = TaskWrapper::new(T2 {}, "Task 2");

    t2.rely_on(&[&t1]);

    let mut dag = DagEngine::new();
    dag.add_task(t1);
    dag.add_task(t2);

    dag.run().unwrap();
}