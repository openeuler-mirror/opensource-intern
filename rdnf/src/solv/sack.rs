use std::{
    ffi::{c_uint, CStr, CString},
    io::Read,
    mem::size_of,
    os::raw::c_void,
};

use anyhow::{bail, Result};
use libc::{snprintf, fclose};
use solv_sys::ffi::{
    pool_create, pool_createwhatprovides, pool_set_flag, pool_set_installed, pool_set_rootdir,
    pool_setarch, repo_create, s_Pool, solv_chksum_add, solv_chksum_create, solv_chksum_free,
    solv_knownid_REPOKEY_TYPE_SHA256, Repo, POOL_FLAG_ADDFILEPROVIDESFILTERED, _IO_FILE, REPO_REUSE_REPODATA, RPM_ADD_WITH_HDRID, REPO_USE_ROOTDIR, repo_add_rpmdb_reffp, 
};

use crate::{
    conf::ConfigMain,
    default::{SOLV_COOKIE_IDENT, SOLV_COOKIE_LEN},
    errors::{ERROR_RDNF_INVALID_PARAMETER, ERROR_RDNF_SOLV_CHKSUM},
    Cli,
};

const SYSTEM_REPO_NAME: &str = "@System";
#[derive(Debug, Clone)]
pub struct Solvsack {
    pub pool: *mut s_Pool,
    pub dw_num_of_command_pkgs: usize,
    pub cachedir: String,
    pub rootdir: String,
}
unsafe impl Send for Solvsack {}
impl Solvsack {
    pub fn from(conf: &ConfigMain, cli: &Cli,) -> Result<Self> {
        unsafe {
            let pool = pool_create();
            if pool.is_null() {
                bail!(ERROR_RDNF_INVALID_PARAMETER);
            }
            let root = CString::new(cli.installroot.as_str()).unwrap_or(CString::new("/").unwrap());
            pool_set_rootdir(pool, root.as_ptr());
            let evr = CString::new(conf.var_base_arch.as_str()).unwrap();
            pool_setarch(pool, evr.as_ptr());
            pool_set_flag(pool, POOL_FLAG_ADDFILEPROVIDESFILTERED as i32, 1);
            let system_repo = CString::new(SYSTEM_REPO_NAME).unwrap();
            let repo: *mut Repo = repo_create(pool, system_repo.as_ptr());
            if !cli.alldeps {
                let cache_dir = CString::new(conf.cachedir.as_str()).unwrap();
                let mode = CString::new("r").unwrap();
                let p_cache_file = libc::fopen(cache_dir.as_ptr(), mode.as_ptr());
                let dw_flags=REPO_REUSE_REPODATA | RPM_ADD_WITH_HDRID | REPO_USE_ROOTDIR;
                if repo_add_rpmdb_reffp(repo, p_cache_file as *mut _IO_FILE, dw_flags as i32) != 0 {
                    bail!("Failed to init solvack,can't open rpmdb");
                };
                if !p_cache_file.is_null() {
                    fclose(p_cache_file);
                }
            }
            if repo.is_null() {
                bail!(ERROR_RDNF_INVALID_PARAMETER);
            }
            pool_set_installed(pool, repo);
            pool_createwhatprovides(pool);
            Ok(Solvsack {
                pool,
                dw_num_of_command_pkgs: 0,
                cachedir: conf.cachedir.clone(),
                rootdir: cli.installroot.clone(),
            })
        }
    }
}
pub fn solv_create_cache_name(name: &String, url: &String) -> Result<String> {
    unsafe {
        let p_chk_sum = solv_chksum_create(solv_knownid_REPOKEY_TYPE_SHA256.try_into().unwrap());
        if p_chk_sum.is_null() {
            bail!(ERROR_RDNF_SOLV_CHKSUM);
        }
        let psz_url = CString::new(url.as_str()).unwrap();
        solv_chksum_add(
            p_chk_sum,
            psz_url.as_ptr() as *const c_void,
            libc::strlen(psz_url.as_ptr()) as i32,
        );
        let mut p_cookie: [u8; SOLV_COOKIE_LEN] = [0; SOLV_COOKIE_LEN];
        let mut psz_cookie: [u8; 9] = [0; 9];
        solv_chksum_free(p_chk_sum, p_cookie.as_mut_ptr() as *mut u8);
        snprintf(
            psz_cookie.as_mut_ptr() as *mut i8,
            size_of::<[i8; 9]>(),
            "%.2x%.2x%.2x%.2x".as_ptr() as *const i8,
            p_cookie[0] as c_uint,
            p_cookie[1] as c_uint,
            p_cookie[2] as c_uint,
            p_cookie[3] as c_uint,
        );
        let cookie = CStr::from_ptr(psz_cookie.as_ptr() as *const i8)
            .to_str()
            .unwrap()
            .to_string();
        Ok(format!("{}-{}", name, cookie))
    }
}
pub fn solv_calcuate_cookie_for_file(path: &str) -> Result<[u8; SOLV_COOKIE_LEN]> {
    let mut file = std::fs::File::open(path)?;
    let mut buf = [0u8; 8192];
    unsafe {
        let p_chk_sum = solv_chksum_create(solv_knownid_REPOKEY_TYPE_SHA256.try_into().unwrap());
        if p_chk_sum.is_null() {
            bail!(ERROR_RDNF_SOLV_CHKSUM);
        }
        let ident = CString::new(SOLV_COOKIE_IDENT).unwrap();
        solv_chksum_add(
            p_chk_sum,
            ident.as_ptr() as *const c_void,
            libc::strlen(ident.as_ptr()) as i32,
        );
        loop {
            let len = file.read(&mut buf)?;
            if len <= 0 {
                break;
            }
            solv_chksum_add(p_chk_sum, buf.as_ptr() as *const c_void, len as i32);
            buf = [0u8; 8192];
        }
        let mut p_cookie: [u8; SOLV_COOKIE_LEN] = [0; SOLV_COOKIE_LEN];
        solv_chksum_free(p_chk_sum, p_cookie.as_mut_ptr() as *mut u8);
        Ok(p_cookie)
    }
}
