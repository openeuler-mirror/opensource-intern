
use specdiff::*;

#[tokio::main]
async fn main() -> Result<(), SpecError> {
    let args = Cli::parse();
    let addresses: Vec<Address> = Cli::get_address_list_from_cli().await?;
    let (spec_save_path, out_terminal, report_out_path) = (args.spec_save_path, args.terminal_out, args.report_out_path);
    let mut out = true;
    let mut diff_ratio_list: Vec<f32> = Vec::new();
    let mut stdout = io::stdout();

    let spec_path: String = match spec_save_path {
        Some(path) => path,
        None => "/tmp/specdiff/download/".to_string(),
    };

    let report_path: String = match report_out_path {
        Some(path) => path,
        None => ".".to_string(),
    };

    if let Some(false) = out_terminal {
        out = false;
    } 

    fs::create_dir_all(&spec_path)?;
    fs::create_dir_all(&report_path)?;
    for address in addresses {
        get_diff_from_address(address, &spec_path, &out, &report_path, &mut diff_ratio_list, &mut stdout).await?;
    }

    let avg_ratio:f32 = diff_ratio_list.iter().sum::<f32>() / diff_ratio_list.len() as f32;
    println!("The avg_ratio is: {:.2}%", avg_ratio*100.0);
    
    Ok(())
}