
use specdiff::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let addresses: Vec<Address> = Cli::get_address_list_from_cli().await?;
    let mut specs_list: HashMap<String, Vec<String>> = HashMap::new();
    for address in addresses {
        let f1 = reqwest::get(address.x);
        let f2 = reqwest::get(address.y);
        let (res1, res2) = try_join!(f1, f2)?;
        let body = vec![res1.text().await?, res2.text().await?];
        specs_list.insert(address.name, body);
    }

    for (name, specs) in specs_list {
        get_diff(name, specs).await?;
    }
    
    Ok(())
}