#[derive(Debug)]
pub struct Header<T: num::FromPrimitive> {
    pub(crate) index_header: IndexHeader,
    pub(crate) index_entries: Vec<IndexEntry<T>>,
    pub(crate) store: Vec<u8>,
}

impl<T> Header<T>
where
    T: Tag,
    {
        fn parse<I: std::io::BufReader>(input: &mut I) -> Result<Header<T>,RPMError> {
            let mut buf: [u8; 16] = [0; 16];
            input.read_exact(&mut buf)?;
            let index_header = IndexHeader::parse(&buf)?;

        }
    }

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct IndexEntry<T: num::FromPrimitive> {
    pub(crate) tag: T,
    pub(crate) data: IndexData,
    pub(crate) offset: i32,
    pub(crate) num_items: u32,
}

#[derive(Debug)]
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