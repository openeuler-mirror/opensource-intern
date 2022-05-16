extern crate clap;
extern crate chrono;

use std::env;
use std::time::{UNIX_EPOCH, SystemTime};
use clap::{Arg, App, arg};
use std::fs::*;
use std::io::prelude::*;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

use chrono::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_dir = env::current_dir().unwrap();
    
    let mut file1_dir = current_dir.clone();
    file1_dir.push(args[1].clone());
    let mut file2_dir = current_dir.clone();
    file2_dir.push(args[2].clone());

    let mut file1 = File::open(file1_dir.clone()).unwrap();
    let mut file2 = File::open(file2_dir.clone()).unwrap();

    let meta1 = file1.metadata().unwrap();
    let meta2 = file2.metadata().unwrap();

    let file1_time = system_time_to_date_time(meta1.modified().unwrap());
    let file2_time = system_time_to_date_time(meta2.modified().unwrap());

    println!("--- {}   {}", file1_dir.clone().file_name().unwrap().to_str().unwrap(), file1_time);
    println!("+++ {}   {}", file2_dir.clone().file_name().unwrap().to_str().unwrap(), file2_time);

    let reader1 = std::io::BufReader::new(file1);
    let reader2 = std::io::BufReader::new(file2);
    
    let mut file1_str_array:Vec<String> = Vec::new();

    for line in reader1.lines() {
        file1_str_array.push(line.unwrap());
    }

    let mut file2_str_array:Vec<String> = Vec::new();

    for line in reader2.lines() {
        file2_str_array.push(line.unwrap());
    }

    let dp = longestCommonSubsequence(file1_str_array.clone(), file2_str_array.clone());

    let coords = get_coords(dp);

    let (first, second) = divide(coords.clone());

    let (first_map_area, second_map_area) = get_map_area(first, second,  file1_str_array.len(), file2_str_array.len());

    diff(first_map_area, second_map_area, file1_str_array, file2_str_array);
}

fn longestCommonSubsequence(text1: Vec<String>, text2: Vec<String>) -> Vec<Vec<usize>> {
    let mut dp = vec![vec![0 as usize; text2.len() + 1]; text1.len() + 1];

    for i in 1..(text1.len() + 1) {
        for j in 1..(text2.len() + 1) {
            if(text1[i - 1] == text2[j - 1]) {
                dp[i][j] = dp[i-1][j-1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i-1][j], dp[i][j-1]);
            }
        }
    }
    dp
}

fn system_time_to_date_time(t: SystemTime) -> DateTime<Local> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => { // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        },
    };
    Local.timestamp(sec, nsec)
}

fn get_coords(input: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut temp = 1 as usize;
    let mut ret: Vec<Vec<usize>> = Vec::new();
    for (i, i_v) in input.iter().enumerate() {
        for (j, j_v) in i_v.iter().enumerate() {
            if input[i][j] == temp {
                let coord = vec![i, j];
                ret.push(coord);
                temp += 1;
            }
        }
    }
    ret
}

fn diff(first_map_area: Vec<Vec<usize>>, second_map_area: Vec<Vec<usize>>, file1: Vec<String>, file2: Vec<String>) {
    let mut output_string: Vec<String> = Vec::new();
    let mut last: Vec<usize> = vec![1, 1];
    let mut row1: usize = 1;
    let mut row2: usize = 1;
    for i in 0..first_map_area.len() {
        if first_map_area[i][0] - last[0] > 3 {
            if output_string.len() != 0 {
                println!("@@ -{},{} +{},{} @@", row1 + 1, get_first_col(output_string.clone()), row2 + 1, get_sec_col(output_string.clone()));
                for i in 0..output_string.len() {
                    println!("{}", output_string[i]);
                }
                output_string = Vec::new();
            } 

            if first_map_area[i][0] - 4 <= 0 {
                row1 = 1;
            } else {
                row1 = first_map_area[i][0] - 4;
            }

            if second_map_area[i][0] - 4 <= 0 {
                row2 = 1;
            } else {
                row2 = second_map_area[i][0] - 4;
            }

            for j in 0..3 {
                output_string.push(" ".to_owned() + &file1[first_map_area[i][0] - 4 + j]);
            }

            for j in 0..first_map_area[i][1] {
                output_string.push("-".to_owned() + &file1[first_map_area[i][0] + j - 1]);
            }

            for j in 0..second_map_area[i][1] {
                output_string.push("+".to_owned() + &file2[second_map_area[i][0] + j - 1]);
            }
            
            last = vec![first_map_area[i][0] + first_map_area[i][1] - 1, 0];
            
            if i != (first_map_area.len() - 1) {
                if first_map_area[i + 1][0] - last[0] <= 3 {
                    for j in (last[0] + 1)..first_map_area[i + 1][0] {
                        output_string.push(" ".to_owned() + &file1[j - 1]);
                    }
                    last = vec![first_map_area[i + 1][0] - 1, 0];
                } else {
                    for j in 0..3 {
                        output_string.push(" ".to_owned() + &file1[last[0] + j]);
                    }
                    last = vec![last[0] + 4, 0];
                }
            } else {
                if (file1.len() + 1) - last[0] <= 3 {
                    for j in (last[0] + 1)..(file1.len() + 1) {
                        output_string.push(" ".to_owned() + &file1[j - 1]);
                    }
                    output_string.push("\\ No newline at end of file".to_owned());
                } else {
                    for j in 0..3 {
                        output_string.push(" ".to_owned() + &file1[last[0] + j]);
                    }
                }
            }

        } else {
            for j in (last[0] + 1)..first_map_area[i][0] {
                output_string.push(" ".to_owned() + &file1[j]);
            }

            for j in 0..first_map_area[i][1] {
                output_string.push("-".to_owned() + &file1[first_map_area[i][0] + j - 1]);
            }

            for j in 0..second_map_area[i][1] {
                output_string.push("+".to_owned() + &file2[second_map_area[i][0] + j - 1]);
            }

            last = vec![first_map_area[i][0] + first_map_area[i][1] - 1, 0];

            if i != (first_map_area.len() - 1) {
                if first_map_area[i + 1][0] - last[0] <= 3 {
                    for j in (last[0] + 1)..first_map_area[i + 1][0] {
                        output_string.push(" ".to_owned() + &file1[j - 1]);
                    }
                    last = vec![first_map_area[i + 1][0] - 1, 0];
                } else {
                    for j in 0..3 {
                        output_string.push(" ".to_owned() + &file1[last[0] + j]);
                    }
                    last = vec![last[0] + 3, 0];
                }
            } else {
                if (file1.len() + 1) - last[0] <= 3 {
                    for j in (last[0] + 1)..(file1.len() + 1) {
                        output_string.push(" ".to_owned() + &file1[j - 1]);
                    }
                    output_string.push("\\ No newline at end of file".to_owned());
                } else {
                    for j in 0..3 {
                        output_string.push(" ".to_owned() + &file1[last[0] + j]);
                    }
                }
            }
        }
        
    }
    println!("@@ -{},{} +{},{} @@", row1 + 1, get_first_col(output_string.clone()), row2 + 1, get_sec_col(output_string.clone()));
    for i in 0..output_string.len() {
        println!("{}", output_string[i]);
    }
}

fn get_first_col(fir: Vec<String>) -> usize {
    let mut ret = 0;
    for i in 0..fir.len() {
        let temp = fir[i].chars().next().unwrap();
        if temp == ' ' || temp == '-' {
            ret += 1;
        }
    }
    ret
}

fn get_sec_col(sec: Vec<String>) -> usize {
    let mut ret = 0;
    for i in 0..sec.len() {
        let temp = sec[i].chars().next().unwrap();
        if temp == ' ' || temp == '+' {
            ret += 1;
        }
    }
    ret
}

fn get_map_area(first: Vec<usize>, second: Vec<usize>, top1: usize, top2: usize) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut first_ret: Vec<Vec<usize>> = Vec::new();
    let mut second_ret: Vec<Vec<usize>> = Vec::new();
    if first[0] > 1 {
        let temp: Vec<usize> = vec![1, first[0] - 1];
        first_ret.push(temp);
    }
    if second[0] > 1 {
        let temp: Vec<usize> = vec![1, first[0] - 1];
        second_ret.push(temp);
    }

    if first_ret.len() > second_ret.len() {
        let mut temp: Vec<usize> = Vec::new();
        temp.push(1);
        temp.push(0);
        second_ret.push(temp);
    } else if second_ret.len() > first_ret.len() {
        let mut temp: Vec<usize> = Vec::new();
        temp.push(1);
        temp.push(0);
        first_ret.push(temp);
    }
    
    for i in 1..first.len() {
        if first[i] > first[i-1] + 1 {
            let temp: Vec<usize> = vec![first[i - 1] + 1, first[i] - first[i - 1] - 1];
            first_ret.push(temp);
        }

        if second[i] > second[i-1] + 1 {
            let temp: Vec<usize> = vec![second[i - 1] + 1, second[i] - second[i - 1] - 1];
            second_ret.push(temp);
        }

        if first_ret.len() > second_ret.len() {
            let mut temp: Vec<usize> = Vec::new();
            temp.push(second[i - 1]);
            temp.push(0);
            second_ret.push(temp);
        } else if second_ret.len() > first_ret.len() {
            let mut temp: Vec<usize> = Vec::new();
            temp.push(first[i - 1]);
            temp.push(0);
            first_ret.push(temp);
        }
    }
    if top1 > first[first.len() - 1] {
        let temp: Vec<usize> = vec![first[first.len() - 1] + 1, top1 - first[first.len() - 1]];
        first_ret.push(temp);
    }
    if top2 > second[second.len() - 1] {
        let temp: Vec<usize> = vec![second[second.len() - 1] + 1, top2 - second[second.len() - 1]];
        second_ret.push(temp);
    }
    if first_ret.len() > second_ret.len() {
        let mut temp: Vec<usize> = Vec::new();
        temp.push(second[second.len() - 1]);
        temp.push(0);
        second_ret.push(temp);
    } else if second_ret.len() > first_ret.len() {
        let mut temp: Vec<usize> = Vec::new();
        temp.push(first[first.len() - 1]);
        temp.push(0);
        first_ret.push(temp);
    }

    (first_ret, second_ret)
}

fn divide(input: Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>) {
    let mut first: Vec<usize> = Vec::new();
    let mut second: Vec<usize> = Vec::new();

    for i in 0..input.len() {
        first.push(input[i][0]);
        second.push(input[i][1]);
    }

    (first, second)
}