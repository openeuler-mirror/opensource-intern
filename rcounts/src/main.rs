extern crate easy_http_request;
use easy_http_request::DefaultHttpRequest;
use git2::Repository;

use std::{
    fs::File,
    io::{stdin, Write},
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

fn choose() -> Result<String> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let res = match buffer.trim_end() {
        "" => "Did not choose".to_owned(),
        name => name.to_string(),
    };
    Ok(res)
}

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

fn get(org1: &str, token1: &str, which: i32) -> Vec<String> {
    let mut page = 1;
    let mut j: Vec<String> = Vec::new();
    let org = org1.to_string();
    let token = token1.to_string();
    let api: &str;
    let rep;
    if which == 1 {
        api = "https://gitee.com/api/v5/orgs/";
        rep = "/repos?access_token=";
    } else {
        api = "https://api.github.com/orgs/";
        rep = "/repos?YOUR-TOKEN=";
    }
    loop {
        let url = api.to_string()
            + &org
            + rep
            + &token
            + "&type=all&page="
            + &page.to_string()
            + "&per_page=100";
        let response = DefaultHttpRequest::get_from_url_str(url)
            .unwrap()
            .send()
            .unwrap();
        let body = String::from_utf8(response.body);
        let p = body.unwrap();
        if p == "[]" {
            if page == 1 {
                panic!("Have no content!");
            }
            break;
        }
        j.push(p);
        page += 1;
    }
    return j;
}

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
    return url;
}

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

fn clone_dep(url: String) -> String {
    let name1 = url.split('/');
    let mut rep_name = "";
    for j in name1.rev() {
        let name = j.split('.');
        for i in name.rev() {
            rep_name = i;
        }
        break;
    }
    let location = "./".to_string() + &rep_name;
    println!("{}", location);
    let _repo = match Repository::clone(&url, location) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
    return rep_name.to_string();
}

fn tokei(path: &str) {
    println!("results are as follows:");
    let paths = &[path];
    let excluded = &["target", ".gitignore", ".ignore"];
    let config = Config::from_config_files();
    let mut languages = Languages::new();
    languages.get_statistics(paths, excluded, &config);
    let ser = languages;
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
    for r in ser {
        let mut count_file = 0;
        let mut count_line = 0;
        let mut count_code = 0;
        let mut count_commit = 0;
        let mut count_blank = 0;
        for s in r.1.reports {
            total[0] += 1;
            total[1] += s.stats.lines();
            total[2] += s.stats.code;
            total[3] += s.stats.comments;
            total[4] += s.stats.blanks;
            count_file += 1;
            count_line += s.stats.lines();
            count_code += s.stats.code;
            count_commit += s.stats.comments;
            count_blank += s.stats.blanks;
            if s.stats.blobs.is_empty() {
                continue;
            } else {
                '_a: for dd in s.stats.blobs {
                    let blob2 = Blob {
                        language: r.0,
                        child_language: dd.0,
                        lines: dd.1.lines(),
                        code: dd.1.code,
                        comments: dd.1.comments,
                        blanks: dd.1.blanks,
                        files: 1,
                    };

                    let f = branch.len();
                    let mut n = 0;
                    while n != f {
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
                language: r.0,
                child_language: r.0,
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
    let mut md = File::create(md_path).unwrap();
    let mut csv = File::create(csv_path).unwrap();
    md.write_all("|language| Files |Lines |Code |Comments |Blanks|\n".as_bytes())
        .unwrap();
    md.write_all(
        "|    :----:   |    :----:   |    :----:   |    :----:   |    :----:   |    :----:   |\n"
            .as_bytes(),
    )
    .unwrap();

    csv.write_all("language,Files,Lines,Code,Comments,Blanks\n".as_bytes())
        .unwrap();
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
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(a.language.to_string().as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(a.files.to_string().as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(a.lines.to_string().as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(a.code.to_string().as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(a.comments.to_string().as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(a.blanks.to_string().as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all("\n".as_bytes()).unwrap();

        csv.write_all(a.language.to_string().as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(a.files.to_string().as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(a.lines.to_string().as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(a.code.to_string().as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(a.comments.to_string().as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(a.blanks.to_string().as_bytes()).unwrap();
        csv.write_all("\n".as_bytes()).unwrap();
    }
    md.write_all("|".as_bytes()).unwrap();
    md.write_all("(total)".as_bytes()).unwrap();
    md.write_all("|".as_bytes()).unwrap();
    md.write_all(total[0].to_string().as_bytes()).unwrap();
    md.write_all("|".as_bytes()).unwrap();
    md.write_all(total[1].to_string().as_bytes()).unwrap();
    md.write_all("|".as_bytes()).unwrap();
    md.write_all(total[2].to_string().as_bytes()).unwrap();
    md.write_all("|".as_bytes()).unwrap();
    md.write_all(total[3].to_string().as_bytes()).unwrap();
    md.write_all("|".as_bytes()).unwrap();
    md.write_all(total[4].to_string().as_bytes()).unwrap();
    md.write_all("|".as_bytes()).unwrap();
    md.write_all("\n".as_bytes()).unwrap();

    csv.write_all("(total)".as_bytes()).unwrap();
    csv.write_all(",".as_bytes()).unwrap();
    csv.write_all(total[0].to_string().as_bytes()).unwrap();
    csv.write_all(",".as_bytes()).unwrap();
    csv.write_all(total[1].to_string().as_bytes()).unwrap();
    csv.write_all(",".as_bytes()).unwrap();
    csv.write_all(total[2].to_string().as_bytes()).unwrap();
    csv.write_all(",".as_bytes()).unwrap();
    csv.write_all(total[3].to_string().as_bytes()).unwrap();
    csv.write_all(",".as_bytes()).unwrap();
    csv.write_all(total[4].to_string().as_bytes()).unwrap();
    csv.write_all("\n".as_bytes()).unwrap();
    println!(
        "{:<20}{:>20}{:>20}{:>20}{:>20}{:>20}",
        "(total)", total[0], total[1], total[2], total[3], total[4]
    );
    let mut print = true;

    if branch.len() != 0 {
        let mut lan = branch[0].language;
        let mut total_child: [usize; 5] = [0, 0, 0, 0, 0];
        println!(
            "{:-^20}{:-^20}{:-^20}{:-^20}{:-^20}{:-^20}",
            "-", "-", "-", "-", "-", "-"
        );
        println!("{:<20}", branch[0].language.to_string());

        md.write_all("|".as_bytes()).unwrap();
        md.write_all(branch[0].language.to_string().as_bytes())
            .unwrap();
        md.write_all("|\n".as_bytes()).unwrap();

        csv.write_all(branch[0].language.to_string().as_bytes())
            .unwrap();
        csv.write_all("\n".as_bytes()).unwrap();
        for b in branch {
            if lan == b.language {
                print = false;
            } else {
                print = true;
            }
            if print {
                md.write_all("|".as_bytes()).unwrap();
                md.write_all("(total)".as_bytes()).unwrap();
                md.write_all("|".as_bytes()).unwrap();
                md.write_all(total_child[0].to_string().as_bytes()).unwrap();
                md.write_all("|".as_bytes()).unwrap();
                md.write_all(total_child[1].to_string().as_bytes()).unwrap();
                md.write_all("|".as_bytes()).unwrap();
                md.write_all(total_child[2].to_string().as_bytes()).unwrap();
                md.write_all("|".as_bytes()).unwrap();
                md.write_all(total_child[3].to_string().as_bytes()).unwrap();
                md.write_all("|".as_bytes()).unwrap();
                md.write_all(total_child[4].to_string().as_bytes()).unwrap();
                md.write_all("|".as_bytes()).unwrap();
                md.write_all("\n".as_bytes()).unwrap();

                csv.write_all("(total)".as_bytes()).unwrap();
                csv.write_all(",".as_bytes()).unwrap();
                csv.write_all(total_child[0].to_string().as_bytes())
                    .unwrap();
                csv.write_all(",".as_bytes()).unwrap();
                csv.write_all(total_child[1].to_string().as_bytes())
                    .unwrap();
                csv.write_all(",".as_bytes()).unwrap();
                csv.write_all(total_child[2].to_string().as_bytes())
                    .unwrap();
                csv.write_all(",".as_bytes()).unwrap();
                csv.write_all(total_child[3].to_string().as_bytes())
                    .unwrap();
                csv.write_all(",".as_bytes()).unwrap();
                csv.write_all(total_child[4].to_string().as_bytes())
                    .unwrap();
                csv.write_all("\n".as_bytes()).unwrap();

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
                md.write_all("|".as_bytes()).unwrap();
                md.write_all(b.language.to_string().as_bytes()).unwrap();
                md.write_all("|\n".as_bytes()).unwrap();
                csv.write_all(b.language.to_string().as_bytes()).unwrap();
                csv.write_all("\n".as_bytes()).unwrap();
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
            md.write_all("|".as_bytes()).unwrap();
            md.write_all(b.child_language.to_string().as_bytes())
                .unwrap();
            md.write_all("|".as_bytes()).unwrap();
            md.write_all(b.files.to_string().as_bytes()).unwrap();
            md.write_all("|".as_bytes()).unwrap();
            md.write_all(b.lines.to_string().as_bytes()).unwrap();
            md.write_all("|".as_bytes()).unwrap();
            md.write_all(b.code.to_string().as_bytes()).unwrap();
            md.write_all("|".as_bytes()).unwrap();
            md.write_all(b.comments.to_string().as_bytes()).unwrap();
            md.write_all("|".as_bytes()).unwrap();
            md.write_all(b.blanks.to_string().as_bytes()).unwrap();
            md.write_all("|".as_bytes()).unwrap();
            md.write_all("\n".as_bytes()).unwrap();

            csv.write_all(b.child_language.to_string().as_bytes())
                .unwrap();
            csv.write_all(",".as_bytes()).unwrap();
            csv.write_all(b.files.to_string().as_bytes()).unwrap();
            csv.write_all(",".as_bytes()).unwrap();
            csv.write_all(b.lines.to_string().as_bytes()).unwrap();
            csv.write_all(",".as_bytes()).unwrap();
            csv.write_all(b.code.to_string().as_bytes()).unwrap();
            csv.write_all(",".as_bytes()).unwrap();
            csv.write_all(b.comments.to_string().as_bytes()).unwrap();
            csv.write_all(",".as_bytes()).unwrap();
            csv.write_all(b.blanks.to_string().as_bytes()).unwrap();
            csv.write_all("\n".as_bytes()).unwrap();
        }

        md.write_all("|".as_bytes()).unwrap();
        md.write_all("(total)".as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(total_child[0].to_string().as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(total_child[1].to_string().as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(total_child[2].to_string().as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(total_child[3].to_string().as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all(total_child[4].to_string().as_bytes()).unwrap();
        md.write_all("|".as_bytes()).unwrap();
        md.write_all("\n".as_bytes()).unwrap();

        csv.write_all("(total)".as_bytes()).unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(total_child[0].to_string().as_bytes())
            .unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(total_child[1].to_string().as_bytes())
            .unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(total_child[2].to_string().as_bytes())
            .unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(total_child[3].to_string().as_bytes())
            .unwrap();
        csv.write_all(",".as_bytes()).unwrap();
        csv.write_all(total_child[4].to_string().as_bytes())
            .unwrap();
        csv.write_all("\n".as_bytes()).unwrap();

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
    let choose1 = choose().unwrap();
    if choose1 == "1" {
        let url = get_dep().unwrap();
        let path = clone_dep(url);
        tokei(&path);
    } else if choose1 == "2" {
        println!("Enter 1 for gitee, enter 2 for github:");
        let choose2 = choose().unwrap();
        let mut which = 0;
        if choose2 == "1" {
            which = 1;
        } else if choose2 == "2" {
            which = 2;
        } else {
            panic!("Please input correctly!");
        }
        let org = org_name().unwrap();
        let token = get_token().unwrap();
        let x = get(&org, &token, which);
        let url = get_url(x);
        clone_org(&url);
        tokei(&org);
    } else {
        panic!("Please input correctly!");
    }
}
