use std::fs;
use std::io::Write;
use std::fs::OpenOptions;
use git2::Repository;
extern crate packageurl;

use std::string::ToString;
use packageurl::PackageUrl;

//// Generate the purl and write it to the file
pub fn write_purl(Name:&String,Version:&String,sys_version:&String){
    let mut ver=String::new();
    ver.push_str(&sys_version[..]);

    //Generate purl
    let purl1 = PackageUrl::new("rpm", &*Name)
     .expect("only fails if type is invalid")
     .with_namespace("openeuler")
     .with_version(Version)
     .add_qualifier("arch","arrch64")
     .unwrap()
     .add_qualifier("distro","oe1")
     .unwrap()
     .add_qualifier("version",&ver)
     .unwrap()
     .to_string();
   let purl2 = PackageUrl::new("rpm", &*Name)
   .expect("only fails if type is invalid")
   .with_namespace("openeuler")
   .with_version(Version)
   .add_qualifier("arch","source")
   .unwrap()
   .add_qualifier("distro","oe1")
   .unwrap()
   .add_qualifier("version",&ver)
   .unwrap()
   .to_string();
 let purl3 = PackageUrl::new("rpm", &*Name)
   .expect("only fails if type is invalid")
   .with_namespace("openeuler")
   .with_version(Version)
   .add_qualifier("arch","x86_64")
   .unwrap()
   .add_qualifier("distro","oe1")
   .unwrap()
   .add_qualifier("version",&ver)
   .unwrap()
   .to_string();

   let mut path1:String="../../packageurl/openEuler-".to_string();
   path1.push_str(&sys_version);
   path1.push_str("-aarch64.manifest");
   let mut path2:String="../../packageurl/openEuler-".to_string();
   path2.push_str(&sys_version);
   path2.push_str("-source.manifest");
   let mut path3:String="../../packageurl/openEuler-".to_string();
   path3.push_str(&sys_version);
   path3.push_str("-x86_64.manifest");

   //write purl
   let mut file1 = OpenOptions::new().append(true)
       .open(&path1[..])
       .expect( "cannot open file");
   file1.write_all(&Name.as_bytes())
       .expect("write failed");
   file1.write_all(":  ".as_bytes())
       .expect("write failed");
   file1.write_all(&purl1.as_bytes())
       .expect("write failed");
   file1.write_all("\n".as_bytes())
               .expect("write failed");

   let mut file2 = OpenOptions::new().append(true)
       .open(&path2[..])
       .expect( "cannot open file");
   file2.write_all(&Name.as_bytes())
       .expect("write failed");
   file2.write_all(":  ".as_bytes())
       .expect("write failed");
   file2.write_all(&purl2.as_bytes())
       .expect("write failed");
   file2.write_all("\n".as_bytes())
       .expect("write failed");
       
   let mut file3 = OpenOptions::new().append(true)
       .open(&path3[..])
       .expect( "cannot open file");
   file3.write_all(&Name.as_bytes())
       .expect("write failed");
   file3.write_all(":  ".as_bytes())
       .expect("write failed");
   file3.write_all(&purl3.as_bytes())
       .expect("write failed");
   file3.write_all("\n".as_bytes())
       .expect("write failed");


}

//For downloading software packages
pub fn git2_download(name:&String){

    let mut url =String::from("https://gitee.com/src-openeuler/");
    let name_str:&str=&name[..];
    url.push_str(&name_str);
    url.push_str(".git");
    let mut path=String::from("/root/Cangku/");
    path.push_str(&name_str);

    let repo = match Repository::clone(&url, path) {
    Ok(repo) => repo,
    Err(e) => panic!("failed to clone: {}", e), };
}

//Delete folder
pub fn rmdir(name:&String) {
   let mut path=String::from("/root/Cangku/");
   let name_str:&str=&name[..];
   path.push_str(&name_str);
   fs::remove_dir_all(path);
}
