use nom::bytes::complete;
use nom::number::complete::{be_u16, be_u8};

use super::errors::*;
pub const RPM_MAGIC: [u8; 4] = [0xed, 0xab, 0xee, 0xdb];
#[derive(Debug)]
pub struct Lead {
    magic: [u8; 4],
    major: u8,
    minor: u8,
    package_type: u16,
    arch: u16,
    name: [u8; 66],
    os: u16,
    signature_type: u16,
    reserved: [u8; 16],
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