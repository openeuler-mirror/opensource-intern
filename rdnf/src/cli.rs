use anyhow::bail;
use anyhow::Result;
use clap::arg;
use clap::command;
use clap::Args;
use clap::Parser;
use clap::Subcommand;

use crate::default::RDNF_CONF_FILE;
use crate::errors::ERROR_RDNF_RPM_INIT;
use clap::ArgAction::Append;
use clap::ArgAction::SetTrue;
#[derive(Parser, Debug, Clone)]
#[command(name = "rdnf", version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(default_value_t=String::from("/"),long,value_parser)]
    pub installroot: String,
    #[arg(default_value_t=String::from(RDNF_CONF_FILE),short='c',long,value_parser)]
    pub config_file: String,
    #[arg(long,action=SetTrue)]
    pub plugins: bool,
    ///download all dependencies even if already installed
    #[arg(long,action=SetTrue)]
    pub alldeps: bool,
    ///enable repoid,don't enable or disable other repoids
    #[arg(long,value_name="repoid",action=Append)]
    pub enablerepo: Option<Vec<String>>,
    ///disable repoid,don't disable or enable other repoids
    #[arg(long,value_name="repoid",action=Append)]
    pub disablerepo: Option<Vec<String>>,
    ///enable repoids and disable other repoids
    #[arg(long,value_name="repoid",action=Append)]
    pub repoid: Option<Vec<String>>,
    #[arg(long, value_parser)]
    pub releasever: Option<String>,
    #[arg(long, value_parser)]
    pub reposdir: Option<String>,
    #[arg(long,action=SetTrue)]
    pub cacheonly: bool,
    #[arg(long,action=SetTrue)]
    pub refresh: bool,
    #[arg(long,action=SetTrue)]
    pub security: bool,
    #[arg(long, value_name = "severity")]
    pub sec_severity: Option<String>,
    ///set to rpm verbosity level (emergency,alert,critical,error,warning,notice,info,debug)
    #[arg(default_value_t=String::from("error"),long,value_name="debug level name")]
    pub rpm_verbosity: String,
    #[arg(long,action=SetTrue)]
    pub reboot_required: bool,
    #[arg(long,action=SetTrue)]
    pub disable_excludes: bool,
    #[arg(long,value_name="pkg",action=Append)]
    pub exclude: Option<Vec<String>>,
}
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Repolist(RepolistOption),
    Makecache,
    Search { pkgs: Vec<String> },
    Remove(AlterOption),
    Install(AlterOption),
    Reinstall(AlterOption),
    Update(AlterOption),
    Info(InfoOption),
}
#[derive(Args, Clone, Debug, Copy)]
pub struct RepolistOption {
    #[arg(long,action=clap::ArgAction::SetTrue)]
    pub all: bool,
    #[arg(long,action=clap::ArgAction::SetTrue)]
    pub enabled: bool,
    #[arg(long,action=clap::ArgAction::SetTrue)]
    pub disabled: bool,
}
#[derive(Args, Clone, Debug)]
pub struct AlterOption {
    pub pkgs: Vec<String>,
    ///allow erasures when solving
    #[arg(long,action=SetTrue)]
    pub allow_erasing: bool,
    ///assume no for all questions
    #[arg(long,action=SetTrue)]
    pub assume_no: bool,
    ///assume yes for all questions
    #[arg(long,action=SetTrue)]
    pub assume_yes: bool,
    ///download packages only, no install
    #[arg(long,action=SetTrue)]
    pub download_only: bool,
    #[arg(long,action=SetTrue)]
    pub quiet: bool,
    ///resolve packages to latest version
    #[arg(long,action=SetTrue)]
    pub best: bool,
    ///dump solv debug info
    #[arg(long,action=SetTrue)]
    pub debug_solver: bool,
    ///overide clean_requirements_on_remove config option
    #[arg(long,action=SetTrue)]
    pub no_auto_remove: bool,
    ///skip gpg check
    #[arg(long,action=SetTrue)]
    pub no_gpg_check: bool,
    ///skip conflict problems
    #[arg(long,action=SetTrue)]
    pub skip_confilicts: bool,
    ///skip verifying RPM digest
    #[arg(long,action=SetTrue)]
    pub skip_digest: bool,
    ///skip obsolete problems
    #[arg(long,action=SetTrue)]
    pub skip_obsolete: bool,
    ///skip verifying RPM signatures
    #[arg(long,action=SetTrue)]
    pub skip_signatures: bool,
    #[arg(long,action=SetTrue)]
    pub tsflags_noscripts: bool,
}
#[derive(Args, Clone, Debug)]
pub struct InfoOption{
    pub pkgs:Vec<String>,
    #[arg(long,action=SetTrue)]
    pub all: bool,
    #[arg(long,action=SetTrue)]
    pub installed: bool,
    #[arg(long,action=SetTrue)]
    pub available: bool,
    #[arg(long,action=SetTrue)]
    pub extras:bool,
    #[arg(long,action=SetTrue)]
    pub obsoletes:bool,
    #[arg(long,action=SetTrue)]
    pub recent:bool,
    #[arg(long,action=SetTrue)]
    pub upgrades:bool,
    #[arg(long,action=SetTrue)]
    pub updates:bool,
    #[arg(long,action=SetTrue)]
    pub dwongrades:bool,
}
impl Cli {
    pub fn init(self) -> Result<Self> {
        let cli = self.check_fs_path();
        Ok(cli)
    }
    pub fn check_fs_path(mut self) -> Self {
        self.installroot = self.installroot.trim_end_matches("/").to_string() + "/";
        self.config_file = self.installroot.clone() + self.config_file.trim_start_matches("/");
        match self.reposdir {
            Some(s) => {
                self.reposdir = Some(self.installroot.clone() + s.trim_start_matches("/"));
            }
            None => {}
        }
        self
    }
}
pub fn rpm_init() -> Result<()> {
    unsafe {
        if rpm_sys::ffi::rpmReadConfigFiles(0 as *mut i8, 0 as *mut i8) != 0 {
            bail!(ERROR_RDNF_RPM_INIT);
        };
    }
    Ok(())
}
