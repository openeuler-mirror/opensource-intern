use std::fmt;
use nom::bytes::complete;
use nom::number::complete::{be_i16, be_i32, be_i64, be_i8, be_u32, be_u8};
use num::*;
use super::constants::{self,*};
use std::convert::TryInto;
use super::errors::*;
use serde::{Serialize, Deserialize};

pub trait Tag:
    num::FromPrimitive + num::ToPrimitive + PartialEq + fmt::Display + fmt::Debug + Copy + TypeName
{
}

impl<T> Tag for T where
    T: num::FromPrimitive
        + num::ToPrimitive
        + PartialEq
        + fmt::Display
        + fmt::Debug
        + Copy
        + TypeName
{
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Header<T: num::FromPrimitive> {
    pub(crate) index_header: IndexHeader,
    pub(crate) index_entries: Vec<IndexEntry<T>>,
    pub(crate) store: Vec<u8>,
}

impl<T> Header<T> 
where
    T: Tag,
{
    pub(crate) fn parse<I: std::io::BufRead>(input: &mut I) -> Result<Header<T>, RPMError> {
        let mut buf: [u8; 16] = [0; 16];
        input.read_exact(&mut buf)?;
        let index_header = IndexHeader::parse(&buf)?;
        // read rest of header => each index consists of 16 bytes. The index header knows how large the store is.
        let mut buf = vec![0; (index_header.header_size + index_header.num_entries * 16) as usize];
        input.read_exact(&mut buf)?;
        Self::parse_header(index_header, &buf[..])
    }

    fn parse_header(index_header: IndexHeader, mut bytes: &[u8]) -> Result<Header<T>, RPMError> {
        // parse all entries
        let mut entries: Vec<IndexEntry<T>> = Vec::new();
        let mut buf_len = bytes.len();
        for _ in 0..index_header.num_entries {
            let (rest, entry) = IndexEntry::parse(bytes)?;
            entries.push(entry);
            bytes = rest;
            assert_eq!(16, buf_len - bytes.len());
            buf_len = bytes.len();
        }

        assert_eq!(bytes.len(), index_header.header_size as usize);

        let store = Vec::from(bytes);
        // add data to entries
        for entry in &mut entries {
            let mut remaining = &bytes[entry.offset as usize..];
            match &mut entry.data {
                IndexData::Null => {}
                IndexData::Char(ref mut chars) => {
                    parse_entry_data_number(remaining, entry.num_items, chars, be_u8)?;
                }
                IndexData::Int8(ref mut ints) => {
                    parse_entry_data_number(remaining, entry.num_items, ints, be_i8)?;
                }
                IndexData::Int16(ref mut ints) => {
                    parse_entry_data_number(remaining, entry.num_items, ints, be_i16)?;
                }
                IndexData::Int32(ref mut ints) => {
                    parse_entry_data_number(remaining, entry.num_items, ints, be_i32)?;
                }
                IndexData::Int64(ref mut ints) => {
                    parse_entry_data_number(remaining, entry.num_items, ints, be_i64)?;
                }
                IndexData::StringTag(ref mut string) => {
                    let (_rest, raw_string) = complete::take_till(|item| item == 0)(remaining)?;
                    string.push_str(String::from_utf8_lossy(raw_string).as_ref());
                }
                IndexData::Bin(ref mut bin) => {
                    parse_entry_data_number(remaining, entry.num_items, bin, be_u8)?;
                }
                IndexData::StringArray(ref mut strings) => {
                    for _ in 0..entry.num_items {
                        let (rest, raw_string) = complete::take_till(|item| item == 0)(remaining)?;
                        // the null byte is still in there.. we need to cut it out.
                        remaining = &rest[1..];
                        let string = String::from_utf8_lossy(raw_string).to_string();
                        strings.push(string);
                    }
                }
                IndexData::I18NString(ref mut strings) => {
                    for _ in 0..entry.num_items {
                        let (rest, raw_string) = complete::take_till(|item| item == 0)(remaining)?;
                        remaining = rest;
                        let string = String::from_utf8_lossy(raw_string).to_string();
                        strings.push(string);
                    }
                }
            }
        }

        Ok(Header {
            index_header,
            index_entries: entries,
            store,
        })
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct IndexHeader {
    /// rpm specific magic header
    pub(crate) magic: [u8; 3],
    /// rpm version number, always 1
    pub(crate) version: u8,
    /// number of header entries
    pub(crate) num_entries: u32,
    /// total header size excluding the fixed part ( I think )
    pub(crate) header_size: u32,
}

impl IndexHeader {
    pub(crate) fn parse(input: &[u8]) -> Result<Self,RPMError> {
        let (rest, magic) = complete::take(3usize)(input)?;
        for i in 0..2 {
            if HEADER_MAGIC[i] != magic[i] {
                return Err(RPMError::InvalidMagic {
                    expected: HEADER_MAGIC[i],
                    actual: magic[i],
                    complete_input: input.to_vec(),
                })
            }
        }
        let (rest, version) = be_u8(rest)?;

        if version != 1 {
            return Err(RPMError::UnsupportedHeaderVersion(version));
        }

        let (rest, _) = complete::take(4usize)(rest)?;

        let (rest, num_entries) = be_u32(rest)?;

        let (rest, header_size) = be_u32(rest)?;

        Ok(IndexHeader {
            magic: magic.try_into().unwrap(),
            version: 1,
            num_entries,
            header_size,
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct IndexEntry<T: num::FromPrimitive> {
    pub(crate) tag: T,
    pub(crate) data: IndexData,
    pub(crate) offset: i32,
    pub(crate) num_items: u32,
}
use super::constants::TypeName;

impl <T: num::FromPrimitive + num::ToPrimitive + fmt::Debug + TypeName> IndexEntry<T> {
    pub(crate) fn parse(input: &[u8]) -> Result<(&[u8], Self), RPMError> {
        //first 4 bytes are the tag.
        let (input, raw_tag) = be_u32(input)?;

        let tag: T = num::FromPrimitive::from_u32(raw_tag).ok_or_else(|| RPMError::InvalidTag {
            raw_tag: raw_tag,
            store_type: T::type_name(),
        })?;
        //next 4 bytes is the tag type
        let (input, raw_tag_type) = be_u32(input)?;

        // initialize the datatype. Parsing of the data happens later since the store comes after the index section.
        let data =
            IndexData::from_u32(raw_tag_type).ok_or_else(|| RPMError::InvalidTagDataType {
                raw_data_type: raw_tag_type,
                store_type: T::type_name(),
            })?;

        //  next 4 bytes is the offset relative to the beginning of the store
        let (input, offset) = be_i32(input)?;

        // last 4 bytes are the count that contains the number of data items pointed to by the index entry
        let (rest, num_items) = be_u32(input)?;

        Ok((
            rest,
            IndexEntry {
                tag,
                data,
                offset,
                num_items,
            },
        ))
    }
}

impl Header<IndexSignatureTag> {
    pub(crate) fn parse_signature<I: std::io::BufRead>(
        input: &mut I,
    ) -> Result<Header<IndexSignatureTag>, RPMError> {
        let result = Self::parse(input)?;
        // this structure is aligned to 8 bytes - rest is filled up with zeros.
        // if the size of our store is not a modulo of 8, we discard bytes to align to the 8 byte boundary.
        let modulo = result.index_header.header_size % 8;
        if modulo > 0 {
            let align_size = 8 - modulo;
            let mut discard = vec![0; align_size as usize];
            input.read_exact(&mut discard)?;
        }
        Ok(result)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum IndexData {
    Null,
    Char(Vec<u8>),
    Int8(Vec<i8>),
    Int16(Vec<i16>),
    Int32(Vec<i32>),
    Int64(Vec<i64>),
    StringTag(String),
    Bin(Vec<u8>),
    StringArray(Vec<String>),
    I18NString(Vec<String>),
}

impl fmt::Display for IndexData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rep = match self {
            IndexData::Null => "Null",
            IndexData::Bin(_) => "Bin",
            IndexData::Char(_) => "Char",
            IndexData::I18NString(_) => "I18NString",
            IndexData::StringTag(_) => "String",
            IndexData::StringArray(_) => "StringArray",
            IndexData::Int8(_) => "i8",
            IndexData::Int16(_) => "i16",
            IndexData::Int32(_) => "i32",
            IndexData::Int64(_) => "i64",
        };
        write!(f, "{}", rep)
    }
}

impl IndexData {
    pub(crate) fn append(&self, store: &mut Vec<u8>) -> u32 {
        match &self {
            IndexData::Null => 0,
            IndexData::Char(d) => {
                store.extend_from_slice(d);
                0
            }
            IndexData::Int8(d) => {
                for i in d.iter().map(|i| i.to_be_bytes()) {
                    store.push(i[0]);
                }
                0
            }
            IndexData::Int16(d) => {
                // align to 2 bytes

                let alignment = if store.len() % 2 != 0 {
                    store.push(0);
                    1
                } else {
                    0
                };
                let iter = d.iter().flat_map(|item| item.to_be_bytes().to_vec());
                for byte in iter {
                    store.push(byte);
                }
                alignment
            }
            IndexData::Int32(d) => {
                // align to 4 bytes
                let mut alignment = 0;
                while store.len() % 4 > 0 {
                    store.push(0);
                    alignment += 1;
                }
                let iter = d.iter().flat_map(|item| item.to_be_bytes().to_vec());
                for byte in iter {
                    store.push(byte);
                }
                alignment
            }
            IndexData::Int64(d) => {
                // align to 8 bytes
                let mut alignment = 0;
                while store.len() % 8 > 0 {
                    store.push(0);
                    alignment += 1;
                }
                let iter = d.iter().flat_map(|item| item.to_be_bytes().to_vec());
                for byte in iter {
                    store.push(byte);
                }
                alignment
            }
            IndexData::StringTag(d) => {
                store.extend_from_slice(d.as_bytes());
                store.push(0);
                0
            }
            IndexData::Bin(d) => {
                store.extend_from_slice(&d);
                0
            }
            IndexData::StringArray(d) => {
                for item in d {
                    store.extend_from_slice(item.as_bytes());
                    store.push(0);
                }
                0
            }
            IndexData::I18NString(d) => {
                for item in d {
                    store.extend_from_slice(item.as_bytes());
                    store.push(0);
                }
                0
            }
        }
    }

    pub(crate) fn num_items(&self) -> u32 {
        match self {
            IndexData::Null => 0,
            IndexData::Bin(items) => items.len() as u32,
            IndexData::Char(items) => items.len() as u32,
            IndexData::I18NString(items) => items.len() as u32,
            IndexData::StringTag(_) => 1,
            IndexData::StringArray(items) => items.len() as u32,
            IndexData::Int8(items) => items.len() as u32,
            IndexData::Int16(items) => items.len() as u32,
            IndexData::Int32(items) => items.len() as u32,
            IndexData::Int64(items) => items.len() as u32,
        }
    }
    pub(crate) fn from_u32(i: u32) -> Option<Self> {
        match i {
            0 => Some(IndexData::Null),
            1 => Some(IndexData::Char(Vec::new())),
            2 => Some(IndexData::Int8(Vec::new())),
            3 => Some(IndexData::Int16(Vec::new())),
            4 => Some(IndexData::Int32(Vec::new())),
            5 => Some(IndexData::Int64(Vec::new())),
            6 => Some(IndexData::StringTag(String::new())),
            7 => Some(IndexData::Bin(Vec::new())),
            8 => Some(IndexData::StringArray(Vec::new())),
            9 => Some(IndexData::I18NString(Vec::new())),
            _ => None,
        }
    }
    pub(crate) fn to_u32(&self) -> u32 {
        match self {
            IndexData::Null => 0,
            IndexData::Char(_) => 1,
            IndexData::Int8(_) => 2,
            IndexData::Int16(_) => 3,
            IndexData::Int32(_) => 4,
            IndexData::Int64(_) => 5,
            IndexData::StringTag(_) => 6,
            IndexData::Bin(_) => 7,

            IndexData::StringArray(_) => 8,
            IndexData::I18NString(_) => 9,
        }
    }

    pub(crate) fn as_str(&self) -> Option<&str> {
        match self {
            IndexData::StringTag(s) => Some(&s),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn as_char_array(&self) -> Option<Vec<u8>> {
        match self {
            IndexData::Char(s) => Some(s.to_vec()),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn as_i8_array(&self) -> Option<Vec<i8>> {
        match self {
            IndexData::Int8(s) => Some(s.to_vec()),
            _ => None,
        }
    }

    pub(crate) fn as_i16_array(&self) -> Option<Vec<i16>> {
        match self {
            IndexData::Int16(s) => Some(s.to_vec()),
            _ => None,
        }
    }

    pub(crate) fn as_i32(&self) -> Option<i32> {
        match self {
            IndexData::Int32(s) => {
                if !s.is_empty() {
                    Some(s[0])
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    pub(crate) fn as_i32_array(&self) -> Option<Vec<i32>> {
        match self {
            IndexData::Int32(s) => Some(s.to_vec()),
            _ => None,
        }
    }

    pub(crate) fn as_i64(&self) -> Option<i64> {
        match self {
            IndexData::Int64(s) => {
                if !s.is_empty() {
                    Some(s[0])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub(crate) fn as_i64_array(&self) -> Option<Vec<i64>> {
        match self {
            IndexData::Int64(s) => Some(s.to_vec()),
            _ => None,
        }
    }

    pub(crate) fn as_string_array(&self) -> Option<&[String]> {
        match self {
            IndexData::StringArray(d) | IndexData::I18NString(d) => Some(&d),
            _ => None,
        }
    }

    pub(crate) fn as_binary(&self) -> Option<&[u8]> {
        match self {
            IndexData::Bin(d) => Some(d.as_slice()),
            _ => None,
        }
    }
}

fn parse_entry_data_number<'a, T, E, F>(
    mut input: &'a [u8],
    num_items: u32,
    items: &mut Vec<T>,
    parser: F,
) -> nom::IResult<&'a [u8], (), E>
where
    E: nom::error::ParseError<&'a [u8]>,
    F: Fn(&'a [u8]) -> nom::IResult<&'a [u8], T, E>,
{
    for _ in 0..num_items {
        let (rest, data) = parser(input)?;
        items.push(data);
        input = rest;
    }

    Ok((input, ()))
}

