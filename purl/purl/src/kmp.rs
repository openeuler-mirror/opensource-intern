use std::fs;
use std::io::Write;
use std::{str, fs::OpenOptions};
//use kmp algorithm Parse the json file, extract the filename, and write it to list.txt
pub struct KMP {
    pattern: Vec<char>,
    failure_function: Vec<usize>,
    pattern_length: usize
}

impl KMP {
    pub fn new(pattern: &str) -> KMP {
        let pattern: Vec<char> = pattern.chars().collect();
        let pattern_length = pattern.len();
        KMP {
            failure_function: KMP::find_failure_function(&pattern),
            pattern: pattern,
            pattern_length: pattern_length
        }
    }

    fn find_failure_function(pattern: &Vec<char>) -> Vec<usize>{
        let mut i = 1;
        let mut j = 0;
        let pattern_length = pattern.len();
        let end_i = pattern_length - 1;
        let mut failure_function = vec![0usize; pattern_length];
        while i <= end_i {
            if pattern[i] == pattern[j] {
                failure_function[i] = j + 1;
                i = i + 1;
                j = j + 1;
            } else {
                if j == 0 {
                    failure_function[i] = 0;
                    i = i + 1;
                } else {
                    j = failure_function[j - 1];
                }
            }
        }
        failure_function
    }

    pub fn index_of_any(&self, target: &str) -> i32 {
        let mut file = OpenOptions::new().append(true)
                .open("../list.txt")
                .expect( "cannot open file");
        let target: Vec<char> = target.chars().collect();
        let mut t_i: usize = 0;
        let mut p_i: usize = 0;
        let mut cnt:i32 =0;
        let target_len = target.len();
        let mut result_idx = 0i32;
        let pattern_len = self.pattern_length;
        while (t_i <= target_len - 1) && (p_i <= pattern_len - 1) {
            if target[t_i] == self.pattern[p_i] {
                if result_idx == 0 {
                    result_idx = t_i as i32;
                }
                t_i = t_i + 1;
                p_i = p_i + 1;
                if p_i >= pattern_len{
                    //Write operation after matching a string
                    let mut st:usize=(result_idx+26)as usize;
                    let mut fen:usize =target_len-1;
                    while target[st]!='"'{
                         st+=1;
                     }
                    fen=st;
                    let s=&target[(result_idx+26)as usize..fen];
                    let S:String=s.iter().collect::<String>();
                    let mut list:String=(cnt+1).to_string();
                    list.push_str(" ");
                    list.push_str(&S[..]);
                    list.push_str("\n");
                    file.write_all(&list.as_bytes())
                    .expect("write failed");
                    cnt+=1;
                    p_i = self.failure_function[p_i - 1];
                }
            } else {
                if p_i == 0 {
                    p_i = 0;
                } else {
                    p_i = self.failure_function[p_i - 1];
                }
                t_i = t_i + 1;
                result_idx = 0;
            }
        }
        cnt
    }
    pub fn request_list(&self,path:&str){
        let p = fs::read_to_string(path).unwrap();
        println!("Generated a list of {} repositories",self.index_of_any(&p[..]));
    }
}