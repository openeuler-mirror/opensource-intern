use std::{
    ffi::{c_long, CString},
    fs::{create_dir_all, rename},
    mem::size_of_val,
};

use anyhow::{bail, Result};
use libc::{
    c_void, fchmod, fclose, fdopen, fopen, fread, fseek, fwrite, mkstemp, rewind, FILE, SEEK_END,
};
use solv_sys::ffi::{
    repo_add_repomdxml, repo_add_rpmmd, repo_add_solv, repo_add_updateinfoxml, repo_write, s_Repo,
    solv_xfopen, REPO_EXTEND_SOLVABLES, _IO_FILE,
};

use crate::{
    default::{SOLVCACHE_DIR_NAME, SOLV_COOKIE_LEN},
    errors::{
        ERROR_RDNF_ADD_SOLV, ERROR_RDNF_INVALID_PARAMETER, ERROR_RDNF_REPO_WRITE,
        ERROR_RDNF_SOLV_IO,
    },
    repomd::RepoMd,
};

use super::SolvRepoInfoInernal;
pub fn solv_user_metadata_cache(
    solv_repo_info: &SolvRepoInfoInernal,
    cache_file_path: &str,
) -> Result<bool> {
    unsafe {
        let cache_file_ptr = CString::new(cache_file_path).unwrap();
        let mode = CString::new("r").unwrap();
        let fp = fopen(cache_file_ptr.as_ptr(), mode.as_ptr());
        if fp.is_null() {
            return Ok(false);
        }
        let mut temp_cookie: [u8; SOLV_COOKIE_LEN] = [0; SOLV_COOKIE_LEN];
        let temp_cookie_ptr = temp_cookie.as_mut_ptr();
        let off_set = size_of_val(&temp_cookie);
        let off_set_neg = off_set as i32 * -1;
        if fseek(fp, off_set_neg as c_long, SEEK_END) != 0
            || fread(temp_cookie_ptr as *mut c_void, off_set, 1, fp) != 1
        {
            bail!(ERROR_RDNF_SOLV_IO);
        }
        let cookie = solv_repo_info.cookie.unwrap();
        if temp_cookie != cookie {
            bail!(ERROR_RDNF_SOLV_IO);
        }
        rewind(fp);
        if repo_add_solv(solv_repo_info.repo, fp as *mut _IO_FILE, 0) != 0 {
            bail!(ERROR_RDNF_ADD_SOLV);
        };
        fclose(fp);
    }
    Ok(true)
}

pub fn solv_read_yum_repo(
    p_repo: &*mut s_Repo,
    repo_md_file: String,
    repo_md: RepoMd,
) -> Result<()> {
    solv_load_repomd(*p_repo, &repo_md_file)?;
    if let Some(primary) = repo_md.primary {
        solv_load_repomd_primary(*p_repo, primary.location.as_str())?;
    }
    if let Some(filelists) = repo_md.filelists {
        solv_load_repomd_filelists(*p_repo, filelists.location.as_str())?;
    }
    if let Some(updateinfo) = repo_md.updateinfo {
        solv_load_repomd_updateinfo(*p_repo, updateinfo.location.as_str())?;
    }
    if let Some(other) = repo_md.other {
        solv_load_repomd_other(*p_repo, other.location.as_str())?;
    }
    Ok(())
}
pub fn solv_load_repomd(p_repo: *mut s_Repo, repo_md_file: &String) -> Result<()> {
    unsafe {
        let file_name = CString::new(repo_md_file.as_str()).unwrap();
        let mode = CString::new("r").unwrap();
        let fp = fopen(file_name.as_ptr(), mode.as_ptr());
        if fp.is_null() {
            println!("a {}", repo_md_file);
            bail!(ERROR_RDNF_SOLV_IO);
        }
        if repo_add_repomdxml(p_repo, fp as *mut _IO_FILE, 0) != 0 {
            println!("b {}", repo_md_file);
            bail!(ERROR_RDNF_SOLV_IO);
        }
        fclose(fp as *mut FILE);
    }
    Ok(())
}
pub fn solv_load_repomd_primary(p_repo: *mut s_Repo, primary: &str) -> Result<()> {
    unsafe {
        let psz_primary = CString::new(primary).unwrap();
        let mode = CString::new("r").unwrap();
        let fp = solv_xfopen(psz_primary.as_ptr(), mode.as_ptr());
        if fp.is_null() {
            bail!(ERROR_RDNF_SOLV_IO)
        }
        if repo_add_rpmmd(p_repo, fp, 0 as *const i8, 0) != 0 {
            println!("c");
            bail!(ERROR_RDNF_SOLV_IO)
        };
        fclose(fp as *mut FILE);
    }
    Ok(())
}
pub fn solv_load_repomd_filelists(p_repo: *mut s_Repo, filelists: &str) -> Result<()> {
    let psz_filelists = CString::new(filelists).unwrap();
    let mode = CString::new("r").unwrap();
    let language = CString::new("FL").unwrap();
    unsafe {
        let fp = solv_xfopen(psz_filelists.as_ptr(), mode.as_ptr());
        if fp.is_null() {
            println!("e {}", filelists);
            bail!(ERROR_RDNF_SOLV_IO)
        }
        if repo_add_rpmmd(p_repo, fp, language.as_ptr(), REPO_EXTEND_SOLVABLES as i32) != 0 {
            println!("f {}", filelists);
            bail!(ERROR_RDNF_SOLV_IO)
        }
        fclose(fp as *mut FILE);
    }
    Ok(())
}
pub fn solv_load_repomd_updateinfo(p_repo: *mut s_Repo, updateinfo: &str) -> Result<()> {
    let psz_updateinfo = CString::new(updateinfo).unwrap();
    let mode = CString::new("r").unwrap();
    unsafe {
        let fp = solv_xfopen(psz_updateinfo.as_ptr(), mode.as_ptr());
        if fp.is_null() {
            println!("g {}", updateinfo);
            bail!(ERROR_RDNF_SOLV_IO)
        }
        if repo_add_updateinfoxml(p_repo, fp, 0) != 0 {
            println!("h {}", updateinfo);
            bail!(ERROR_RDNF_SOLV_IO)
        }
        fclose(fp as *mut FILE);
    }
    Ok(())
}
pub fn solv_load_repomd_other(p_repo: *mut s_Repo, other: &str) -> Result<()> {
    let psz_other = CString::new(other).unwrap();
    let mode = CString::new("r").unwrap();
    let language = CString::new("en").unwrap();
    unsafe {
        let fp = solv_xfopen(psz_other.as_ptr(), mode.as_ptr());
        if fp.is_null() {
            println!("i {}", other);
            bail!(ERROR_RDNF_SOLV_IO)
        }
        if repo_add_rpmmd(p_repo, fp, language.as_ptr(), REPO_EXTEND_SOLVABLES as i32) != 0 {
            println!("j {}", other);
            bail!(ERROR_RDNF_SOLV_IO)
        }
        fclose(fp as *mut FILE);
    }
    Ok(())
}

pub fn solv_create_metadata_cache(
    solv_repo_info: &SolvRepoInfoInernal,
    repo_id: &str,
) -> Result<()> {
    let solv_cache_dir = solv_repo_info.repo_cache_dir.clone().unwrap() + SOLVCACHE_DIR_NAME;
    create_dir_all(solv_cache_dir.clone())?;
    let temp_solv_file = solv_cache_dir.clone() + "/" + ".newsolv-XXXXXX";
    let temp_solv_file_ptr = CString::new(temp_solv_file.as_str()).unwrap().into_raw();
    unsafe {
        let fd = mkstemp(temp_solv_file_ptr);
        if fd < 0 {
            println!("k {}", repo_id);
            bail!(ERROR_RDNF_SOLV_IO);
        }
        fchmod(fd, 0o444);
        let mode = CString::new("w").unwrap();
        let fp = fdopen(fd, mode.as_ptr());
        if fp.is_null() {
            println!("l {}", repo_id);
            bail!(ERROR_RDNF_SOLV_IO);
        }
        let p_repo = solv_repo_info.repo;
        if repo_write(p_repo, fp as *mut _IO_FILE) != 0 {
            bail!(ERROR_RDNF_REPO_WRITE);
        }
        let cookie = solv_repo_info.cookie.unwrap();
        if fwrite(cookie.as_ptr() as *const c_void, SOLV_COOKIE_LEN, 1, fp) != 1 {
            // println!("m {}", repo_id);
            bail!(ERROR_RDNF_SOLV_IO)
        }
        if (*p_repo).pool.is_null() {
            bail!(ERROR_RDNF_INVALID_PARAMETER);
        }
        //   let solvables_start=(*(*p_repo).pool).solvables;
        //   slice::from_raw_parts(solvables_start, len);
        // let temp_solv_file = CStr::from_ptr(temp_solv_file).to_str().unwrap();
        let temp_solv_file = CString::from_raw(temp_solv_file_ptr);
        let solv_file_path = solv_cache_dir + "/" + repo_id + ".solv";
        rename(temp_solv_file.to_str().unwrap(), solv_file_path.clone())?;
        //  let mut perms=fs::metadata(solv_file_path.clone())?.permissions();
        //  perms.set_readonly(true);
        //  fs::set_permissions(solv_file_path.clone(), perms)?;
    }

    Ok(())
}
