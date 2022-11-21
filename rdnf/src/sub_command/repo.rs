use std::{ffi::CString, os::raw::c_void};

use anyhow::{bail, Result};
use config::Config;
use console::style;
use glob::Pattern;
use solv_sys::ffi::{pool_createwhatprovides, repo_create, Repo};

use crate::solv::sack::solv_create_cache_name;
use crate::solv::sack::Solvsack;
use crate::{
    cli::Commands,
    conf::ConfigMain,
    default::{VAR_BASEARCH, VAR_RELEASEVER},
    errors::ERROR_RDNF_INVALID_PARAMETER,
    i18n::repo_list::{
        REPOLIST_REPO_ID, REPOLIST_REPO_NAME, REPOLIST_REPO_STATUS, REPOLIST_REPO_STATUS_DISABLED,
        REPOLIST_REPO_STATUS_ENABLED,
    },
    solv::SolvRepoInfoInernal,
    utils::check_dir,
    Cli, Rdnf,
};

const CMDLINE_REPO_NAME: &str = "@cmdline";
pub enum RepoListFilter {
    All,
    Enabled,
    Disabled,
}

impl Rdnf {
    pub fn repo_list(self) -> Result<()> {
        match self.rc.cli.command {
            Commands::Repolist(opt) => {
                let sum = opt.all as usize + opt.enabled as usize + opt.disabled as usize;
                if sum > 1 {
                    bail!("you can only choose one of three options (all,enabled,disabled)")
                };
                let filter = match opt.all {
                    true => RepoListFilter::All,
                    false => match opt.enabled {
                        true => RepoListFilter::Enabled,
                        false => match opt.disabled {
                            true => RepoListFilter::Disabled,
                            false => RepoListFilter::Enabled,
                        },
                    },
                };
                let (_heigt, width) = self.rc.term.size();
                let l = (width as f32 * 0.3) as usize;
                let c = (width as f32 * 0.5) as usize;
                let r = width as usize - l - c;
                let title = format!(
                    "{:<left$}{:<center$}{:<rest$}",
                    REPOLIST_REPO_ID,
                    REPOLIST_REPO_NAME,
                    REPOLIST_REPO_STATUS,
                    left = l,
                    center = c,
                    rest = r
                );
                self.rc.term.write_line(title.as_str())?;
                for repo in self.repos {
                    let mut is_show = match filter {
                        RepoListFilter::All => true,
                        RepoListFilter::Enabled => repo.base.enabled,
                        RepoListFilter::Disabled => !repo.base.enabled,
                    };
                    if repo.psz_name == CMDLINE_REPO_NAME {
                        is_show = false;
                    }
                    if is_show {
                        let status = match repo.base.enabled {
                            true => {
                                format!("{}", style(REPOLIST_REPO_STATUS_ENABLED).green())
                            }
                            false => {
                                format!("{}", style(REPOLIST_REPO_STATUS_DISABLED).red())
                            }
                        };
                        let item = format!(
                            "{:<left$}{:<center$}{:<rest$}",
                            repo.psz_id.as_str(),
                            repo.psz_name.as_str(),
                            status.as_str(),
                            left = l,
                            center = c,
                            rest = r
                        );
                        self.rc.term.write_line(item.as_str())?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
pub fn load_repo_data(config: &ConfigMain, filter: RepoListFilter) -> Result<Vec<RepoData>> {
    let repo_cmdline = RepoData::create_repo(CMDLINE_REPO_NAME);
    let mut repodatas: Vec<RepoData> = Vec::new();
    repodatas.push(repo_cmdline);
    let dir = config.repodir.as_str();
    check_dir(dir)?;
    match glob::glob((dir.to_string() + "*.repo").as_str()) {
        Ok(paths) => {
            match Config::builder()
                .add_source(
                    paths
                        .map(|p| config::File::from(p.unwrap()).format(config::FileFormat::Ini))
                        .collect::<Vec<_>>(),
                )
                .build()
            {
                Ok(c) => {
                    for (psz_id, value) in c
                        .try_deserialize::<config::Map<String, config::Value>>()
                        .unwrap()
                    {
                        let mut repo = RepoData::create_repo(psz_id.as_str());
                        for (key, v) in value.into_table().unwrap() {
                            repo.update_repo(key.as_str(), v)?
                        }
                        match filter {
                            RepoListFilter::All => repodatas.push(repo),
                            RepoListFilter::Enabled => {
                                if repo.base.enabled {
                                    repodatas.push(repo)
                                }
                            }
                            RepoListFilter::Disabled => {
                                if !repo.base.enabled {
                                    repodatas.push(repo)
                                }
                            }
                        };
                    }
                }
                Err(_) => {
                    bail!("Failed to parse config files .repo dir {}", dir)
                }
            };
        }
        Err(_) => {
            bail!("Failed to search .repo files in dir {}", dir)
        }
    };
    Ok(repodatas)
}
pub fn repo_list_finalize(
    cli: &mut Cli,
    config: &ConfigMain,
    repos: &mut Vec<RepoData>,
) -> Result<()> {
    match &cli.repoid {
        Some(v) => {
            alter_repo_state_enable(repos, false, "*")?;
            for pattern in v {
                alter_repo_state_enable(repos, true, pattern)?;
            }
        }
        None => {}
    }
    match &cli.enablerepo {
        Some(v) => {
            for pattern in v {
                alter_repo_state_enable(repos, true, pattern)?;
            }
        }
        None => {}
    }
    match &cli.disablerepo {
        Some(v) => {
            for pattern in v {
                alter_repo_state_enable(repos, false, pattern)?;
            }
        }
        None => {}
    }
    for repo in repos {
        repo.psz_name = repo
            .psz_name
            .replace(VAR_RELEASEVER, &config.var_release_ver)
            .replace(VAR_BASEARCH, &config.var_base_arch);
        match &repo.details.base_url {
            Some(s) => {
                repo.details.base_url = Some(
                    s.replace(VAR_RELEASEVER, &config.var_release_ver)
                        .replace(VAR_BASEARCH, &config.var_base_arch),
                );
            }
            None => {}
        }
        match &repo.details.meta_link {
            Some(s) => {
                let meta_link = s
                    .replace(VAR_RELEASEVER, &config.var_release_ver)
                    .replace(VAR_BASEARCH, &config.var_base_arch);
                repo.details.cache_name = Some(solv_create_cache_name(&repo.psz_id, &meta_link)?);
                repo.details.meta_link = Some(meta_link);
            }
            None => match &repo.details.base_url {
                Some(s) => {
                    repo.details.cache_name = Some(solv_create_cache_name(&repo.psz_id, s)?);
                }
                None => {}
            },
        }
        match &repo.details.url_gpg_keys {
            Some(s) => {
                let mut url_gpg_keys=Vec::new();
                for ele in s {
                    let m=ele.replace(VAR_RELEASEVER, &config.var_release_ver)
                    .replace(VAR_BASEARCH, &config.var_base_arch);
                    url_gpg_keys.push(m);
                }
                repo.details.url_gpg_keys=Some(url_gpg_keys);
            },
            None => {},
        }
    }
    Ok(())
}
pub fn init_cmdline_repo(sack: &mut Solvsack) -> Result<*mut Repo> {
    unsafe {
        let repo_name = CString::new(CMDLINE_REPO_NAME).unwrap();
        let repo = repo_create(sack.pool, repo_name.as_ptr());
        if repo.is_null() {
            bail!(ERROR_RDNF_INVALID_PARAMETER);
        }
        let solv_repo_info = SolvRepoInfoInernal {
            repo,
            cookie: None,
            n_cookie_set: None,
            repo_cache_dir: None,
        };
        let p = Box::into_raw(Box::new(solv_repo_info)) as *mut c_void;
        (*repo).appdata = p;
        pool_createwhatprovides(sack.pool);
        Ok(repo)
    }
}
fn alter_repo_state_enable(repos: &mut Vec<RepoData>, enable: bool, pattern: &str) -> Result<()> {
    match Pattern::new(pattern) {
        Ok(p) => {
            for repo in &mut *repos {
                if p.matches(&repo.psz_id) {
                    repo.base.enabled = enable;
                }
            }
        }
        Err(e) => {
            bail!("Failed to enablerepo {}, because {}", &pattern, e)
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct RepoDataBase {
    pub enabled: bool,
    pub skip_if_unavailable: bool,
    pub gpgcheck: bool,
    pub priority: i32,
    pub timeout: u64,
    pub retries: i32,
    pub minrate: u32,
    pub throttle: u64,
    pub sslverify: bool,
    pub lmetadata_expire: i128,
    pub skip_md_filelists: bool,
    pub skip_md_updateinfo: bool,
    pub skip_md_other: bool,
}
impl RepoDataBase {
    pub fn default() -> Self {
        RepoDataBase {
            enabled: true,
            skip_if_unavailable: false,
            gpgcheck: true,
            sslverify: true,
            lmetadata_expire: 172800,
            priority: 50,
            timeout: 0,
            minrate: 0,
            throttle: 0,
            retries: 10,
            skip_md_filelists: false,
            skip_md_updateinfo: false,
            skip_md_other: false,
        }
    }
}
#[derive(Debug, Clone)]
pub struct RepoDataDetails {
    pub base_url: Option<String>,
    pub meta_link: Option<String>,
    pub url_gpg_keys: Option<Vec<String>>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub ssl_ca_cert: Option<String>,
    pub ssl_client_cert: Option<String>,
    pub ssl_client_key: Option<String>,
    pub cache_name: Option<String>,
}
impl RepoDataDetails {
    pub fn new() -> Self {
        RepoDataDetails {
            cache_name: None,
            base_url: None,
            meta_link: None,
            url_gpg_keys: None,
            username: None,
            password: None,
            ssl_ca_cert: None,
            ssl_client_cert: None,
            ssl_client_key: None,
        }
    }
}
#[derive(Debug, Clone)]
pub struct RepoData {
    pub base: RepoDataBase,
    pub psz_id: String,
    pub psz_name: String,
    pub details: RepoDataDetails,
}
impl RepoData {
    pub fn create_repo(psz_id: &str) -> Self {
        RepoData {
            base: RepoDataBase::default(),
            psz_id: String::from(psz_id),
            psz_name: String::from(psz_id),
            details: RepoDataDetails::new(),
        }
    }
    pub fn update_repo(self: &mut Self, key: &str, v: config::Value) -> Result<()> {
        match key {
            "enabled" => match v.into_bool() {
                Ok(b) => self.base.enabled = b,
                Err(_) => {
                    bail!("repo {} enabled should be 0 or 1", self.psz_id)
                }
            },
            "name" => match v.into_string() {
                Ok(s) => self.psz_name = s,
                Err(_) => {
                    bail!("repo {} name should be String", self.psz_id)
                }
            },
            "baseurl" => match v.into_string() {
                Ok(s) => self.details.base_url = Some(s),
                Err(_) => {
                    bail!("repo {} baseurl should be String", self.psz_id)
                }
            },
            "metalink" => match v.into_string() {
                Ok(s) => self.details.meta_link = Some(s),
                Err(_) => {
                    bail!("repo {} metalink should be String", self.psz_id)
                }
            },
            "skip_if_unavailable" => match v.into_bool() {
                Ok(b) => self.base.skip_if_unavailable = b,
                Err(_) => {
                    bail!("repo {} skip_if_unavailable should be 0 or 1", self.psz_id)
                }
            },
            "gpgcheck" => match v.into_bool() {
                Ok(b) => self.base.gpgcheck = b,
                Err(_) => {
                    bail!("repo {} gpgcheck should be 0 or 1", self.psz_id)
                }
            },
            "gpgkey" => match v.into_string() {
                Ok(s) => {
                    let mut gpg_keys: Vec<String> = Vec::new();
                    for ele in s.split(" ").collect::<Vec<&str>>() {
                        gpg_keys.push(String::from(ele));
                    }
                    self.details.url_gpg_keys = Some(gpg_keys);
                }
                Err(_) => {
                    bail!(
                        "repo {} gpgkey should be string array split by ' ' ",
                        self.psz_id
                    )
                }
            },
            "username" => match v.into_string() {
                Ok(s) => self.details.username = Some(s),
                Err(_) => {
                    bail!("repo {} username should be string", self.psz_id)
                }
            },
            "password" => match v.into_string() {
                Ok(s) => self.details.password = Some(s),
                Err(_) => {
                    bail!("repo {} password should be string", self.psz_id)
                }
            },
            "priority" => match v.into_int() {
                Ok(i) => self.base.priority = i as i32,
                Err(_) => {
                    bail!("repo {} priority should be int32", self.psz_id)
                }
            },
            "timeout" => match v.into_int() {
                Ok(i) => self.base.timeout = i as u64,
                Err(_) => {
                    bail!("repo {} timeout should be int32", self.psz_id)
                }
            },
            "retries" => match v.into_int() {
                Ok(i) => self.base.retries = i as i32,
                Err(_) => {
                    bail!("repo {} retries should be int32", self.psz_id)
                }
            },
            "minrate" => match v.into_int() {
                Ok(i) => self.base.minrate = i as u32,
                Err(_) => {
                    bail!("repo {} minrate should be int32", self.psz_id)
                }
            },
            "throttle" => match v.into_int() {
                Ok(i) => self.base.throttle = i as u64,
                Err(_) => {
                    bail!("repo {} throttle should be int32", self.psz_id)
                }
            },
            "sslverify" => match v.into_bool() {
                Ok(b) => self.base.sslverify = b,
                Err(_) => {
                    bail!("repo {} sslverify should be 0 or 1", self.psz_id)
                }
            },
            "sslcacert" => match v.into_string() {
                Ok(s) => self.details.ssl_ca_cert = Some(s),
                Err(_) => {
                    bail!("repo {} sslcacert should be string", self.psz_id)
                }
            },
            "sslclientcert" => match v.into_string() {
                Ok(s) => self.details.ssl_client_cert = Some(s),
                Err(_) => {
                    bail!("repo {} sslckuebtcert should be string", self.psz_id)
                }
            },
            "sslclientkey" => match v.into_string() {
                Ok(s) => self.details.ssl_client_key = Some(s),
                Err(_) => {
                    bail!("repo {} sslclientkey should be string", self.psz_id)
                }
            },
            "metadata_expire" => match v.into_string() {
                Ok(s) => {
                    if s == "never" {
                        self.base.lmetadata_expire = -1;
                    } else {
                        self.base.lmetadata_expire = match s.parse::<i128>() {
                            Ok(t) => t,
                            Err(_) => {
                                let (num, mul) = s.split_at(s.len() - 1);
                                let n = match num.parse::<i128>() {
                                    Ok(n) => n,
                                    Err(_) => {
                                        bail!("repo {} metadata_expire should be like 1 or 1d or 1h or 1m or 1s",self.psz_id)
                                    }
                                };
                                match mul {
                                    "s" => n,
                                    "m" => 60 * n,
                                    "h" => 60 * 60 * n,
                                    "d" => 60 * 60 * 24 * n,
                                    _ => {
                                        bail!("repo {} metadata_expire the unit of time should be d,h,m,s(default)",self.psz_id)
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    bail!(
                        "repo {} metadata_expire can't be parse to string ",
                        self.psz_id
                    )
                }
            },
            "skip_md_filelists" => match v.into_bool() {
                Ok(b) => self.base.skip_md_filelists = b,
                Err(_) => {
                    bail!("repo {} skip_md_filelists should be 0 or 1", self.psz_id)
                }
            },
            "skip_md_updateinfo" => match v.into_bool() {
                Ok(b) => self.base.skip_md_updateinfo = b,
                Err(_) => {
                    bail!("repo {} skip_md_updateinfo should be 0 or 1", self.psz_id)
                }
            },
            "skip_md_other" => match v.into_bool() {
                Ok(b) => self.base.skip_md_other = b,
                Err(_) => {
                    bail!("repo {} skip_md_other should be 0 or 1", self.psz_id)
                }
            },
            _ => {}
        }
        Ok(())
    }
}
