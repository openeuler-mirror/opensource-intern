use super::headers::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RPMPackageMetadata {
    pub lead: Lead,
    pub signature: Header<IndexSignatureTag>,
    pub header: Header<IndexTag>,
}
impl RPMPackageMetadata {
    pub(crate) fn parse<T: std::io::BufRead>(input: &mut T) -> Result<Self, RPMError> {
        let mut lead_buffer = [0; LEAD_SIZE];
        input.read_exact(&mut lead_buffer)?;
        let lead = Lead::parse(&lead_buffer)?;
        let signature_header = Header::parse_signature(input)?;
        let header = Header::parse(input)?;
        Ok(RPMPackageMetadata {
            lead,
            signature: signature_header,
            header,
        })
    }
}