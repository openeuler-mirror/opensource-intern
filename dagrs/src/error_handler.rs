/*
 * @Author: Yinwhe
 * @Date: 2022-01-21 11:14:18
 * @LastEditors: Yinwhe
 * @LastEditTime: 2022-01-26 23:17:51
 * @Description: Simple error handler
 * @Copyright: Copyright (c) 2021
 */

use std::fmt::{Debug, Display};
use yaml_rust::ScanError;

#[derive(Debug)]
pub struct DagError(String);

impl Display for DagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<std::io::Error> for DagError {
    fn from(e: std::io::Error) -> Self {
        DagError(e.to_string())
    }
}

impl From<ScanError> for DagError {
    fn from(e: ScanError) -> Self {
        DagError(e.to_string())
    }
}

impl DagError{
    pub fn error(message: &str) -> DagError {
        DagError(message.to_string())
    }
}