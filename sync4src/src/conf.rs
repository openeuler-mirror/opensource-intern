use clap::Parser;
use config::Config;
use std::collections::HashMap;
use std::error::Error;

/// Commandline arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Config file path
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: String,
}

#[derive(Debug, Clone)]
pub enum Credential {
    SshKey(String),
    Password(String),
}

#[derive(Debug, Clone)]
pub struct Remote {
    pub url: String,
    pub username: Option<String>,
    pub cred: Option<Credential>,
}

#[derive(Debug, Clone)]
pub struct Conf {
    pub source: Remote, // source repository
    pub target: Remote, // target repository
}

/// parse the configuration and return `Conf`
pub fn parse_conf() -> Result<Conf, Box<dyn Error>> {
    let args: Args = Args::parse();
    let settings = Config::builder()
        .add_source(config::File::with_name(args.config.as_str()))
        .build()
        .unwrap();

    let source: HashMap<String, String> = settings.get("source")?;
    let source_url = source.get("url").ok_or("url field not found")?.to_string();
    let source_username = source.get("username").map(|x| x.to_string());
    let source_cred = if source.contains_key("ssh_key") {
        Some(Credential::SshKey(
            source.get("ssh_key").unwrap().to_string(),
        ))
    } else if source.contains_key("password") {
        let password = source.get("password").unwrap().to_string();
        Some(Credential::Password(password))
    } else {
        None
    };

    let target: HashMap<String, String> = settings.get("target")?;
    let target_url = target.get("url").ok_or("url field not found")?.to_string();
    let target_username = target.get("username").map(|x| x.to_string());
    let target_cred = if target.contains_key("ssh_key") {
        Some(Credential::SshKey(
            target.get("ssh_key").unwrap().to_string(),
        ))
    } else if target.contains_key("password") {
        let password = target.get("password").unwrap().to_string();
        Some(Credential::Password(password))
    } else {
        None
    };

    Ok(Conf {
        source: Remote {
            url: source_url,
            username: source_username,
            cred: source_cred,
        },
        target: Remote {
            url: target_url,
            username: target_username,
            cred: target_cred,
        },
    })
}
