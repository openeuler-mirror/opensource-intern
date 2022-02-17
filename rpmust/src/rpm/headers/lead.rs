use nom::bytes::complete;
use nom::number::complete::{be_u16, be_u8};
use std::io::prelude::*;
use std::fmt;
use super::constants::*;
use super::errors::*;
use serde::{Serialize, Deserialize};
use std::marker::PhantomData;
use serde::ser::{Serializer, SerializeTuple};
use serde::de::{Deserializer, Visitor, SeqAccess, Error};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Lead {
    magic: [u8; 4],
    major: u8,
    minor: u8,
    package_type: u16,
    arch: u16,
    #[serde(with = "BigArray")]
    name: [u8; 66],
    os: u16,
    signature_type: u16,
    reserved: [u8; 16],
}

impl std::fmt::Debug for Lead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = String::from_utf8_lossy(&self.name);
        f.debug_struct("Lead")
            .field("magic", &self.magic)
            .field("major", &self.major)
            .field("minor", &self.minor)
            .field("package_type", &self.package_type)
            .field("arch", &self.arch)
            .field("name", &name)
            .field("os", &self.os)
            .field("signature_type", &self.signature_type)
            .field("reserved", &self.reserved)
            .finish()
    }
}

impl Lead {
    pub fn parse(input: &[u8]) -> Result<Self,RPMError> {
        let (rest, magic) = complete::take(4usize)(input)?;
        for i in 0..magic.len() {
            if magic[i] != RPM_MAGIC[i] {
                return Err(RPMError::InvalidMagic {
                    expected: RPM_MAGIC[i],
                    actual: magic[i],
                    complete_input: input.to_vec(),
                })
            }
        }
        let (rest,major) = be_u8(rest)?;
        if major != 3 {
            return Err(RPMError::InvalidLeadMajorVersion(major));
        }
        let (rest,minor) = be_u8(rest)?;
        if minor != 0 {
            return Err(RPMError::InvalidLeadMinorVersion(minor));
        }
        let (rest, pkg_type) = be_u16(rest)?;

        if pkg_type > 1 {
            return Err(RPMError::InvalidLeadPKGType(pkg_type));
        }

        let (rest, arch) = be_u16(rest)?;
        let (rest, name) = complete::take(66usize)(rest)?;

        let (rest, os) = be_u16(rest)?;
        if os != 1 {
            return Err(RPMError::InvalidLeadOSType(os));
        }

        let (rest, sigtype) = be_u16(rest)?;
        if sigtype != 5 {
            return Err(RPMError::InvalidLeadSignatureType(sigtype));
        }

        if rest.len() != 16 {
            return Err(RPMError::InvalidReservedSpaceSize {
                expected: 16,
                actual: rest.len(),
            });
        }

        let mut name_arr: [u8; 66] = [0; 66];
        name_arr.copy_from_slice(name);

        //save unwrap here since we've checked length of slices.
        Ok(Lead {
            magic: magic.try_into().unwrap(),
            major,
            minor,
            package_type: pkg_type,
            arch,
            name: name_arr,
            os,
            signature_type: sigtype,
            reserved: rest.try_into().unwrap(),
        })
    }
}

trait BigArray<'de>: Sized {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>;
}

impl<'de, T> BigArray<'de> for [T; 66]
    where T: Default + Copy + Serialize + Deserialize<'de>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut seq = serializer.serialize_tuple(self.len())?;
        for elem in &self[..] {
            seq.serialize_element(elem)?;
        }
        seq.end()
    }

    fn deserialize<D>(deserializer: D) -> Result<[T; 66], D::Error>
        where D: Deserializer<'de>
    {
        struct ArrayVisitor<T> {
            element: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for ArrayVisitor<T>
            where T: Default + Copy + Deserialize<'de>
        {
            type Value = [T; 66];

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(concat!("an array of length ", 66))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<[T; 66], A::Error>
                where A: SeqAccess<'de>
            {
                let mut arr = [T::default(); 66];
                for i in 0..66 {
                    arr[i] = seq.next_element()?
                        .ok_or_else(|| Error::invalid_length(i, &self))?;
                }
                Ok(arr)
            }
        }

        let visitor = ArrayVisitor { element: PhantomData };
        deserializer.deserialize_tuple(66, visitor)
    }
}