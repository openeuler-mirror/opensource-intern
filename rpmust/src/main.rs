use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env::*;
pub mod rpm;
use rpm::*;

fn main() -> io::Result<()> {
    let mut file = std::fs::File::open("D:/rpm/opensource-intern/rpmust/src/test/stratovirt.rpm").expect("should be able to open rpm file");
    let file_size = file.metadata().unwrap().len();
    println!("{}",file_size);
    let mut buf_reader = std::io::BufReader::with_capacity(file_size as usize,file);
    let rpmmeta = RPMPackageMetadata::parse(&mut buf_reader);
    let rpm = rpmmeta.unwrap();

    //file.read_to_end(& mut buf_reader);
    println!("{:#?}",rpm.signature.index_entries);
    println!("{:#?}",rpm.header.index_entries);

    
    let mut out_file = File::create("out.cpio")?;
    
    out_file.write_all(buf_reader.fill_buf().unwrap());
    Ok(())
}
