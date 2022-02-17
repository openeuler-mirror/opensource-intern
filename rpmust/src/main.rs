/*!
本项目是用rust解析rpm文件，开发文档请参考：https://openeuler.feishu.cn/docs/doccnBADaRx5bdfu2zHjqNfnPKe

USAGE:
    rpmust [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    build     merge the RPMPackageMetadata.yaml and out.cpio to a rpm file
    decode    turn the rpm file to RPMPackageMetadata.yaml and out.cpio
    help      Print this message or the help of the given subcommand(s)

例如：

```bash
$ rpmust decode ../../src/test/stratovirt.rpm 
$ rpmust build ./RPMPackageMetadata.yaml

```

*/

use std::io;
use std::io::prelude::*;
use std::fs::File;
use rpm::*;

extern crate clap;

use clap::{Arg, App, arg};

pub mod rpm;

fn main() -> io::Result<()> {
    let matches = App::new("rpmust")
        .version("0.1.0")
        .author("Guan jian <guanjian@isrc.iscas.ac.cn>")
        .subcommand(
            App::new("decode")
                .about("turn the rpm file to RPMPackageMetadata.yaml and out.cpio")
                .arg(arg!(<PATH> ... "The path of rpm file"))
        )
        .subcommand(
            App::new("build")
                .about("merge the RPMPackageMetadata.yaml and out.cpio to a rpm file")
                .arg(arg!(<PATH> ... "The path of RPMPackageMetadata.yaml"))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("decode", _sub_matches)) => {
            /// get the rpm file path
            let file_address = _sub_matches.value_of("PATH");
            let mut file = std::fs::File::open(file_address.unwrap()).expect("should be able to open rpm file");
            
            /// get size of rpm file 
            /// in order to caculate the start 
            /// of cpio
            let file_size = file.metadata().unwrap().len();
            let mut buf_reader = std::io::BufReader::with_capacity(file_size as usize,file);
            let rpmmeta = RPMPackageMetadata::parse(&mut buf_reader);
            let rpm = rpmmeta.unwrap();

            /// output the RPMPackageMetadata.yaml
            let s = serde_yaml::to_string(&rpm).unwrap();
            let mut buffer = File::create("RPMPackageMetadata.yaml").unwrap();
            buffer.write_all(s.as_bytes())?;

            /// output the out.cpio
            let mut out_file = File::create("out.cpio")?;
            out_file.write_all(buf_reader.fill_buf().unwrap())?;
        }
        Some(("build", _sub_matches)) => {
            let yaml_path = _sub_matches.value_of("PATH");
            let mut file = std::fs::File::open(yaml_path.unwrap()).unwrap();
            let mut yaml_str = String::new();
            file.read_to_string(&mut yaml_str).unwrap();
            let rpm: RPMPackageMetadata = serde_yaml::from_str(&yaml_str).expect("yaml read failed!");
            println!("{:#?}",rpm.signature.index_entries);
            println!("{:#?}",rpm.header.index_entries);
        }
        _ => {},
    }

    
    


    
    


    
    
    
    
    

    Ok(())
}
