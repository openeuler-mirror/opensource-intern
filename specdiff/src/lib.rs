
#![allow(unused_imports)]
#![allow(dead_code)]
pub use clap::Parser;
use reqwest::Response;
use similar::{ChangeTag, TextDiff};
use markdown_gen::markdown::Markdown;
use console::{style, Style};
use tokio::try_join;
use serde::Deserialize;
use chrono::prelude::*;
use toml;

pub use std::{
    collections::HashMap,
    fmt,
    fs::{self, File},
    io::{self, Write, Read},
};

mod error;
pub use error::SpecError;

/// 方便 console::style 调用
/// 使用方法来源于 https://github.com/mitsuhiko/similar/blob/main/examples/terminal-inline.rs
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
    pub out_name: Option<String>,
    pub x: String, 
    pub y: String
}

/// 使用 rust 编写的简易 spec 文件比较程序，支持 toml 格式的配置文件输入。
/// 允许在控制台输出以及生成 diff 报告，输出报告格式为 markdown。
#[derive(Parser)]
#[clap(version = "0.2.0", author = "Ke Lei <Ke_lei@foxmail.com>")]
pub struct Cli {
    /// 一个 .toml 格式的配置文件的路径, 文件中需要有至少一个 [[addresses]] 配置项, 其中包括软件名字 name, 输出报告名称 out_name (可不填), 软件不同的spec文件地址 x 和 y (都是 String 类型)
    pub config_path: String,
    /// specdiff 输出的对比报告存放的路径, 默认为当前目录.
    #[clap(short, long)]
    pub report_out_path: Option<String>,
    /// 指定下载 spec 文件的保存目录, 默认为 /tmp/specdiff/download/
    #[clap(short, long)]
    pub spec_save_path: Option<String>,
    /// 是否在控制台输出 diff 内容, 默认为 true
    #[clap(short, long)]
    pub terminal_out: Option<bool>,
}

impl Cli {
    pub async fn get_address_list_from_cli() -> Result<Vec<Address>, SpecError> {
        let path = Cli::parse().config_path;
        get_address_list(&path)
    }

    fn get_save_path() -> String {
        match Cli::parse().spec_save_path {
            Some(s) => s + "/",
            None => "./".to_string(),
        }
    }

    
}

fn get_address_list(path: &str) -> Result<Vec<Address>, SpecError> {
    let mut input = String::new();
    File::open(path)
        .and_then(|mut f| f.read_to_string(&mut input))?;
    let config:Config = toml::from_str(&input[..]).unwrap();
    Ok(config.addresses.unwrap())
}

#[allow(unused)]
/// 获得 diff 内容，并通过参数决定是否输出相应内容到控制台
pub async fn get_diff_from_address(
    address: Address, 
    spec_save_path: &str,
    out_terminal: &bool, 
    report_out_path: &str,
    diff_ratio_list: &mut Vec<f32>,
    writer: &mut dyn Write,
) -> Result<(), SpecError> {
    let dt = Local::now();
    let spec_list = match download_specs(&address, &spec_save_path).await {
        Err(e) => {
            panic!("Internal error: {:?}", e);
        }
        Ok(spec_list) => spec_list
    };


    let mut report_name = report_out_path.to_string();
    report_name += "/";
    match address.out_name {
        Some(name) => report_name = report_name + &name[..] + ".md",
        None => {
            let mut name = address.name.clone();
            name += "-specdiff-";
            name += &dt.format("%Y-%m-%d %H:%M:%S").to_string()[..];
            name += ".md";
            report_name += &name[..];
        }
    }
    let file = File::create(&report_name).expect("create failed");
    let diff = TextDiff::from_lines(&spec_list[0], &spec_list[1]);

    // 默认写入 markdown 文件
    diff.unified_diff().to_writer(&file).expect("write markdown false");
    Markdown::new(file);
    if let true = *out_terminal {
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
                    for (emphasized, value) in change.iter_strings_lossy() {
                        if emphasized {
                            print!("{}", s.apply_to(value).underlined().on_black());
                        } else {
                            print!("{}", s.apply_to(value));
                        }
                    }
                    if change.missing_newline() {
                        println!();
                    }
                }
            }
        }
    }
    let diff_ratio = diff.ratio();
    diff_ratio_list.push(diff_ratio);
    println!("report written successfully in {}", report_name);
    println!("diff-ratio for {} is: {:.2}%", address.name, diff_ratio*100.0);

    Ok(())
}



/// 下载一组 spec 文件，保存文件内容，并通过 Vec<String> 返回 spec 内容
/// 默认下载保存目录在 /tmp/specdiff/download/
/// [TODO] 下载进度条
pub async fn download_specs(address: &Address, spec_save_path: &str) -> Result<Vec<String>, SpecError>{
    let f1 = reqwest::get(&address.x);
    let f2 = reqwest::get(&address.y);
    let (res1, res2) = try_join!(f1, f2)?;
    let report_name = spec_save_path.to_string() + "/" + &address.name;
    let mut file = File::create(&report_name)?;

    let (s1, s2) = (res1.text().await?, res2.text().await?);
    file.write_all(s1.as_bytes())?;
    file.write_all(b"\n")?;
    file.write_all(s2.as_bytes())?;
    Ok(vec![s1, s2])

}


#[cfg(test)]
mod tests {

    use super::*;



}

