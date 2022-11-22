use anyhow::Result;
use clap::Parser;
use console::Term;
use indicatif::MultiProgress;
use solv_sys::ffi::Repo;

use cli::{rpm_init, Cli, Commands};
use conf::ConfigMain;

use solv::sack::Solvsack;
use sub_command::{
    install::AlterType,
    repo::{init_cmdline_repo, load_repo_data, repo_list_finalize, RepoData, RepoListFilter},
};
use utils::is_already_running;

use crate::utils::check_root;

mod sub_command;

mod c_lib;
mod cli;
mod conf;
mod default;
mod errors;
mod goal;
mod gpgcheck;
mod i18n;
mod lock;
mod metalink;
mod output;
mod pkgutils;
mod repomd;
mod rpm_trans;
mod solv;
mod utils;
#[derive(Debug, Clone)]
pub struct Rdnf {
    rc: RdnfContext,
    repos: Vec<RepoData>,
    solv_cmdline_repo: *mut Repo,
}
#[derive(Debug, Clone)]
pub struct RdnfContext {
    sack: Solvsack,
    cli: Cli,
    conf: ConfigMain,
    multi_process: MultiProgress,
    term: Term,
}
impl Rdnf {
    pub fn new() -> Result<Self> {
        is_already_running()?;
        rpm_init()?;
        let mut cli = Cli::parse().init()?;
        let conf = ConfigMain::from(&mut cli)?;
        let mut sack = Solvsack::from(&conf, &cli)?;
        let mut repos = load_repo_data(&conf, RepoListFilter::All)?;
        repos.sort_by(|a, b| a.base.priority.cmp(&(b.base.priority)));
        repos.sort_by(|a, b| a.psz_id.cmp(&b.psz_id));
        repo_list_finalize(&mut cli, &conf, &mut repos)?;
        let solv_cmdline_repo = init_cmdline_repo(&mut sack)?;
        let term = Term::stdout();
        let multi_process = MultiProgress::new();
        let rc = RdnfContext {
            sack,
            cli,
            conf,
            multi_process,
            term,
        };
        let rdnf = Rdnf {
            rc,
            repos,
            solv_cmdline_repo,
        };
        Ok(rdnf)
    }
    pub fn refresh_sack() -> Result<()> {
        Ok(())
    }
}

impl AsRef<Rdnf> for Rdnf {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}
// #[tokio::main(flavor = "current_thread")]
fn main() -> Result<()> {
    let mut rdnf = Rdnf::new()?;
    match &rdnf.rc.cli.command {
        Commands::Repolist(_) => {
            rdnf.repo_list()?;
        }
        Commands::Makecache => {
            rdnf.rc.cli.refresh = true;
            rdnf.make_cache()?;
        }
        Commands::Search { pkgs } => {
            let pkgs = pkgs.clone();
            rdnf.search_pkg(pkgs)?;
        }
        Commands::Install(alter) => {
            check_root()?;
            let pkgs = alter.pkgs.clone();
            let alter = alter.clone();
            rdnf.alter_command(pkgs, AlterType::Install, &alter)?;
        }
        Commands::Remove(alter)=>{
            check_root()?;
            let pkgs=alter.pkgs.clone();
            let alter=alter.clone();
            rdnf.alter_command(pkgs, AlterType::Erase, &alter)?;
        }
        Commands::Reinstall(alter) => {
            check_root()?;
            let pkgs=alter.pkgs.clone();
            let alter=alter.clone();
            rdnf.alter_command(pkgs, AlterType::ReInstall, &alter)?;

        },
        Commands::Update(alter) => {
            check_root()?;
            let pkgs = alter.pkgs.clone();
            let alter=alter.clone();
            if pkgs.is_empty(){
                rdnf.alter_command(pkgs, AlterType::UpgradeAll, &alter)?;
            }else{
                rdnf.alter_command(pkgs, AlterType::Upgrade, &alter)?;
            }
        },
        Commands::Info(info_opt) => {
            rdnf.info_command(info_opt.clone())?;

        },
       
    }
    Ok(())
}
