use thiserror::Error;
use nom;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RPMError {
    #[error("{0}")]
    Nom(String),
    #[error(
        "invalid magic expected: {expected} but got: {actual} - whole input was {complete_input:?}"
    )]
    InvalidMagic {
        expected: u8,
        actual: u8,
        complete_input: Vec<u8>,
    },
    #[error(
        "unsupported lead major version {0} - only version 3 is supported"
    )]
    InvalidLeadMajorVersion(u8),

    #[error("unsupported lead major version {0} - only version 0 is supported")]
    InvalidLeadMinorVersion(u8),

    #[error("invalid type - expected 0 or 1 but got {0}")]
    InvalidLeadPKGType(u16),

    #[error("invalid os-type - expected 1 but got {0}")]
    InvalidLeadOSType(u16),

    #[error("invalid signature-type - expected 5 but got {0}")]
    InvalidLeadSignatureType(u16),

    #[error("invalid size of reserved area - expected length of {expected} but got {actual}")]
    InvalidReservedSpaceSize { expected: u16, actual: usize },
}
impl From<nom::Err<(&[u8], nom::error::ErrorKind)>> for RPMError {
    fn from(error: nom::Err<(&[u8], nom::error::ErrorKind)>) -> Self {
        match error {
            nom::Err::Error((_, kind)) | nom::Err::Failure((_, kind)) => {
                RPMError::Nom(kind.description().to_string())
            }
            nom::Err::Incomplete(_) => RPMError::Nom("unhandled incomplete".to_string()),
        }
    }
}
