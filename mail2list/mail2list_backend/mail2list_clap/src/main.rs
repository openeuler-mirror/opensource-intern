extern crate rbatis;

use clap::{Parser, Subcommand};
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::crud_table;

#[crud_table]
#[derive(Clone, Debug)]
pub struct MailList {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub archive: Option<String>,
    pub description: Option<String>
}

impl Default for MailList {
    fn default() -> Self {
        Self {
            id: 0,
            name: None,
            email: None,
            archive: None,
            description: None,
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord,Subcommand)]
enum Commands {
    Add {
        #[clap(short, long)]
        url: String,
        #[clap(short, long)]
        id: i32,
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        email: String,
        #[clap(short, long)]
        archive: String,
        #[clap(short, long)]
        description: String,
    },

    Select {
        #[clap(short, long)]
        url: String,
    },

    Delete {
        #[clap(short, long)]
        url: String, #[clap(short, long)]
        #[clap(short, long)]
        id: i32,
    },
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { url ,id,name,email,description,archive} => {
            let rb = Rbatis::new();
            rb.link(url).await.unwrap();
            let mail_list = MailList {
                id:id.clone(),
                name: Some(name.to_string()),
                email: Some(email.to_string()),
                archive: Some(archive.to_string()),
                description: Some(description.to_string()),
            };

            let r = rb.save(&mail_list, &[]).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        }
        Commands::Select { url} => {
            let rb = Rbatis::new();
            rb.link(url).await.unwrap();
            let result: Vec<MailList> = rb.fetch_list().await.unwrap();
            for data in result {
                println!("{:?}\r", data);
            }
        }
        Commands::Delete { url,id} => {
            let rb = Rbatis::new();
            rb.link(url).await.unwrap();
            let mail_list = MailList {
                id:id.clone(),
                name: None,
                email: None,
                archive: None,
                description: None,
            };
            let r = rb.remove_by_column::<MailList, _>("id", &mail_list.id).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        }
    }
}
