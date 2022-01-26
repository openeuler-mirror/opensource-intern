/*
 * @Author: Yinwhe
 * @Date: 2022-01-19 14:14:28
 * @LastEditors: Yinwhe
 * @LastEditTime: 2022-01-26 23:39:46
 * @Description: dagrs
 * @Copyright: Copyright (c) 2022
 */

extern crate bimap;
extern crate yaml_rust;

mod error_handler;
mod graph;
mod task;
mod dag_engine;

use dag_engine::DagEngine;

fn main() {
    let mut dagrs = DagEngine::new();
    if let Err(e) = dagrs.run("test/test1.yaml") {
        println!("[Error] {}", e);
    }
}