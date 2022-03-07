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
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use rpm::*;
use flate2::read::GzDecoder;
use xz::read::{XzEncoder, XzDecoder};
use tar::Archive;
use bzip2::Compression;
use bzip2::read::{BzEncoder, BzDecoder};

extern crate clap;

use clap::{Arg, App, arg};

pub mod rpm;

struct CompressFormat {
    gz: GzDecoder<File>,
    xz: XzDecoder<File>,

}

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
            let s = serde_yaml::to_string(&rpm).unwrap();
            let mut buffer = File::create("RPMPackageMetadata.yaml").unwrap();
            buffer.write_all(s.as_bytes())?;

            let mut out_file:File;

            for i in 0..rpm.header.index_entries.len() {
                if rpm.header.index_entries[i].tag == IndexTag::RPMTAG_PAYLOADCOMPRESSOR {
                    match &rpm.header.index_entries[i].data {
                        IndexData::StringTag(s) => {
                            if s == "xz" || s == "lzma" {
                                out_file = File::create("out.cpio.xz")?;
                                out_file.write_all(buf_reader.fill_buf().unwrap())?;
                                let tar_xz = File::open("out.cpio.xz")?;
                                let mut xz_decoder = XzDecoder::new(tar_xz);
                                let mut buf = Vec::new();
                                xz_decoder.read_to_end(&mut buf);
                                let mut file = File::create("out.cpio")?;
                                file.write_all(&buf);
                            } else if s == "gzip" {
                                out_file = File::create("out.cpio.gz")?;
                                out_file.write_all(buf_reader.fill_buf().unwrap())?;
                                let tar_gz = File::open("out.cpio.gz")?;
                                let mut gz_decoder = GzDecoder::new(tar_gz);
                                let mut buf = Vec::new();
                                gz_decoder.read_to_end(&mut buf);
                                let mut file = File::create("out.cpio")?;
                                file.write_all(&buf);
                            } else if s == "zstd" {
                                out_file = File::create("out.cpio.zst")?;
                                out_file.write_all(buf_reader.fill_buf().unwrap())?;
                                let tar_gz = File::open("out.cpio.zst")?;
                                let mut tar_f = decode_all(tar_gz)?;
                                let mut file = File::create("out.cpio")?;
                                file.write_all(&tar_f);
                            } else if s == "bzip2" {
                                out_file = File::create("out.cpio.bz2")?;
                                out_file.write_all(buf_reader.fill_buf().unwrap())?;
                                let tar_bz2 = File::open("out.cpio.bz2")?;
                                let mut bz2_decoder = BzDecoder::new(tar_bz2);
                                let mut buf = Vec::new();
                                bz2_decoder.read_to_end(&mut buf);
                                let mut file = File::create("out.cpio")?;
                                file.write_all(&buf);
                            } else {
                                out_file = File::create("out.cpio")?;
                                out_file.write_all(buf_reader.fill_buf().unwrap())?;
                            }
                        },
                        _ => {

                        }
                    }
                }
            }
            println!("hey");
            let cpio = fs::read("out.cpio").unwrap();

            for entry in cpio_reader::iter_files(&cpio) {
                println!("Entry name: {}", entry.name());
                let mut p = &entry.name()[2..entry.name().len()];
                let p = &("./out/".to_owned() + p);
                let path = std::path::Path::new(p);
                let prefix = path.parent().unwrap();
                std::fs::create_dir_all(prefix).unwrap();
                let mut f = File::create(p)?;
                f.write_all(entry.file());
            }
            println!("hey");
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
