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
    clean     clean the output file
    help      Print this message or the help of the given subcommand(s)

例如：

```bash
$ rpmust decode ../../src/test/stratovirt.rpm 
$ rpmust build ./RPMPackageMetadata.yaml

```

*/

use std::io;
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use rpm::*;
use flate2::read::GzDecoder;
use xz::read::{XzEncoder, XzDecoder};
use tar::Archive;
use bzip2::Compression;
use bzip2::read::{BzEncoder, BzDecoder};
use zstd::decode_all;
use glob::glob;


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
        .subcommand(
            App::new("clean")
                .about("delete the output file")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("decode", _sub_matches)) => {
            // get the rpm file path
            let file_address = _sub_matches.value_of("PATH");
            let mut file = std::fs::File::open(file_address.unwrap()).expect("should be able to open rpm file");
            
            // get size of rpm file 
            // in order to caculate the start 
            // of cpio
            let file_size = file.metadata().unwrap().len();
            let mut buf_reader = std::io::BufReader::with_capacity(file_size as usize,file);
            let rpmmeta = RPMPackageMetadata::parse(&mut buf_reader);
            let rpm = rpmmeta.unwrap();

            // output the RPMPackageMetadata.yaml
            println!("Generate the yaml file!");
            let s = serde_yaml::to_string(&rpm).unwrap();
            let mut buffer = File::create("RPMPackageMetadata.yaml").unwrap();
            buffer.write_all(s.as_bytes())?;
            println!("Yaml file generated");

            let mut out_file:File;

            // Traverse index entries to find RPMTAG_PAYLOADCOMPRESSOR 
            // RPMTAG_PAYLOADCOMPRESSOR shows the compress type
            for i in 0..rpm.header.index_entries.len() {
                if rpm.header.index_entries[i].tag == IndexTag::RPMTAG_PAYLOADCOMPRESSOR {
                    match &rpm.header.index_entries[i].data {
                        IndexData::StringTag(s) => {
                            if s == "xz" || s == "lzma" {
                                println!("Extract out.cpio.xz from rpm file");
                                out_file = File::create("out.cpio.xz")?;
                                out_file.write_all(buf_reader.fill_buf().unwrap())?;
                                let tar_xz = File::open("out.cpio.xz")?;
                                let mut xz_decoder = XzDecoder::new(tar_xz);
                                let mut buf = Vec::new();
                                xz_decoder.read_to_end(&mut buf);
                                let mut file = File::create("out.cpio")?;
                                file.write_all(&buf);
                            } else if s == "gzip" {
                                println!("Extract out.cpio.gz from rpm file");
                                out_file = File::create("out.cpio.gz")?;
                                out_file.write_all(buf_reader.fill_buf().unwrap())?;
                                let tar_gz = File::open("out.cpio.gz")?;
                                let mut gz_decoder = GzDecoder::new(tar_gz);
                                let mut buf = Vec::new();
                                gz_decoder.read_to_end(&mut buf);
                                let mut file = File::create("out.cpio")?;
                                file.write_all(&buf);
                            } else if s == "zstd" {
                                println!("Extract out.cpio.zst from rpm file");
                                out_file = File::create("out.cpio.zst")?;
                                out_file.write_all(buf_reader.fill_buf().unwrap())?;
                                let tar_zst = File::open("out.cpio.zst")?;
                                let mut tar_f = decode_all(tar_zst)?;
                                let mut file = File::create("out.cpio")?;
                                file.write_all(&tar_f);
                            } else if s == "bzip2" {
                                println!("Extract out.cpio.bz2 from rpm file");
                                out_file = File::create("out.cpio.bz2")?;
                                out_file.write_all(buf_reader.fill_buf().unwrap())?;
                                let tar_bz2 = File::open("out.cpio.bz2")?;
                                let mut bz2_decoder = BzDecoder::new(tar_bz2);
                                let mut buf = Vec::new();
                                bz2_decoder.read_to_end(&mut buf);
                                let mut file = File::create("out.cpio")?;
                                file.write_all(&buf);
                            } else {
                                println!("Extract out.cpio from rpm file");
                                out_file = File::create("out.cpio")?;
                                out_file.write_all(buf_reader.fill_buf().unwrap())?;
                            }
                        },
                        _ => {

                        }
                    }
                }
            }
            println!("Decompress the out.cpio");
            let cpio = fs::read("out.cpio").unwrap();

            for entry in cpio_reader::iter_files(&cpio) {
                let mut p = &entry.name()[2..entry.name().len()];
                let p = &("./out/".to_owned() + p);
                let path = std::path::Path::new(p);
                println!("\x1b[93mFile name:\x1b[0m {}",path.display());
                let prefix = path.parent().unwrap();
                if !path.exists() {

                    std::fs::create_dir_all(prefix).unwrap();
                    let mut f = File::create(p)?;
                    f.write_all(entry.file());
                }
            }
        }
        Some(("build", _sub_matches)) => {
            let yaml_path = _sub_matches.value_of("PATH");
            let mut file = std::fs::File::open(yaml_path.unwrap()).unwrap();
            let mut yaml_str = String::new();
            file.read_to_string(&mut yaml_str).expect("Input the yaml file path");
            let rpm: RPMPackageMetadata = serde_yaml::from_str(&yaml_str).expect("yaml read failed!");

            println!("Building the out.rpm");
            let mut file = File::create("out.rpm")?;
            rpm.write(&mut file);
            
            let mut cpio_file = Vec::new();
            for i in 0..rpm.header.index_entries.len() {
                if rpm.header.index_entries[i].tag == IndexTag::RPMTAG_PAYLOADCOMPRESSOR {
                    match &rpm.header.index_entries[i].data {
                        IndexData::StringTag(s) => {
                            if s == "xz" || s == "lzma" {
                                cpio_file = fs::read("out.cpio.xz")?;
                            } else if s == "gzip" {
                                cpio_file = fs::read("out.cpio.gz")?;
                            } else if s == "zstd" {
                                cpio_file = fs::read("out.cpio.zst")?;
                            } else if s == "bzip2" {
                                cpio_file = fs::read("out.cpio.bz2")?;
                            } else {
                                cpio_file = fs::read("out.cpio")?;
                            }
                        },
                        _ => {

                        }
                    }
                }
            }
            file.write_all(&cpio_file);
        }
        Some(("clean", _sub_matches)) => {
            let mut is = true;
            let out_dir_path = std::path::Path::new("./out");
            if out_dir_path.exists() {
                fs::remove_dir_all("./out");
                println!("\x1b[93mRemoving dir out\x1b[0m");
                is = false;
            }

            for path in glob("./*.yaml").unwrap() {
                match path {
                    Ok(path) => {
                        println!("\x1b[93mRemoving file:\x1b[0m {:?}", path.display());
                        std::fs::remove_file(path);
                        is = false;
                    },
                    Err(e) => println!("{:?}", e)
                }
            }

            for path in glob("./out*").unwrap() {
                match path {
                    Ok(path) => {
                        println!("\x1b[93mRemoving file:\x1b[0m {:?}", path.display());
                        std::fs::remove_file(path);
                        is = false;
                    },
                    Err(e) => println!("{:?}", e)
                }
            }
            if is {
                println!("nothing removed");
            }
        }
        _ => {},
    }

    
    


    
    


    
    
    
    
    

    Ok(())
}
