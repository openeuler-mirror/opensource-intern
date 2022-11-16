/*!
本项目是用rust解析patch文件，开发文档请参考：https://openeuler.feishu.cn/docs/doccn1seRdc9f8V7nqATtAy3ulf

USAGE:
    rp -- <TO_PATCH>

Arguments:
    <TO_PATCH>  The file to apply patch

Options:
    -c                   Interpret the patch file as a ordinary context diff
    -u                   Interpret the patch file as a unified context diff
    -e                   Interpret the patch file as an ed script
    -n                   Interpret the patch file as a normal diff
    -o <FILE>            Output patched files to FILE
    -i <PATCH FILE>      Read patch from PATCH FILE instead of stdin
    -h, --help           Print help information
    -V, --version        Print version information

例如：

```bash

$ rp -c -i context.patch -- file_to_apply
$ rp -u -i uni.patch -- file_to_apply

```
*/

use std::io::Write;
use std::{env, fs::File, io::BufRead};

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
        _ => Vec::new(),
    };

    let patch_str_array = match args.input {
        Some(val) => {
            let mut path = current_dir;
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
        parse_context(patch_str_array, &mut file_to_patch);
        file_to_patch
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
                output
                    .write_all((*tmp).as_ref())
                    .expect("Unable to write file");
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
            patch.file_old = str
                .split(' ')
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse()
                .unwrap();
        } else if str.starts_with("+++ ") {
            patch.file_new = str
                .split(' ')
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse()
                .unwrap();
        } else if !patch.file_new.is_empty() && !patch.file_old.is_empty() {
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
    }
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
        if re.is_match(line) {
            for cap in re.captures_iter(line) {
                op = cap[3].parse().unwrap();
                opl_1 = cap[1].parse().unwrap();
                if cap.get(2) != None {
                    opl_2 = cap[2].parse().unwrap();
                } else {
                    opl_2 = -1;
                }
                opr_1 = cap[4].parse().unwrap();
                if cap.get(5) != None {
                    opr_2 = cap[5].parse().unwrap();
                } else {
                    opr_2 = -1;
                }
            }
            match op {
                'd' => {
                    if opl_2 != -1 {
                        file_to_patch
                            .drain((opl_1 + off_set - 1) as usize..(opl_2 + off_set) as usize);
                        off_set -= opl_2 - opl_1 + 1;
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
                        change = Vec::from(
                            &patch_arr[(index as i32 + 3 + opl_2 - opl_1) as usize
                                ..(cap + index as i32 + 3 + opl_2 - opl_1) as usize],
                        );
                        file_to_patch
                            .drain((opl_1 + off_set - 1) as usize..(opl_2 + off_set) as usize);
                    } else {
                        change = Vec::from(
                            &patch_arr
                                [(index as i32 + 3) as usize..(cap + index as i32 + 3) as usize],
                        );
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
                    off_set += if opl_2 == -1 {
                        -2 + cap
                    } else {
                        opl_2 - opl_1 + 1 - cap
                    };
                }
                'a' => {
                    let mut cap = 1;
                    if opr_2 != -1 {
                        cap = opr_2 - opr_1 + 1;
                    }
                    let change =
                        Vec::from(&patch_arr[index + 1..(cap + index as i32 + 1) as usize]);
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
                } else {
                    end_line = -1;
                }
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

fn parse_context(patch_arr: Vec<String>, file_to_patch: &mut Vec<String>) {
    let mut off_set: i32 = 0;
    let re_from = Regex::new(r"^\*\*\* ([0-9]+),([0-9]+) \*\*\*\*$").unwrap();
    let re_to = Regex::new(r"^--- ([0-9]+),([0-9]+) ----$").unwrap();
    for (index, line) in patch_arr.iter().enumerate() {
        let mut op_1 = 0;
        let mut op_2 = 0;
        let mut has_change_seg = true;
        if re_from.is_match(line) {
            for cap in re_from.captures_iter(line) {
                op_1 = cap[1].parse().unwrap();
                op_2 = cap[2].parse().unwrap();
            }
            if re_to.is_match(patch_arr.get(index + 1).unwrap()) {
                has_change_seg = false;
            } else if re_to.is_match(patch_arr.get(index + op_2 as usize).unwrap()) {
                has_change_seg = true;
            }
            off_set = apply_context_huck(
                index as u32,
                off_set,
                &patch_arr,
                file_to_patch,
                op_1,
                op_2,
                has_change_seg,
            );
        }
    }
}

fn apply_context_huck(
    index: u32,
    off_set: i32,
    patch_arr: &Vec<String>,
    file_to_patch: &mut Vec<String>,
    op_1: u32,
    op_2: u32,
    has_change_seg: bool,
) -> i32 {
    let mut change_count = 0;
    let mut off_set = off_set;
    let mut index = index;
    if !has_change_seg {
        index += 2;
        let mut times = 0;
        let mut line = patch_arr.get(index as usize).unwrap().to_string();
        while line != "***************" && ((times + index) as usize) < patch_arr.len() {
            if line.starts_with("- ") {
                file_to_patch.remove(((op_1 + times) as i32 + off_set - 1) as usize);
                off_set -= 1;
            }
            if let Some(line) = line.strip_prefix("+ ") {
                if ((op_1 + times) as i32 + off_set) as usize >= file_to_patch.len() {
                    file_to_patch.push(line.to_string())
                } else {
                    file_to_patch
                        .insert(((op_1 + times) as i32 + off_set) as usize, line.to_string());
                }
                off_set += 1;
            }
            times += 1;
            if ((times + index) as usize) < patch_arr.len() {
                line = patch_arr.get((index + times) as usize).unwrap().to_string();
            }
        }
    } else {
        let mut index_change = index + op_2 - op_1 + 3;
        let mut in_change = false;
        let mut line = patch_arr.get(index_change as usize).unwrap().to_string();
        let mut changes_vec: Vec<u32> = Vec::new();
        while line != "***************" && (index_change as usize) < patch_arr.len() {
            if line.starts_with("! ") {
                if !in_change {
                    changes_vec.push((index_change as i32) as u32);
                }
                in_change = true;
            } else {
                in_change = false;
            }
            index_change += 1;
            if ((index_change) as usize) < patch_arr.len() {
                line = patch_arr.get((index_change) as usize).unwrap().to_string();
            }
        }
        index += 1;
        let mut times = 0;
        let mut line = patch_arr.get(index as usize).unwrap().to_string();
        let re_to = Regex::new(r"^--- ([0-9]+),([0-9]+) ----$").unwrap();
        while line != "***************"
            && !re_to.is_match(&line)
            && ((times + index) as usize) < patch_arr.len()
        {
            if line.starts_with("- ") {
                file_to_patch.remove(((op_1 + times) as i32 + off_set - 1) as usize);
                off_set -= 1;
            }
            if let Some(line) = line.strip_prefix("+ ") {
                if ((op_1 + times) as i32 + off_set) as usize >= file_to_patch.len() {
                    file_to_patch.push(line.to_string())
                } else {
                    file_to_patch
                        .insert(((op_1 + times) as i32 + off_set) as usize, line.to_string());
                }
                off_set += 1;
            }
            if line.starts_with("! ") {
                let mut change_start = *changes_vec.get(change_count as usize).unwrap();
                let mut change_vec: Vec<String> = Vec::new();
                let mut to_change_end_line =
                    patch_arr.get(change_start as usize).unwrap().to_string();
                while to_change_end_line.starts_with("! ")
                    && (change_start as usize) < patch_arr.len()
                {
                    change_vec.push(to_change_end_line.clone()[2..].to_string());
                    change_start += 1;
                    if ((change_start) as usize) < patch_arr.len() {
                        to_change_end_line =
                            patch_arr.get(change_start as usize).unwrap().to_string();
                    }
                }
                let mut end = index + times;
                let to_ins = index + times;
                while line.starts_with("! ") && (end as usize) < patch_arr.len() {
                    end += 1;
                    times += 1;
                    if ((times + index) as usize) < patch_arr.len() {
                        line = patch_arr.get((index + times) as usize).unwrap().to_string();
                    }
                }
                times -= 1;
                let mut start = (op_1 + times) as i32 + off_set - 1;
                off_set += change_vec.len() as i32 - (end - to_ins) as i32;
                file_to_patch.drain(start as usize..(start + (end - to_ins) as i32) as usize);
                if start as usize >= file_to_patch.len() {
                    file_to_patch.append(&mut change_vec);
                } else {
                    for ins in change_vec {
                        file_to_patch.insert(start as usize, ins);
                        start += 1;
                    }
                }
                change_count += 1;
            }
            times += 1;
            if ((times + index) as usize) < patch_arr.len() {
                line = patch_arr.get((index + times) as usize).unwrap().to_string();
            }
        }
    }
    off_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed_script() {
        let mut input_fle = vec![
            "The Way that can be told of is not the eternal Way;".to_string(),
            "The name that can be named is not the eternal name.".to_string(),
            "The Nameless is the origin of Heaven and Earth;".to_string(),
            "The Named is the mother of all things.".to_string(),
            "Therefore let there always be non-being,".to_string(),
            "  so we may see their subtlety,".to_string(),
            "And let there always be being,".to_string(),
            "  so we may see their outcome.".to_string(),
            "The two are the same,".to_string(),
            "But after they are produced,".to_string(),
            "  they have different names.".to_string(),
        ];
        let patch = vec![
            "11a".to_string(),
            "They both may be called deep and profound.".to_string(),
            "Deeper and more profound,".to_string(),
            "The door of all subtleties!".to_string(),
            ".".to_string(),
            "4c".to_string(),
            "The named is the mother of all things.".to_string(),
            "".to_string(),
            ".".to_string(),
            "1,2d".to_string(),
        ];
        let out_put = vec![
            "The Nameless is the origin of Heaven and Earth;",
            "The named is the mother of all things.",
            "",
            "Therefore let there always be non-being,",
            "  so we may see their subtlety,",
            "And let there always be being,",
            "  so we may see their outcome.",
            "The two are the same,",
            "But after they are produced,",
            "  they have different names.",
            "They both may be called deep and profound.",
            "Deeper and more profound,",
            "The door of all subtleties!",
        ];
        parse_ed(patch, &mut input_fle);
        assert_eq!(input_fle, out_put);
    }

    #[test]
    fn test_parse_normal() {
        let mut input_fle = vec![
            "The Way that can be told of is not the eternal Way;".to_string(),
            "The name that can be named is not the eternal name.".to_string(),
            "The Nameless is the origin of Heaven and Earth;".to_string(),
            "The Named is the mother of all things.".to_string(),
            "Therefore let there always be non-being,".to_string(),
            "  so we may see their subtlety,".to_string(),
            "And let there always be being,".to_string(),
            "  so we may see their outcome.".to_string(),
            "The two are the same,".to_string(),
            "But after they are produced,".to_string(),
            "  they have different names.".to_string(),
        ];
        let out_put = vec![
            "The Nameless is the origin of Heaven and Earth;",
            "The named is the mother of all things.",
            "",
            "Therefore let there always be non-being,",
            "  so we may see their subtlety,",
            "And let there always be being,",
            "  so we may see their outcome.",
            "The two are the same,",
            "But after they are produced,",
            "  they have different names.",
            "They both may be called deep and profound.",
            "Deeper and more profound,",
            "The door of all subtleties!",
        ];
        let normal_patch = vec![
            "1,2d0".to_string(),
            "< The Way that can be told of is not the eternal Way;".to_string(),
            "< The name that can be named is not the eternal name.".to_string(),
            "4c2,3".to_string(),
            "< The Named is the mother of all things.".to_string(),
            "---".to_string(),
            "> The named is the mother of all things.".to_string(),
            "> ".to_string(),
            "11a11,13".to_string(),
            "> They both may be called deep and profound.".to_string(),
            "> Deeper and more profound,".to_string(),
            "> The door of all subtleties!".to_string(),
        ];
        parse_normal(normal_patch, &mut input_fle);
        assert_eq!(input_fle, out_put);
    }

    #[test]
    fn test_parse_unified() {
        let input_fle = vec![
            "The Way that can be told of is not the eternal Way;".to_string(),
            "The name that can be named is not the eternal name.".to_string(),
            "The Nameless is the origin of Heaven and Earth;".to_string(),
            "The Named is the mother of all things.".to_string(),
            "Therefore let there always be non-being,".to_string(),
            "  so we may see their subtlety,".to_string(),
            "And let there always be being,".to_string(),
            "  so we may see their outcome.".to_string(),
            "The two are the same,".to_string(),
            "But after they are produced,".to_string(),
            "  they have different names.".to_string(),
        ];
        let out_put = vec![
            "The Nameless is the origin of Heaven and Earth;",
            "The named is the mother of all things.",
            "",
            "Therefore let there always be non-being,",
            "  so we may see their subtlety,",
            "And let there always be being,",
            "  so we may see their outcome.",
            "The two are the same,",
            "But after they are produced,",
            "  they have different names.",
            "They both may be called deep and profound.",
            "Deeper and more profound,",
            "The door of all subtleties!",
        ];
        let unified_patch = vec![
            "--- lao 2022-11-04 14:41:40.495706560 +0800".to_string(),
            "+++ tzu 2022-11-16 16:58:28.009384953 +0800".to_string(),
            "@@ -1,7 +1,6 @@".to_string(),
            "-The Way that can be told of is not the eternal Way;".to_string(),
            "-The name that can be named is not the eternal name.".to_string(),
            " The Nameless is the origin of Heaven and Earth;".to_string(),
            "-The Named is the mother of all things.".to_string(),
            "+The named is the mother of all things.".to_string(),
            "+".to_string(),
            " Therefore let there always be non-being,".to_string(),
            "   so we may see their subtlety,".to_string(),
            " And let there always be being,".to_string(),
            "@@ -9,3 +8,6 @@".to_string(),
            " The two are the same,".to_string(),
            " But after they are produced,".to_string(),
            "   they have different names.".to_string(),
            "+They both may be called deep and profound.".to_string(),
            "+Deeper and more profound,".to_string(),
            "+The door of all subtleties!".to_string(),
        ];
        let pat: Patch = parse_unified(unified_patch);
        let old = pat.file_old.clone();
        let new = pat.file_new.clone();
        assert_eq!(apply_unified_patch(pat, input_fle), out_put);
        assert_eq!(new, "tzu");
        assert_eq!(old, "lao");
    }

    #[test]
    fn test_parce_contest() {
        let mut input_fle = vec![
            "The Way that can be told of is not the eternal Way;".to_string(),
            "The name that can be named is not the eternal name.".to_string(),
            "The Nameless is the origin of Heaven and Earth;".to_string(),
            "The Named is the mother of all things.".to_string(),
            "Therefore let there always be non-being,".to_string(),
            "  so we may see their subtlety,".to_string(),
            "And let there always be being,".to_string(),
            "  so we may see their outcome.".to_string(),
            "The two are the same,".to_string(),
            "But after they are produced,".to_string(),
            "  they have different names.".to_string(),
        ];
        let out_put = vec![
            "The Nameless is the origin of Heaven and Earth;",
            "The named is the mother of all things.",
            "",
            "Therefore let there always be non-being,",
            "  so we may see their subtlety,",
            "And let there always be being,",
            "  so we may see their outcome.",
            "The two are the same,",
            "But after they are produced,",
            "  they have different names.",
            "They both may be called deep and profound.",
            "Deeper and more profound,",
            "The door of all subtleties!",
        ];
        let context = vec![
            "*** lao	2022-11-04 14:41:40.495706560 +0800".to_string(),
            "--- tzu	2022-11-04 14:42:09.495709324 +0800".to_string(),
            "***************".to_string(),
            "*** 1,7 ****".to_string(),
            "- The Way that can be told of is not the eternal Way;".to_string(),
            "- The name that can be named is not the eternal name.".to_string(),
            "  The Nameless is the origin of Heaven and Earth;".to_string(),
            "! The Named is the mother of all things.".to_string(),
            "  Therefore let there always be non-being,".to_string(),
            "    so we may see their subtlety,".to_string(),
            "  And let there always be being,".to_string(),
            "--- 1,6 ----".to_string(),
            "  The Nameless is the origin of Heaven and Earth;".to_string(),
            "! The named is the mother of all things.".to_string(),
            "! ".to_string(),
            "  Therefore let there always be non-being,".to_string(),
            "    so we may see their subtlety,".to_string(),
            "  And let there always be being,".to_string(),
            "***************".to_string(),
            "*** 9,11 ****".to_string(),
            "--- 8,13 ----".to_string(),
            "  The two are the same,".to_string(),
            "  But after they are produced,".to_string(),
            "    they have different names.".to_string(),
            "+ They both may be called deep and profound.".to_string(),
            "+ Deeper and more profound,".to_string(),
            "+ The door of all subtleties!".to_string(),
        ];
        parse_context(context, &mut input_fle);
        assert_eq!(input_fle, out_put);
    }
}
