use std::io;
use std::io::prelude::*;
use std::fs::File;
mod headers;
use headers::*;

fn main() -> io::Result<()>{
    let mut file = std::fs::File::open("./test/stratovirt.rpm").expect("should be able to open rpm file");
    //let mut buf_reader = std::io::BufReader::new(file);
    let mut lead_buffer = [0; LEAD_SIZE];
    file.read_exact(&mut lead_buffer)?;
    let lead = Lead::parse(&lead_buffer);
    println!("{:?}", lead);
    Ok(())
}
