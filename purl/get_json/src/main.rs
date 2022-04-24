extern crate serde_json;
use std::string::String;
use self::serde_json::{Error, Value};
use std::io::Write;
use std::fs::OpenOptions;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Build the client using the builder pattern
    let client = reqwest::Client::builder().build()?;

    // Perform the actual execution of the network request
    let mut cnt=1;
    Ok(
    while cnt<=85 {
    //Remember to update the access_token each time it is generated
    let mut path:String=String::from("https://gitee.com/api/v5/orgs/src-openeuler/repos?access_token=49bf19e5bf636a8531c42891d5f28e28&type=all&page=");
    path.push_str(&cnt.to_string()[..]);
    path.push_str("&per_page=100");
    println!("{}",&path);
    let res = client.get(path).send().await?.json::<Value>().await?;
    let ip:&str=&res.to_string()[..];
    let mut file1 = OpenOptions::new().append(true)
                .open("../json.txt")
                .expect( "cannot open file");
    file1.write_all(&ip.as_bytes())
    .expect("write failed");
    cnt+=1;
    })
    
}

