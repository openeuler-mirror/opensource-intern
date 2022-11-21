use std::{fs::File, io::Read};

use anyhow::{bail, Result};
use quick_xml::{events::Event, Reader};

use crate::sub_command::repoutils::HashKind;
#[derive(Debug, Clone)]
pub struct MetalinkHashInfo {
    pub kind: HashKind,
    pub value: String,
}
#[derive(Debug, Clone)]
pub struct MetalinkUrlInfo {
    pub protocol: String,
    pub kind: String,
    pub location: String,
    pub preference: i32,
    pub url: String,
}
#[derive(Debug, Clone)]
pub struct MetalinkContext {
    pub filename: String,
    pub timestamp: u128,
    pub size: u128,
    pub hashs: Vec<MetalinkHashInfo>,
    pub urls: Vec<MetalinkUrlInfo>,
}
impl MetalinkContext {
    pub fn from(path: &str) -> Result<Vec<Self>> {
        let mut buffer = String::new();
        File::open(path).unwrap().read_to_string(&mut buffer)?;
        let mut reader = Reader::from_str(buffer.as_str());
        reader.trim_text(true);
        let mut files = Vec::new();
        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"file" {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if String::from_utf8_lossy(attr.key.as_ref()) == "name" {
                                let mut mc = MetalinkContext {
                                    filename: String::from_utf8_lossy(attr.value.as_ref())
                                        .to_string(),
                                    timestamp: 0,
                                    size: 0,
                                    hashs: Vec::new(),
                                    urls: Vec::new(),
                                };
                                loop {
                                    match reader.read_event()? {
                                        Event::Start(ele) => match ele.name().as_ref() {
                                            b"size" => {
                                                mc.size = reader
                                                    .read_text(ele.name())?
                                                    .parse::<u128>()?;
                                            }
                                            b"hash" => {
                                                for attr in ele.attributes() {
                                                    let attr = attr?;
                                                    if String::from_utf8_lossy(attr.key.as_ref())
                                                        == "type"
                                                    {
                                                        let hash_info = MetalinkHashInfo {
                                                            kind: HashKind::from(
                                                                String::from_utf8_lossy(
                                                                    attr.value.as_ref(),
                                                                )
                                                                .to_string()
                                                                .as_str(),
                                                            ),
                                                            value: reader
                                                                .read_text(ele.name())?
                                                                .to_string(),
                                                        };
                                                        mc.hashs.push(hash_info);
                                                    }
                                                }
                                            }
                                            b"url" => {
                                                let mut protocol = String::from("https");
                                                let mut kind = String::from("https");
                                                let mut location = String::from("US");
                                                let mut preference = 100;
                                                for attr in ele.attributes() {
                                                    let attr = attr?;
                                                    let key =
                                                        String::from_utf8_lossy(attr.key.as_ref());
                                                    let value = String::from_utf8_lossy(
                                                        attr.value.as_ref(),
                                                    );
                                                    if key == "protocol" {
                                                        protocol = value.to_string();
                                                    } else if key == "type" {
                                                        kind = value.to_string();
                                                    } else if key == "location" {
                                                        location = value.to_string();
                                                    } else if key == "preference" {
                                                        preference = value.parse::<i32>()?;
                                                    }
                                                }
                                                let url = reader.read_text(ele.name())?.to_string();
                                                let url_info = MetalinkUrlInfo {
                                                    protocol,
                                                    kind,
                                                    location,
                                                    preference,
                                                    url,
                                                };
                                                if url_info
                                                    .protocol
                                                    .matches("http")
                                                    .collect::<Vec<_>>()
                                                    .len()
                                                    >= 1
                                                {
                                                    mc.urls.push(url_info);
                                                }
                                            }
                                            _ => {
                                                if String::from_utf8_lossy(ele.name().as_ref())
                                                    .matches("timestamp")
                                                    .collect::<Vec<_>>()
                                                    .len()
                                                    >= 1
                                                {
                                                    mc.timestamp = reader
                                                        .read_text(ele.name())?
                                                        .parse::<u128>()?;
                                                }
                                            }
                                        },
                                        Event::End(ele) => {
                                            if ele.name().as_ref() == b"file" {
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                mc.urls.sort_by(|a, b| b.preference.cmp(&a.preference));
                                files.push(mc);
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        Ok(files)
    }
    pub fn from_with_filename(path: &str, name: &str) -> Result<Self> {
        let vec = Self::from(path)?;
        let mut t = None;
        for ele in vec {
            if ele.filename == name {
                t = Some(ele);
                break;
            }
        }
        let t = match t {
            Some(t) => t,
            None => {
                bail!("check metalink url")
            }
        };
        Ok(t)
    }
}
