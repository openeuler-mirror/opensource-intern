mod doc;
mod error;
mod find;
mod kmp;
use std::fs;
use std::string::ToString;
fn main(){
    //Parsing json files with kmp
    let pattern = "full_name";
    let kmp = kmp::KMP::new(pattern);
    kmp.request_list("../../get_json/get_json/json.txt");
    
    // introduces a list of Repository
    let list = fs::read_to_string("../list.txt").unwrap();
    let mut cnt=1;
    let repositories_sum=8433;
    
    while &cnt<=&repositories_sum
    {
        //Get a list of mirror warehouse software based on system version
        //This is used to determine if the software is present in the system
        let sys_version=vec!["20.03-LTS","20.03-LTS-SP1","20.03-LTS-SP2","20.03-LTS-SP3","20.09","21.03","21.09","22.03-LTS"];
        let mut V:Vec<String>=Vec::new();
        for index in &sys_version{
            let mut s:String="../version_package/openEuler-".to_string();
            s.push_str(&index);
            s.push_str(".txt");
            V.push(s);
         }
         let mut v:Vec<String>=Vec::new();
         for index in V{
         let s=fs::read_to_string(&index[..]).unwrap();
         v.push(s)
         }

         //// Find the name of the repository based on the list
         let name:String=find::find_cname(&list,cnt);

         //Determine if you need to generate a purl
         let mut tag:bool=false;
         for index in &v {
         if find::find_exist(&name,&index) {tag=true;break;}
         }

         //If the tag is false, then do not download the package
         if tag==false {println!("{} no produce",cnt);cnt=cnt+1;continue;}
         
         
         //Download the software package
         println!("The {} repository is being downloaded and is named {}",cnt,&name);
         doc::git2_download(&name);

         //Build spec file path
         let name_str:&str=&name[..];
         let mut path=String::from("/root/Cangku/");
         path.push_str(&name_str);
         path.push_str("/");
         path.push_str("*.spec");
         println!("A purl is being generated for the {} repository, named {}",cnt,&name);
         let Path2:String=find::find_path(&path);
         
         //Get the contents of the spec file
         let contents = fs::read_to_string(&Path2[..]).unwrap();
         let spec :&str =&contents[..];

         //find soft package version
         let version :String="Version:".to_string();
         let mut Version :String=Default::default();
         Version=find::match_version(&contents,&version);
         
         //check verison
         println!("--{}--",Version);
         error::check_version(&Version);

         //write purl
         let mut idx:usize = 0;
         for index in &v {
                if find::find_exist(&name,&index) {doc::write_purl(&name,&Version,&sys_version[idx].to_string());}
                idx=idx+1;
            }

         //Delete soft package
         println!("The {} warehouse is being deleted, the repository name is {}",cnt,&name);
         doc::rmdir(&name);

         println!("Generation complete{}",cnt);

         cnt=cnt+1;
    }
    println!("purl has all been generated!");
}

