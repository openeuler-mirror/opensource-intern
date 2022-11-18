extern crate easy_http_request;
use easy_http_request::DefaultHttpRequest;
use git2::Repository;

use std::{
    fs::File,
    io::{stdin, Write},
    vec,
};

use tokei::{Config, LanguageType, Languages};
type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
#[derive(Clone)]
struct Blob {
    language: LanguageType,
    child_language: LanguageType,
    lines: usize,
    code: usize,
    comments: usize,
    blanks: usize,
    files: usize,
}

// 读取用户输入的组织名称
fn org_name() -> Result<String> {
    println!("Input organization name:");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let org = match buffer.trim_end() {
        "" => "Did not input organization name".to_owned(),
        name => name.to_string(),
    };
    Ok(org)
}

// 读取用户输入的token
fn get_token() -> Result<String> {
    println!("Input token:");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let res = match buffer.trim_end() {
        "" => "Did not input token".to_owned(),
        name => name.to_string(),
    };
    Ok(res)
}

// 读取用户输入的选择（Gitee还是GitHub）
fn choose_ways() -> Result<String> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let res = match buffer.trim_end() {
        "" => "Did not choose".to_owned(),
        name => name.to_string(),
    };
    Ok(res)
}

// 获得单个仓库的url
fn get_dep() -> Result<String> {
    println!("input url:");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let res = match buffer.trim_end() {
        "" => "Did not input url".to_owned(),
        name => name.to_string(),
    };
    Ok(res)
}

// 获取组织的所有信息
fn get_all_information(orgname: &str, usertoken: &str, whichway: i32) -> Vec<String> {
    let mut page = 1;
    let mut information: Vec<String> = Vec::new();
    let org = orgname.to_string();
    let token = usertoken.to_string();
    let api: &str;
    let rep;
    if whichway == 1 {
        api = "https://gitee.com/api/v5/orgs/";
        rep = "/repos?access_token=";
    } else {
        api = "https://api.github.com/orgs/";
        rep = "/repos?YOUR-TOKEN=";
    }
    loop {
        let url = format!(
            "{}{}{}{}&type=all&page={}&per_page=100",
            api, org, rep, token, page
        );
        let response = DefaultHttpRequest::get_from_url_str(url)
            .unwrap()
            .send()
            .unwrap();
        if response.status_code == 404 {
            panic!("Error organization name!");
        } else if response.status_code == 401 {
            panic!("Error token!");
        }
        let body = String::from_utf8(response.body);
        let p = body.unwrap();
        if p == "[]" {
            if page == 1 {
                panic!("Have no content!");
            }
            break;
        }
        information.push(p);
        page += 1;
    }
    information
}

// 从获取的所有信息中取出所有仓库的url
fn get_url(all: Vec<String>) -> Vec<String> {
    let mut url: Vec<String> = Vec::new();
    for page in all {
        let res = json::parse(&page).unwrap();
        let mut i = 0;
        while !res[i]["full_name"].is_empty() {
            url.push(res[i]["full_name"].to_string());
            i += 1;
        }
    }
    url
}

// 将组织的仓库clone到本地
fn clone_org(url: &Vec<String>) {
    for i in url {
        let url2 = "https://gitee.com/".to_string() + i;
        let location = "./".to_string() + i;
        println!("Clone {} to {}", url2, location);
        let _repo = match Repository::clone(&url2, location) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
    }
}

// 将单个仓库clone到本地
fn clone_dep(url: String) -> String {
    let name1 = url.split('/');
    let mut rep_name = "";
    if let Some(j) = name1.rev().next() {
        let name = j.split('.');
        for i in name.rev() {
            rep_name = i;
        }
    }
    let location = "./".to_string() + rep_name;
    println!("{}", location);
    let _repo = match Repository::clone(&url, location) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
    rep_name.to_string()
}

// 将统计报告写入markdown文件
fn make_markdown(mut md: &File, part: &str, analyse: Vec<String>) {
    let mut language = "".to_string();
    let mut file = "".to_string();
    let mut line = "".to_string();
    let mut code = "".to_string();
    let mut comment = "".to_string();
    let mut blank = "".to_string();
    if !analyse.is_empty() {
        language = analyse.get(0).unwrap().to_string();
        file = analyse.get(1).unwrap().to_string();
        line = analyse.get(2).unwrap().to_string();
        code = analyse.get(3).unwrap().to_string();
        comment = analyse.get(4).unwrap().to_string();
        blank = analyse.get(5).unwrap().to_string();
    }
    if part == "head" {
        md.write_all("|language| Files |Lines |Code |Comments |Blanks|\n".as_bytes())
            .unwrap();
        md.write_all(
        "|    :----:   |    :----:   |    :----:   |    :----:   |    :----:   |    :----:   |\n".as_bytes())
            .unwrap();
    } else if part == "main" || part == "main_total" {
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(language.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(file.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(line.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(code.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(comment.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(blank.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all("\n".as_bytes()).unwrap();
    } else if part == "branch_head" {
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(language.as_bytes()).unwrap();
        md.write_all("|\n".as_bytes()).unwrap();
    } else if part == "branch_total" || part == "branch" {
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(language.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(file.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(line.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(code.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(comment.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(blank.as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all("\n".as_bytes()).unwrap();
    } else {
        panic!("Markdown creation failure");
    }
}

// 将统计报告写入csv文件
fn make_csv(mut csv: &File, part: &str, analyse: Vec<String>) {
    let mut language = "".to_string();
    let mut file = "".to_string();
    let mut line = "".to_string();
    let mut code = "".to_string();
    let mut comment = "".to_string();
    let mut blank = "".to_string();
    if !analyse.is_empty() {
        language = analyse.get(0).unwrap().to_string();
        file = analyse.get(1).unwrap().to_string();
        line = analyse.get(2).unwrap().to_string();
        code = analyse.get(3).unwrap().to_string();
        comment = analyse.get(4).unwrap().to_string();
        blank = analyse.get(5).unwrap().to_string();
    }
    if part == "head" {
        csv.write_all("language,Files,Lines,Code,Comments,Blanks\n".as_bytes())
            .unwrap();
    } else if part == "main" || part == "main_total" {
        csv.write_all(language.as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(file.as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(line.as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(code.as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(comment.as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(blank.as_bytes()).unwrap();
        csv.write_all("\n".as_bytes()).unwrap();
    } else if part == "branch_head" {
        csv.write_all(language.as_bytes()).unwrap();
        csv.write_all("\n".as_bytes()).unwrap();
    } else if part == "branch_total" || part == "branch" {
        csv.write_all(language.as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(file.as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(line.as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(code.as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(comment.as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(blank.as_bytes()).unwrap();
        csv.write_all("\n".as_bytes()).unwrap();
    } else {
        panic!("Csv creation failure");
    }
}

// 生成tokei的统计报告
fn tokei(path: &str) {
    println!("results are as follows:");
    let paths = &[path];
    let excluded = &["target", ".gitignore", ".ignore"];
    let config = Config::from_config_files();
    let mut languages = Languages::new();
    languages.get_statistics(paths, excluded, &config);
    let lang = languages;
    println!(
        "{:=^20}{:=^20}{:=^20}{:=^20}{:=^20}{:=^20}",
        "=", "=", "=", "=", "=", "="
    );
    println!(
        "{:<20}{:>20}{:>20}{:>20}{:>20}{:>20}",
        "language", "Files", "Lines", "Code", "Comments", "Blanks"
    );
    println!(
        "{:=^20}{:=^20}{:=^20}{:=^20}{:=^20}{:=^20}",
        "=", "=", "=", "=", "=", "="
    );
    let mut branch: Vec<Blob> = Vec::new();
    let mut main: Vec<Blob> = Vec::new();
    let mut total: [usize; 5] = [0, 0, 0, 0, 0];
    for each in lang {
        let mut count_file = 0;
        let mut count_line = 0;
        let mut count_code = 0;
        let mut count_commit = 0;
        let mut count_blank = 0;
        for report in each.1.reports {
            total[0] += 1;
            total[1] += report.stats.lines();
            total[2] += report.stats.code;
            total[3] += report.stats.comments;
            total[4] += report.stats.blanks;
            count_file += 1;
            count_line += report.stats.lines();
            count_code += report.stats.code;
            count_commit += report.stats.comments;
            count_blank += report.stats.blanks;
            if report.stats.blobs.is_empty() {
                continue;
            } else {
                '_a: for b in report.stats.blobs {
                    let blob2 = Blob {
                        language: each.0,
                        child_language: b.0,
                        lines: b.1.lines(),
                        code: b.1.code,
                        comments: b.1.comments,
                        blanks: b.1.blanks,
                        files: 1,
                    };

                    let l = branch.len();
                    let mut n = 0;
                    while n != l {
                        if blob2.child_language == branch[n].child_language {
                            branch[n].lines += blob2.lines;
                            branch[n].code += blob2.code;
                            branch[n].comments += blob2.comments;
                            branch[n].blanks += blob2.blanks;
                            branch[n].files += blob2.files;
                            continue '_a;
                        }
                        n += 1;
                    }
                    branch.push(blob2);
                }
            }
        }
        if count_file != 0 {
            let blob1 = Blob {
                language: each.0,
                child_language: each.0,
                lines: count_line,
                code: count_code,
                comments: count_commit,
                blanks: count_blank,
                files: count_file,
            };
            main.push(blob1);
        }
    }
    let md_path = path.to_string() + ".md";
    let csv_path = path.to_string() + ".csv";
    let md = File::create(md_path).unwrap();
    let csv = File::create(csv_path).unwrap();
    make_markdown(&md, "head", vec![]);
    make_csv(&csv, "head", vec![]);
    for a in main {
        println!(
            "{:<20}{:>20}{:>20}{:>20}{:>20}{:>20}",
            a.language.to_string(),
            a.files,
            a.lines,
            a.code,
            a.comments,
            a.blanks
        );
        make_markdown(
            &md,
            "main",
            vec![
                a.language.to_string(),
                a.files.to_string(),
                a.lines.to_string(),
                a.code.to_string(),
                a.comments.to_string(),
                a.blanks.to_string(),
            ],
        );
        make_csv(
            &csv,
            "main",
            vec![
                a.language.to_string(),
                a.files.to_string(),
                a.lines.to_string(),
                a.code.to_string(),
                a.comments.to_string(),
                a.blanks.to_string(),
            ],
        );
    }
    make_markdown(
        &md,
        "main_total",
        vec![
            "(total)".to_string(),
            total[0].to_string(),
            total[1].to_string(),
            total[2].to_string(),
            total[3].to_string(),
            total[4].to_string(),
        ],
    );
    make_csv(
        &csv,
        "main_total",
        vec![
            "(total)".to_string(),
            total[0].to_string(),
            total[1].to_string(),
            total[2].to_string(),
            total[3].to_string(),
            total[4].to_string(),
        ],
    );
    println!(
        "{:<20}{:>20}{:>20}{:>20}{:>20}{:>20}",
        "(total)", total[0], total[1], total[2], total[3], total[4]
    );
    let mut print = true;
    if print && !branch.is_empty() {
        let mut lan = branch[0].language;
        let mut total_child: [usize; 5] = [0, 0, 0, 0, 0];
        println!(
            "{:-^20}{:-^20}{:-^20}{:-^20}{:-^20}{:-^20}",
            "-", "-", "-", "-", "-", "-"
        );
        println!("{:<20}", branch[0].language.to_string());
        make_markdown(
            &md,
            "branch_head",
            vec![
                branch[0].language.to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
        );
        make_csv(
            &csv,
            "branch_head",
            vec![
                branch[0].language.to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
        );
        for b in branch {
            if lan == b.language {
                print = false;
            } else {
                print = true;
            }
            if print {
                make_markdown(
                    &md,
                    "branch_total",
                    vec![
                        "(total)".to_string(),
                        total_child[0].to_string(),
                        total_child[1].to_string(),
                        total_child[2].to_string(),
                        total_child[3].to_string(),
                        total_child[4].to_string(),
                    ],
                );
                make_csv(
                    &csv,
                    "branch_total",
                    vec![
                        "(total)".to_string(),
                        total_child[0].to_string(),
                        total_child[1].to_string(),
                        total_child[2].to_string(),
                        total_child[3].to_string(),
                        total_child[4].to_string(),
                    ],
                );
                println!(
                    "{:<20}{:>20}{:>20}{:>20}{:>20}{:>20}",
                    "(total)",
                    total_child[0],
                    total_child[1],
                    total_child[2],
                    total_child[3],
                    total_child[4]
                );
                println!(
                    "{:-^20}{:-^20}{:-^20}{:-^20}{:-^20}{:-^20}",
                    "-", "-", "-", "-", "-", "-"
                );
                println!("{:<20}", b.language.to_string());
                make_markdown(
                    &md,
                    "branch_head",
                    vec![
                        b.language.to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                    ],
                );
                make_csv(
                    &csv,
                    "branch_head",
                    vec![
                        b.language.to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                    ],
                );
                total_child[0] = 0;
                total_child[1] = 0;
                total_child[2] = 0;
                total_child[3] = 0;
                total_child[4] = 0;
            }
            lan = b.language;
            total_child[0] += b.files;
            total_child[1] += b.lines;
            total_child[2] += b.code;
            total_child[3] += b.comments;
            total_child[4] += b.blanks;
            println!(
                "|-{:<18}{:>20}{:>20}{:>20}{:>20}{:>20}",
                b.child_language.to_string(),
                b.files,
                b.lines,
                b.code,
                b.comments,
                b.blanks
            );
            make_markdown(
                &md,
                "branch",
                vec![
                    b.child_language.to_string(),
                    b.files.to_string(),
                    b.lines.to_string(),
                    b.code.to_string(),
                    b.comments.to_string(),
                    b.blanks.to_string(),
                ],
            );
            make_csv(
                &csv,
                "branch",
                vec![
                    b.child_language.to_string(),
                    b.files.to_string(),
                    b.lines.to_string(),
                    b.code.to_string(),
                    b.comments.to_string(),
                    b.blanks.to_string(),
                ],
            );
        }
        make_markdown(
            &md,
            "branch_total",
            vec![
                "(total)".to_string(),
                total_child[0].to_string(),
                total_child[1].to_string(),
                total_child[2].to_string(),
                total_child[3].to_string(),
                total_child[4].to_string(),
            ],
        );
        make_csv(
            &csv,
            "branch_total",
            vec![
                "(total)".to_string(),
                total_child[0].to_string(),
                total_child[1].to_string(),
                total_child[2].to_string(),
                total_child[3].to_string(),
                total_child[4].to_string(),
            ],
        );
        println!(
            "{:<20}{:>20}{:>20}{:>20}{:>20}{:>20}",
            "(total)",
            total_child[0],
            total_child[1],
            total_child[2],
            total_child[3],
            total_child[4]
        );
    }
}

fn main() {
    println!("Enter 1 to generate statistics for a single code repository");
    println!(
        "Enter 2 to perform code aggregate statistics for all warehouses in the organization:"
    );
    let choose1 = choose_ways().unwrap();
    if choose1 == "1" {
        let url = get_dep().unwrap();
        let path = clone_dep(url);
        tokei(&path);
    } else if choose1 == "2" {
        println!("Enter 1 for gitee, enter 2 for github:");
        let choose2 = choose_ways().unwrap();
        let mut which = 0;
        if which == 0 && choose2 == "1" {
            which = 1;
        } else if which == 0 && choose2 == "2" {
            which = 2;
        } else {
            panic!("Please input correctly!");
        }
        let org = org_name().unwrap();
        let token = get_token().unwrap();
        let information = get_all_information(&org, &token, which);
        let url = get_url(information);
        clone_org(&url);
        tokei(&org);
    } else {
        panic!("Please input correctly!");
    }
}

#[cfg(test)]
mod tests {
    use std::{io::{Read, BufReader, BufRead}, fs::remove_file};

    use super::*;

    #[test]
    #[should_panic]
    fn test_get_all_information_fail() {
        get_all_information("", "b62f886ba07dcbf2c52c6d9b93463401", 1);
        get_all_information("", "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", 2);
    }
    #[test]
    fn test_get_all_information() {
        let vec1 = get_all_information("zs_9", "b62f886ba07dcbf2c52c6d9b93463401", 1);
        assert!(!vec1.is_empty());
        let vec2 = get_all_information("4Paradigm", "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", 2);
        assert!(!vec2.is_empty());
    }

    #[test]
    fn test_get_url() {
        let vec: Vec<String> = vec!["[{\"full_name\":\"test\"}]".to_string()];
        let url = get_url(vec);
        assert_eq!("test", url[0]);
    }
    #[test]
    fn test_make_markdown() {
        let md = File::create("test_markdown.md").unwrap();
        make_markdown(&md, "head", vec![]);
        let mut file = std::fs::File::open("test_markdown.md").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!("|language| Files |Lines |Code |Comments |Blanks|\n|    :----:   |    :----:   |    :----:   |    :----:   |    :----:   |    :----:   |\n",contents);
        remove_file("test_markdown.md").unwrap();
    }

    #[test]
    fn test_make_csv() {
        let csv = File::create("test_csv.csv").unwrap();
        make_csv(&csv, "head", vec![]);
        let mut file = std::fs::File::open("test_csv.csv").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!("language,Files,Lines,Code,Comments,Blanks\n",contents);
        remove_file("test_csv.csv").unwrap();
    }
    #[test]
    fn test_tokei(){
        tokei("test");
        let md_file_read = BufReader::new(File::open("test.md").unwrap());
        let mut index=0;
        assert_eq!(4,BufReader::new( File::open("test.md").unwrap()).lines().count());
        for line in md_file_read.lines() {
        if index==0 {
            assert_eq!("|language| Files |Lines |Code |Comments |Blanks|",line.unwrap());
        }else if index==1{
            assert_eq!("|    :----:   |    :----:   |    :----:   |    :----:   |    :----:   |    :----:   |",line.unwrap());
        }else if index==2 {
            assert_eq!("|C++|1|6|5|0|1|",line.unwrap());
        }else if index==3 {
            assert_eq!("|(total)|1|6|5|0|1|",line.unwrap());
        }
        index+=1;
        }
        let csv_file_read = BufReader::new( File::open("test.csv").unwrap());
        let mut index=0;
        assert_eq!(3,BufReader::new( File::open("test.csv").unwrap()).lines().count());
        for line in csv_file_read.lines() {
            if index==0 {
                assert_eq!("language,Files,Lines,Code,Comments,Blanks",line.unwrap());
            }else if index==1 {
                assert_eq!("C++,1,6,5,0,1",line.unwrap());
            }else if index==2 {
                assert_eq!("(total),1,6,5,0,1",line.unwrap());
            }
            index+=1;
        }
    }
}
