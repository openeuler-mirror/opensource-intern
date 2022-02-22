
#![allow(unused_imports)]
#![allow(dead_code)]
use clap::Parser;
use similar::{ChangeTag, TextDiff};
use markdown_gen::markdown::Markdown;
use console::{style, Style};

use std::fmt;
use std::fs::File;
use std::io::{Write};

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

#[derive(Parser)]
struct Cli {

    path1: String,

    path2: String,
}

impl Cli {
    async fn get_spec_from_html() -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let args = Cli::parse();
        let p1 = args.path1;
        let p2 = args.path2;

        
        let res1 = reqwest::get(p1).await?;
        let res2 = reqwest::get(p2).await?;
        let body = vec![res1.text().await?, res2.text().await?];

        Ok(body)
    }

    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let specs = Cli::get_spec_from_html().await?;
    let diff = TextDiff::from_lines(&specs[0], &specs[1]);

    let file = File::create("diffreport.md").expect("create failed");

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


    println!("data written successfully");
    println!("diff ratio: {}", diff.ratio());

    Ok(())

}
