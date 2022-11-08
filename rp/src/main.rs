use std::{
    env,
    fs::File,
    io::{BufRead},
};
use std::io::Write;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(author, version = "v0.1.0", about, long_about = None)]
struct Cli {
    /// Interpret the patch file as a ordinary context diff.
    #[arg(short = 'c')]
    context: bool,

    /// Interpret the patch file as a unified context diff.
    #[arg(short = 'u')]
    unified: bool,

    /// Interpret the patch file as an ed script.
    #[arg(short = 'e')]
    ed: bool,

    /// Interpret the patch file as a normal diff.
    #[arg(short = 'n')]
    normal: bool,

    /// Output patched files to FILE.
    #[arg(short = 'o', value_name = "FILE")]
    output: Option<String>,

    /// Read patch from PATCH FILE instead of stdin
    #[arg(short = 'i', value_name = "PATCH FILE")]
    input: Option<String>,

    /// The file to apply patch
    #[arg(last = true)]
    to_patch: String,
}

#[derive(Debug)]
struct Patch {
    file_old: String,
    file_new: String,
    huck: Vec<UnifiedHuck>,
}

#[derive(Debug, Clone)]
struct UnifiedHuck {
    old_count: u32,
    old_lines: u32,
    new_count: u32,
    new_lines: u32,
    diff: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    let current_dir = env::current_dir().unwrap();

    let mut file_to_patch = match args.to_patch.as_str().is_empty() {
        false => {
            let mut to_patch_array: Vec<String> = Vec::new();
            let mut path = current_dir.clone();
            path.push(args.to_patch);
            let patch_file = File::open(path).unwrap();

            let reader1 = std::io::BufReader::new(patch_file);

            for line in reader1.lines() {
                to_patch_array.push(line.unwrap());
            }
            to_patch_array
        }
        _ => { Vec::new() }
    };

    let patch_str_array = match args.input {
        Some(val) => {
            let mut path = current_dir.clone();
            path.push(val);
            let patch_file = File::open(path).unwrap();

            let mut patch_str_array: Vec<String> = Vec::new();
            let reader1 = std::io::BufReader::new(patch_file);
            for line in reader1.lines() {
                patch_str_array.push(line.unwrap());
            }
            patch_str_array
        }
        None => {
            todo!("read patch from stdin")
        }
    };

    let res_file = if args.context {
        todo!("parse and apply context patch")
    } else if args.unified {
        let pat: Patch = parse_unified(patch_str_array);
        apply_unified_patch(pat, file_to_patch)
    } else if args.normal {
        parse_normal(patch_str_array, &mut file_to_patch);
        file_to_patch
    } else if args.ed {
        parse_ed(patch_str_array, &mut file_to_patch);
        file_to_patch
    } else {
        panic!("Parse Error")
    };

    match args.output {
        Some(path) => {
            let mut output = File::create(path).expect("Unable to create file");
            for line in res_file {
                let tmp = line + "\n";
                output.write_all((*tmp).as_ref()).expect("Unable to write file");
            }
        }
        _ => {
            for line in res_file {
                println!("{line}");
            }
        }
    }
}

fn parse_unified(patch_arr: Vec<String>) -> Patch {
    let mut patch = Patch {
        file_old: String::from(""),
        file_new: String::from(""),
        huck: Vec::new(),
    };
    for str in &patch_arr {
        if str.starts_with("--- ") {
            patch.file_old = str.split(" ").collect::<Vec<&str>>().get(1).
                unwrap().parse().unwrap();
        } else if str.starts_with("+++ ") {
            patch.file_new = str.split(" ").collect::<Vec<&str>>().get(1).
                unwrap().parse().unwrap();
        } else if patch.file_new != "" && patch.file_old != "" {
            break;
        }
    }
    let mut huck = UnifiedHuck {
        old_count: 0,
        old_lines: 0,
        new_count: 0,
        new_lines: 0,
        diff: vec![],
    };
    let mut flag = false;
    let mut index = patch_arr.len();
    for str in patch_arr {
        index -= 1;
        if str.starts_with("@@ ") && str.ends_with(" @@") {
            if flag {
                patch.huck.push(huck);
            }
            flag = true;
            huck = UnifiedHuck {
                old_count: 0,
                old_lines: 0,
                new_count: 0,
                new_lines: 0,
                diff: Vec::new(),
            };
            let re = Regex::new(r"^@@ -([0-9]+),?([0-9]+)? \+([0-9]+),?([0-9]+)? @@$").unwrap();
            for cap in re.captures_iter(&str) {
                huck.old_count = cap[1].parse().unwrap();
                huck.old_lines = cap[2].parse().unwrap();
                huck.new_count = cap[3].parse().unwrap();
                huck.new_lines = cap[4].parse().unwrap();
            }
        } else {
            huck.diff.push(str);
        }
        if index == 0 {
            patch.huck.push(huck.clone());
        }
    };
    patch
}

fn parse_normal(patch_arr: Vec<String>, file_to_patch: &mut Vec<String>) {
    let mut off_set: i32 = 0;
    let re = Regex::new(r"^([0-9]+),?([0-9]+)?(d|c|a)([0-9]+),?([0-9]+)?$").unwrap();
    let mut op: char = ' ';
    let mut opl_1: i32 = 0;
    let mut opl_2: i32 = -1;
    let mut opr_1: i32 = 0;
    let mut opr_2: i32 = -1;
    for (index, line) in patch_arr.iter().enumerate() {
        if re.is_match(&line) {
            for cap in re.captures_iter(&line) {
                op = cap[3].parse().unwrap();
                opl_1 = cap[1].parse().unwrap();
                if cap.get(2) != None {
                    opl_2 = cap[2].parse().unwrap();
                } else { opl_2 = -1; }
                opr_1 = cap[4].parse().unwrap();
                if cap.get(5) != None {
                    opr_2 = cap[5].parse().unwrap();
                } else { opr_2 = -1; }
            }
            match op {
                'd' => {
                    if opl_2 != -1 {
                        file_to_patch.drain((opl_1 + off_set - 1)
                            as usize..(opl_2 + off_set) as usize);
                        off_set = off_set - (opl_2 - opl_1 + 1);
                    } else {
                        file_to_patch.remove((opl_1 + off_set) as usize);
                        off_set -= 1;
                    }
                }
                'c' => {
                    let mut cap = 1;
                    if opr_2 != -1 {
                        cap = opr_2 - opr_1 + 1;
                    }
                    let change;
                    if opl_2 != -1 {
                        change = Vec::from(&patch_arr[(index as i32 + 3 + opl_2 - opl_1) as usize..
                            (cap + index as i32 + 3 + opl_2 - opl_1) as usize]);
                        file_to_patch.drain((opl_1 + off_set - 1)
                            as usize..(opl_2 + off_set) as usize);
                    } else {
                        change = Vec::from(&patch_arr[(index as i32 + 3) as usize..
                            (cap + index as i32 + 3) as usize]);
                        file_to_patch.remove((opl_1 + off_set - 1) as usize);
                    }
                    let mut index_change = opl_1 + off_set - 1;
                    for line in change {
                        if index_change > file_to_patch.len() as i32 {
                            file_to_patch.push(line[2..].to_string());
                        } else {
                            file_to_patch.insert(index_change as usize, line[2..].to_string());
                        }
                        index_change += 1;
                    }
                    off_set += if opl_2 == -1 { -2 + cap } else { opl_2 - opl_1 + 1 - cap };
                }
                'a' => {
                    let mut cap = 1;
                    if opr_2 != -1 {
                        cap = opr_2 - opr_1 + 1;
                    }
                    let change = Vec::from(&patch_arr[index + 1..(cap + index as i32 + 1) as usize]);
                    let mut index_change = opl_1 + off_set + 1;
                    for line in change {
                        if index_change > file_to_patch.len() as i32 {
                            file_to_patch.push(line[2..].to_string());
                        } else {
                            file_to_patch.insert(index_change as usize, line[2..].to_string());
                        }
                        index_change += 1;
                        off_set += 1;
                    }
                }
                _ => {}
            }
        }
    }
}

fn parse_ed(patch_arr: Vec<String>, file_to_patch: &mut Vec<String>) {
    let re = Regex::new(r"^([0-9]+),?([0-9]+)?(d|c|a)$").unwrap();
    let mut op = ' ';
    let mut start_line = -1;
    let mut end_line = -1;
    let mut is_op = false;
    for line in patch_arr {
        if re.is_match(&line) {
            is_op = false;
            for cap in re.captures_iter(&line) {
                start_line = cap[1].parse().unwrap();
                op = cap[3].parse().unwrap();
                if cap.get(2) != None {
                    end_line = cap[2].parse().unwrap();
                } else { end_line = -1; }
            }
        } else if !line.eq(".") {
            match op {
                'a' => {
                    if start_line > file_to_patch.len() as i32 {
                        file_to_patch.push(line.to_string());
                        start_line += 1;
                    } else {
                        file_to_patch.insert(start_line as usize, line.to_string());
                        start_line += 1;
                    }
                }
                'c' => {
                    if end_line != -1 && !is_op {
                        file_to_patch.drain((start_line - 1) as usize..(end_line - 1) as usize);
                        is_op = true;
                    } else if !is_op {
                        file_to_patch.remove((start_line - 1) as usize);
                        is_op = true;
                    }
                    if start_line > file_to_patch.len() as i32 {
                        file_to_patch.push(line.to_string());
                    } else {
                        file_to_patch.insert((start_line - 1) as usize, line.to_string());
                    }
                    start_line += 1;
                }
                _ => {}
            }
        }
        if op == 'd' {
            if end_line != -1 && start_line != end_line {
                file_to_patch.drain((start_line - 1) as usize..=(end_line - 1) as usize);
            } else {
                file_to_patch.remove((start_line - 1) as usize);
            }
        }
    }
}

fn apply_unified_patch(patch: Patch, tp: Vec<String>) -> Vec<String> {
    let mut file_to_patch = Vec::new();
    if tp.is_empty() {
        let mut current_dir = env::current_dir().unwrap();
        current_dir.push(patch.file_old);
        let patch_file = File::open(current_dir).unwrap();
        let reader = std::io::BufReader::new(patch_file);
        for line in reader.lines() {
            file_to_patch.push(line.unwrap());
        }
    } else {
        file_to_patch = tp;
    }
    let mut off_set: i32 = 0;
    for huck in patch.huck {
        let old_count = huck.old_count;
        let mut diff: i32 = (old_count - 1) as i32 + off_set;
        for line in huck.diff {
            match line.chars().next().unwrap() {
                '-' => {
                    file_to_patch.remove(diff as usize);
                    diff -= 1;
                    off_set -= 1;
                }
                '+' => {
                    if diff > file_to_patch.len() as i32 {
                        file_to_patch.push(line[1..].to_string());
                    } else {
                        file_to_patch.insert(diff as usize, line[1..].parse().unwrap());
                    }
                    off_set += 1;
                }
                ' ' => {
                    assert_eq!(&line[1..], file_to_patch.get(diff as usize).unwrap());
                }
                _ => {}
            }
            diff += 1;
        }
    }
    file_to_patch
}
