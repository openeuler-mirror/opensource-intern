use specdiff::*;

pub fn get_address_list(toml_str: &str) -> Result<Vec<Address>, Box<dyn std::error::Error + Send + Sync>> {
    let decoded:Config = toml::from_str(toml_str).unwrap();
    Ok(decoded.addresses.unwrap())
}