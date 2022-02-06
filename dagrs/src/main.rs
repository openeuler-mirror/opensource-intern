/*
 * @Author: Yinwhe
 * @Date: 2022-01-19 14:14:28
 * @LastEditors: Yinwhe
 * @LastEditTime: 2022-02-06 13:48:41
 * @Description: dagrs
 */

extern crate bimap;
extern crate yaml_rust;

mod dag_engine;
mod error_handler;
mod graph;
mod task;

use dag_engine::DagEngine;

fn main() {
    let mut dagrs = DagEngine::new();
    if let Some(filename) = std::env::args().nth(1) {
        if let Err(e) = dagrs.run(&filename) {
            println!("[Error] {}", e);
        }
    } else {
        println!("Usage: dargs <filename>")
    }
}

#[test]
fn test1() {
    let res = DagEngine::new().run("test/test1.yaml").unwrap();
    assert_eq!(res, false);
}

#[test]
fn test2() {
    let res = DagEngine::new().run("test/test2.yaml").unwrap();
    assert_eq!(res, true)
}

#[test]
fn test3() {
    let res = DagEngine::new().run("test/test3.yaml").unwrap();
    assert_eq!(res, false)
}

#[test]
fn test4() {
    let res = DagEngine::new().run("test/test4.yaml").unwrap();
    assert_eq!(res, false)
}