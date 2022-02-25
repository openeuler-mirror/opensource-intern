
#![allow(unused_imports)]
#![allow(dead_code)]
use clap::Parser;
use similar::{ChangeTag, TextDiff};
use markdown_gen::markdown::Markdown;
use console::{style, Style};
pub use tokio::try_join;
use serde::Deserialize;
use toml;

pub use std::collections::HashMap;
pub use std::fmt;
pub use std::fs::File;
pub use std::io::{Write, Read};

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub path: Option<String>,
    pub addresses: Option<Vec<Address>>,
}
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Address{
    pub name: String,
    pub x: String, 
    pub y: String
}

#[derive(Parser)]
pub struct Cli {
    path_config: String,
}

impl Cli {
    pub async fn get_address_list_from_cli() -> Result<Vec<Address>, Box<dyn std::error::Error + Send + Sync>> {
        let path = Cli::parse().path_config;
        get_address_list(path)
    }
}
fn get_address_list(path: String) -> Result<Vec<Address>, Box<dyn std::error::Error + Send + Sync>> {
    let mut input = String::new();
    File::open(&path)
        .and_then(|mut f| f.read_to_string(&mut input))?;
    let config:Config = toml::from_str(&input[..]).unwrap();
    Ok(config.addresses.unwrap())
}





pub async fn get_diff(name: String, specs: Vec<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let diff = TextDiff::from_lines(&specs[0], &specs[1]);

    let path = name + "-diffreport.md";
    let file = File::create(&path[..]).expect("create failed");
    let mut mdfile = Markdown::new(file);
    
    for group in diff.grouped_ops(3).iter(){
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };
                print!(
                    "{}{} |{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                );

                let mut line = format!("{:<4}|{}",  Line(change.old_index()), sign);
                // mdfile.write(&format!("{}{} | {}", change.old_index().unwrap(), change.new_index().unwrap(), sign)[..])?;
                for (emphasized, value) in change.iter_strings_lossy() {
                    let st = value.clone();
                    line.push_str(&format!("{}",st));
                    if emphasized {
                        print!("{}", s.apply_to(value).underlined().on_black());
                    } else {
                        print!("{}", s.apply_to(value));
                    }
                }
                mdfile.write(&line[..])?;
                
                if change.missing_newline() {
                    println!();
                }
            }
        }
    }


    println!("data written successfully in {}", path);
    println!("diff ratio: {}", diff.ratio());

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn toml_should_word() {
        
    }

    #[test]
    fn default_strategy_should_work() {

    }
}

