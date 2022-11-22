use std::{
    ffi::{CStr, CString},
    os::raw::c_void,
};

use anyhow::{bail, Result};
use config::{Config, ConfigError, File, FileFormat};
use libc::utsname;
use serde::Deserialize;

use rpm_sys::ffi::{
    headerGetString, headerLink, rpmTag_e_RPMTAG_VERSION, rpmdbNextIterator, rpmtsInitIterator,
};

use crate::{
    cli::Cli,
    default::{
        DEFAULT_CACHE_LOCATION, DEFAULT_DISTROVERPKG, DEFAULT_PLUGIN_CONF_PATH,
        DEFAULT_PLUGIN_PATH, DEFAULT_REPO_LOCATION,
    },
    errors::{
        ERROR_RDNF_DISTROVERPKG_READ, ERROR_RDNF_NO_DISTROVERPKG, ERROR_RDNF_RPMTS_CREATE_FAILED,
    },
    utils::check_dir,
};

#[derive(Debug, Deserialize, Clone)]
struct Main {
    installonly_limit: Option<usize>,
    clean_requirements_on_remove: Option<bool>,
    gpgcheck: Option<bool>,
    keepcache: Option<bool>,
    repodir: Option<String>,
    cachedir: Option<String>,
    proxy: Option<String>,
    proxy_username: Option<String>,
    proxy_password: Option<String>,
    distroverpkg: Option<String>,
    excludepkgs: Option<Vec<String>>,
    minversions: Option<Vec<String>>,
    plugins: Option<bool>,
    pluginconfpath: Option<String>,
    pluginpath: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configfile {
    main: Main,
}
#[derive(Debug, Clone)]
pub struct ConfigMain {
    pub installonly_limit: usize,
    pub clean_requirements_on_remove: bool,
    pub gpgcheck: bool,
    pub keepcache: bool,
    pub repodir: String,
    pub cachedir: String,
    pub distroverpkg: String,
    pub excludepkgs: Option<Vec<String>>,
    pub minversions: Option<Vec<String>>,
    pub proxy: Option<String>,
    pub proxy_username: Option<String>,
    pub proxy_password: Option<String>,
    pub plugins: bool,
    pub pluginconfpath: String,
    pub pluginpath: String,
    pub var_release_ver: String,
    pub var_base_arch: String,
}
impl ConfigMain {
    pub fn from(cli: &mut Cli) -> Result<Self> {
        let main = match Config::builder()
            .add_source(File::new(&cli.config_file, FileFormat::Ini))
            .build()
        {
            Ok(s) => {
                let c: Result<Configfile, ConfigError> = s.try_deserialize();
                match c {
                    Ok(t) => {
                        let m = t.main;
                        let pkg = m.distroverpkg.unwrap_or(String::from(DEFAULT_DISTROVERPKG));
                        ConfigMain {
                            gpgcheck: m.gpgcheck.unwrap_or(false),
                            installonly_limit: m.installonly_limit.unwrap_or(1),
                            clean_requirements_on_remove: m
                                .clean_requirements_on_remove
                                .unwrap_or(false),
                            keepcache: m.keepcache.unwrap_or(false),
                            repodir: {
                                match &cli.reposdir {
                                    Some(s) => s.to_string(),
                                    None => {
                                        cli.installroot.to_string()
                                            + m.repodir
                                                .unwrap_or(String::from(DEFAULT_REPO_LOCATION))
                                                .trim_start_matches("/")
                                                .trim_end_matches("/")
                                            + "/"
                                    }
                                }
                            },
                            cachedir: cli.installroot.to_string()
                                + m.cachedir
                                    .unwrap_or(String::from(DEFAULT_CACHE_LOCATION))
                                    .trim_start_matches("/")
                                    .trim_end_matches("/")
                                + "/",
                            distroverpkg: pkg.clone(),
                            excludepkgs: m.excludepkgs,
                            minversions: m.minversions,
                            proxy: m.proxy,
                            proxy_username: m.proxy_username,
                            proxy_password: m.proxy_password,
                            plugins: if cli.plugins {
                                true
                            } else {
                                if m.plugins.unwrap_or(false) {
                                    cli.plugins = true;
                                    true
                                } else {
                                    false
                                }
                            },
                            pluginconfpath: m
                                .pluginconfpath
                                .unwrap_or(String::from(DEFAULT_PLUGIN_CONF_PATH)),
                            pluginpath: m.pluginpath.unwrap_or(String::from(DEFAULT_PLUGIN_PATH)),
                            var_release_ver: { ConfigMain::get_package_version(pkg, cli)? },
                            var_base_arch: { ConfigMain::get_kernel_arch()? },
                        }
                    }
                    Err(_) => {
                        bail!("Failed to parse config file: {}", cli.config_file)
                    }
                }
            }
            Err(_) => {
                bail!("Failed to read config file: {}", cli.config_file)
            }
        };
        if !check_dir(main.repodir.as_str())? {
            bail!("Dir repodir {} don't have .repo file", main.repodir);
        };
        Ok(main)
    }
    pub fn get_package_version(pkg: String, cli: &mut Cli) -> Result<String> {
        let ver = match cli.releasever.clone() {
            Some(ver) => ver,
            None => unsafe {
                let p_ts = rpm_sys::ffi::rpmtsCreate();
                if p_ts.is_null() {
                    bail!(ERROR_RDNF_RPMTS_CREATE_FAILED);
                }
                let root_ptr = CString::new(cli.installroot.clone())
                    .unwrap_or(CString::new("/").unwrap());
                if rpm_sys::ffi::rpmtsSetRootDir(p_ts, root_ptr.as_ptr()) != 0 {
                    bail!("Failed to set root dir {} for rpmts", cli.installroot);
                };
                let pkg_t = CString::new(pkg.as_str()).unwrap();
                let t = pkg_t.as_ptr() as *const c_void;
                let p_iter = rpmtsInitIterator(p_ts, 1047, t, 0);
                if p_iter.is_null() {
                    bail!(ERROR_RDNF_NO_DISTROVERPKG)
                };
                let p_header = rpmdbNextIterator(p_iter);
                if p_header.is_null() {
                    bail!(ERROR_RDNF_DISTROVERPKG_READ)
                };
                let p_header = headerLink(p_header);
                if p_header.is_null() {
                    bail!(ERROR_RDNF_DISTROVERPKG_READ)
                };
                let psz_version_temp = headerGetString(p_header, rpmTag_e_RPMTAG_VERSION);
                if psz_version_temp.is_null() {
                    bail!(ERROR_RDNF_DISTROVERPKG_READ)
                };
                let version = CStr::from_ptr(psz_version_temp).to_str().unwrap();
                String::from(version)
            },
        };
        Ok(ver)
    }
    pub fn get_kernel_arch() -> Result<String> {
        let c = [0; 65];
        let mut system_info = utsname {
            sysname: c,
            nodename: c,
            release: c,
            version: c,
            machine: c,
            domainname: c,
        };
        unsafe {
            if libc::uname(&mut system_info) != 0 {
                bail!("Failed to uname");
            };
            let arch = CStr::from_ptr(system_info.machine.as_ptr())
                .to_str()
                .unwrap()
                .to_string();
            Ok(arch)
        }
    }
}
