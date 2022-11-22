use crate::{
    default::{
        REPODATA_DIR_NAME, REPO_BASEURL_FILE_NAME, REPO_METADATA_FILE_NAME,
        REPO_METADATA_FILE_PATH, REPO_METADATA_MARKER, REPO_METALINK_FILE_NAME, RPM_CACHE_DIR_NAME,
        SOLVCACHE_DIR_NAME, SOLV_COOKIE_LEN,
    },
    errors::ERROR_RDNF_INVALID_PARAMETER,
    metalink::MetalinkContext,
    repomd::RepoMd,
    solv::{
        rdnf_repo::{solv_create_metadata_cache, solv_read_yum_repo, solv_user_metadata_cache},
        sack::solv_calcuate_cookie_for_file,
        SolvRepoInfoInernal,
    },
    RdnfContext,
};
use anyhow::{bail, Result};
use console::Term;
use curl::easy::{Easy2, Handler};
use indicatif::{ProgressBar, ProgressStyle};
use libc::c_void;
use md5::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use solv_sys::ffi::{pool_createwhatprovides, repo_create};
use std::{
    ffi::CString,
    fs::{create_dir_all, remove_file, rename, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use super::{cache::recursively_remove_dir, repo::RepoData};

impl RdnfContext {
    pub fn init_repo(&self, repo: RepoData) -> Result<RepoData> {
        let cache_name = repo.details.cache_name.as_ref().unwrap().as_str();
        let repo_cache_dir = self.conf.cachedir.clone() + cache_name + "/";
        let mut repo = repo;
        let repo_md = self.get_repo_md(&mut repo, repo_cache_dir.clone())?;
        let pool = self.sack.pool;
        let id_ptr = CString::new(repo.psz_id.as_str()).unwrap();
        unsafe {
            let p_repo = repo_create(pool, id_ptr.as_ptr());
            if p_repo.is_null() {
                bail!(ERROR_RDNF_INVALID_PARAMETER);
            }
            let repo_md_file = repo_cache_dir.clone() + REPO_METADATA_FILE_PATH;
            let cookie = solv_calcuate_cookie_for_file(repo_md_file.as_str())?;
            let solv_repo_info = SolvRepoInfoInernal {
                repo: p_repo,
                cookie: Some(cookie),
                n_cookie_set: Some(1),
                repo_cache_dir: Some(repo_cache_dir.clone()),
            };
            let p = Box::into_raw(Box::new(&solv_repo_info)) as *mut c_void;
            (*p_repo).appdata = p;
            let psz_cache_file_path =
                repo_cache_dir.clone() + SOLVCACHE_DIR_NAME + "/" + repo.psz_id.as_str() + ".solv";
            if !solv_user_metadata_cache(&solv_repo_info, psz_cache_file_path.as_str())? {
                solv_read_yum_repo(&solv_repo_info.repo, repo_md_file.clone(), repo_md)?;
                solv_create_metadata_cache(&solv_repo_info, repo.psz_id.as_str())?;
            }
            pool_createwhatprovides(self.sack.pool);
        }
        Ok(repo)
    }
    pub fn get_repo_md(&self, repo: &mut RepoData, repo_cache_dir: String) -> Result<RepoMd> {
        let mut replace_repo_md = false;
        let metalink = repo.details.meta_link.is_some();
        let keepcache = self.conf.keepcache;
        let mut replace_base_url = false;
        let mut new_repo_md_file = false;
        let mut cookie: [u8; SOLV_COOKIE_LEN] = [0; SOLV_COOKIE_LEN];
        if repo.details.base_url.is_none() && repo.details.meta_link.is_none() {
            bail!("Cannot find a valid base URL for repo: {}", repo.psz_name);
        }
        let repo_data_dir = repo_cache_dir.clone() + REPODATA_DIR_NAME + "/";
        let solv_cache_dir = repo_cache_dir.clone() + SOLVCACHE_DIR_NAME + "/";
        let last_refresh_marker = repo_cache_dir.clone() + REPO_METADATA_MARKER;
        let rpms_cache_dir = repo_cache_dir.clone() + RPM_CACHE_DIR_NAME + "/";
        let repo_md_file = repo_data_dir.clone() + REPO_METADATA_FILE_NAME;
        let meta_link_file = repo_data_dir.clone() + REPO_METALINK_FILE_NAME;
        let base_url_file = repo_data_dir.clone() + REPO_BASEURL_FILE_NAME;
        let tmp_repo_data_dir = repo_cache_dir.clone() + "tmp/";
        let tmp_repo_md_file = tmp_repo_data_dir.clone() + REPO_METADATA_FILE_NAME;
        let tmp_meta_link_file = tmp_repo_data_dir.clone() + REPO_METALINK_FILE_NAME;
        let tmp_base_url_file = tmp_repo_data_dir.clone() + REPO_BASEURL_FILE_NAME;
        let mut need_download = if repo.details.meta_link.is_some() {
            !Path::new(meta_link_file.as_str()).exists()
                && !Path::new(base_url_file.as_str()).exists()
        } else {
            !Path::new(repo_md_file.as_str()).exists()
        };
        if self.cli.refresh {
            if repo.details.meta_link.is_some() {
                if Path::new(&meta_link_file).exists() {
                    cookie = solv_calcuate_cookie_for_file(meta_link_file.as_str())?;
                }
            } else {
                if Path::new(&repo_md_file).exists() {
                    cookie = solv_calcuate_cookie_for_file(repo_md_file.as_str())?;
                }
            }
            need_download = true;
        }

        if need_download && !self.cli.cacheonly {
            create_dir_all(&tmp_repo_data_dir)?;
            if metalink {
                let url = repo.details.meta_link.clone().unwrap();
                download_file(
                    &self,
                    repo,
                    url.as_str(),
                    tmp_meta_link_file.as_str(),
                    &repo.psz_name,
                )?;
                let mut mc = match MetalinkContext::from_with_filename(
                    &tmp_meta_link_file,
                    REPO_METADATA_FILE_NAME,
                ) {
                    Ok(s) => s,
                    Err(_) => {
                        bail!("check {}.repo metalink url", repo.psz_id)
                    }
                };

                replace_repo_md = true;
                if cookie[0] != 0 {
                    let tmp_cookie = solv_calcuate_cookie_for_file(tmp_meta_link_file.as_str())?;
                    if tmp_cookie == cookie {
                        replace_repo_md = false;
                    }
                }
                if replace_repo_md {
                    let mut choose_url = None;
                    for ele in mc.urls {
                        ele.url.ends_with(REPO_METADATA_FILE_PATH);
                        if download_file(
                            &self,
                            repo,
                            ele.url.as_str(),
                            tmp_repo_md_file.as_str(),
                            repo.psz_name.as_str(),
                        )
                        .is_ok()
                        {
                            choose_url = Some(ele.url.clone());
                            break;
                        };
                    }
                    match choose_url {
                        Some(choose_url) => {
                            let tmp_base_url_file =
                                tmp_repo_data_dir.clone() + REPO_BASEURL_FILE_NAME;
                            let baseurl = choose_url.trim_end_matches(REPO_METADATA_FILE_PATH);
                            repo.details.base_url = Some(baseurl.to_string());
                            if Path::new(&tmp_base_url_file).exists() {
                                remove_file(&tmp_base_url_file)?;
                            }
                            OpenOptions::new()
                                .write(true)
                                .create(true)
                                .open(tmp_base_url_file)?
                                .write_all(baseurl.as_bytes())?;
                            mc.hashs.sort_by(|a, b| a.value.cmp(&b.value));
                            let mut flag = mc.hashs.len();
                            for ele in mc.hashs {
                                if ele
                                    .kind
                                    .checksum(tmp_repo_md_file.as_str(), ele.value.as_str())?
                                {
                                    break;
                                };
                                flag -= 1;
                            }
                            if flag == 0 {
                                bail!("{}.repo repomd.xml invalid", repo.psz_id);
                            }
                        }
                        None => {
                            bail!("{}.repo metalink don't have vaild url ", repo.psz_id)
                        }
                    }
                    replace_base_url = true;
                    new_repo_md_file = true;

                    if Path::new(&repo_md_file).exists() {
                        let cookie = solv_calcuate_cookie_for_file(repo_md_file.as_str())?;
                        let tmp_cookie = solv_calcuate_cookie_for_file(tmp_repo_md_file.as_str())?;
                        if cookie == tmp_cookie {
                            replace_repo_md = false;
                        }
                    }
                }
            } else {
                let url = repo
                    .details
                    .base_url
                    .clone()
                    .unwrap()
                    .trim_end_matches("/")
                    .to_string()
                    + "/"
                    + REPO_METADATA_FILE_PATH;
                download_file(
                    &self,
                    repo,
                    url.as_str(),
                    tmp_repo_md_file.as_str(),
                    repo.psz_name.as_str(),
                )?;
                replace_repo_md = true;
                if cookie[0] != 0 {
                    let tmp_cookie = solv_calcuate_cookie_for_file(tmp_repo_md_file.as_str())?;
                    if tmp_cookie == cookie {
                        replace_repo_md = false;
                    }
                }
                new_repo_md_file = true;
            }
        }

        if metalink && !replace_base_url && Path::new(base_url_file.as_str()).exists() {
            let mut buf = String::new();
            File::open(base_url_file.clone())
                .unwrap()
                .read_to_string(&mut buf)?;
            repo.details.base_url = Some(buf.trim_end_matches("/").to_string() + "/");
        }
        if replace_repo_md {
            recursively_remove_dir(&PathBuf::from(repo_data_dir.clone()))?;
            recursively_remove_dir(&PathBuf::from(solv_cache_dir))?;
            match remove_file(last_refresh_marker.clone()) {
                Ok(_) => {}
                Err(_) => {}
            };
            if !keepcache {
                recursively_remove_dir(&PathBuf::from(rpms_cache_dir))?;
            }
            create_dir_all(repo_data_dir)?;
            rename(tmp_repo_md_file, repo_md_file.clone())?;
        }
        if new_repo_md_file {
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(last_refresh_marker)?;
        }
        if metalink && replace_base_url {
            rename(tmp_meta_link_file, meta_link_file)?;
            rename(tmp_base_url_file, base_url_file)?;
        }

        let repo_md = RepoMd::parse_from(repo_md_file.as_str())?;
        let repo_md = repo_md.ensure_repo_md_parts(&self, repo, repo_cache_dir.clone())?;

        Ok(repo_md)
    }
}
// impl RdnfContext {
pub fn download_file(
    rc: &RdnfContext,
    repo: &RepoData,
    url: &str,
    file: &str,
    msg: &str,
) -> Result<()> {
    let pb = rc.multi_process.add(ProgressBar::new(0));
    let (_, width) = Term::stdout().size();
    let style = format!("{{msg:{}}}{{spinner:.green}}[{{bar:{}.cyan/blue}}] {{bytes}}/{{total_bytes}} ({{bytes_per_sec}},{{eta}})",width /3,width/3);
    let style = ProgressStyle::with_template(style.as_str())
        .unwrap()
        .progress_chars("#>-");
    pb.set_style(style);
    pb.set_message(msg.to_string());
    let file_path = format!("{}.tmp", file);
    let mut easy = Easy2::new(Collector {
        buffer: Vec::new(),
        pb,
        file: OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path.as_str())
            .unwrap(),
    });

    if let Some(user) = &repo.details.username {
        easy.username(user.as_str())?;
    }
    if let Some(password) = &repo.details.password {
        easy.password(password.as_str())?;
    }
    if let Some(proxy) = &rc.conf.proxy {
        easy.proxy(proxy.as_str())?;
        if let Some(username) = &rc.conf.proxy_username {
            easy.proxy_username(username.as_str())?;
            if let Some(password) = &rc.conf.proxy_password {
                easy.proxy_password(password.as_str())?;
            }
        }
    }
    easy.timeout(Duration::from_secs(repo.base.timeout))?;
    easy.low_speed_time(Duration::from_secs(repo.base.timeout))?;
    easy.low_speed_limit(repo.base.minrate)?;
    easy.max_recv_speed(repo.base.throttle)?;
    easy.ssl_verify_peer(repo.base.sslverify)?;
    easy.ssl_verify_host(repo.base.sslverify)?;
    if let Some(cert) = &repo.details.ssl_ca_cert {
        easy.ssl_cainfo_blob(cert.as_bytes())?;
    }
    if let Some(cert) = &repo.details.ssl_client_cert {
        easy.ssl_cert(cert)?;
    }
    if let Some(key) = &repo.details.ssl_client_key {
        easy.ssl_key(key)?;
    }
    easy.url(url)?;
    easy.follow_location(true)?;
    easy.progress(true)?;
    match easy.perform() {
        Ok(_) => {}
        Err(_) => {
            for x in 1..10 {
                let co = easy.get_mut();
                co.pb.set_message(format!("{}: retrying {}/10", msg, x));
                match easy.perform() {
                    Ok(_) => {
                        break;
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }
    }
    let collector = easy.get_mut();
    collector.finish()?;
    collector.pb.finish_and_clear();
    let status = easy.response_code()?;
    if status >= 400 {
        bail!("{} when downloading {}  Please check repo url or refresh metadata with 'rdnf makecache'",status,url);
    } else {
        rename(file_path, file)?;
    }

    Ok(())
}
// }

struct Collector {
    buffer: Vec<u8>,
    pb: ProgressBar,
    file: File,
}
impl Collector {
    fn finish(&mut self) -> Result<()> {
        self.file.write(self.buffer.as_slice())?;
        self.buffer.clear();
        Ok(())
    }
}
const LIMIT: usize = 4 * 1024;
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, curl::easy::WriteError> {
        self.buffer.extend_from_slice(data);
        if self.buffer.len() >= LIMIT {
            match self.file.write_all(self.buffer.as_slice()) {
                Ok(_) => {}
                Err(_) => {
                    return Err(curl::easy::WriteError::Pause);
                }
            };
            self.buffer.clear();
        }
        Ok(data.len())
    }
    fn progress(&mut self, dltotal: f64, dlnow: f64, _ultotal: f64, _ulnow: f64) -> bool {
        if dltotal <= 0.0 {
            self.pb.set_length(0);
        }
        self.pb.set_length(dltotal as u64);
        self.pb.set_position(dlnow as u64);
        true
    }
}
#[derive(Debug, Clone)]
pub enum HashKind {
    MD5,
    SHA1,
    SHA256,
    SHA512,
    Invalid,
}
impl From<&str> for HashKind {
    #[inline]
    fn from(kind: &str) -> Self {
        match kind.as_bytes() {
            b"md5" => Self::MD5,
            b"sha1" => Self::SHA1,
            b"sha256" => Self::SHA256,
            b"sha512" => Self::SHA512,
            _ => Self::Invalid,
        }
    }
}
impl HashKind {
    #[inline]
    pub fn checksum(self, file: &str, hash: &str) -> Result<bool> {
        let mut file = File::open(file)?;
        let sum = match self {
            HashKind::MD5 => {
                let mut hasher = Md5::new();
                io::copy(&mut file, &mut hasher)?;
                hex::encode(hasher.finalize())
            }
            HashKind::SHA1 => {
                let mut hasher = Sha1::new();
                io::copy(&mut file, &mut hasher)?;
                hex::encode(hasher.finalize())
            }
            HashKind::SHA256 => {
                let mut hasher = Sha256::new();
                io::copy(&mut file, &mut hasher)?;
                hex::encode(hasher.finalize())
            }

            HashKind::SHA512 => {
                let mut hasher = Sha512::new();
                io::copy(&mut file, &mut hasher)?;
                hex::encode(hasher.finalize())
            }
            HashKind::Invalid => {
                bail!("Invalid hash algorithm")
            }
        };
        Ok(sum == hash)
    }
}
// #[inline]
// pub fn check_hash(kind:&String,file:&str,hash:&String)->Result<bool>{
//     let mut file=File::open(file)?;
//    let s=match kind.as_bytes() {
//         b"md5" => {
//             let mut hasher = Md5::new();
//             io::copy(&mut file,&mut hasher)?;
//             hex::encode(hasher.finalize())
//         },
//         b"sha1"=>{
//             let mut hasher=Sha1::new();
//             io::copy(&mut file,&mut hasher)?;
//             hex::encode(hasher.finalize())
//         },
//         b"sha256"=>{
//             let mut hasher=Sha256::new();
//             io::copy(&mut file,&mut hasher)?;
//             hex::encode(hasher.finalize())
//         }
//         b"sha512"=>{
//             let mut hasher=Sha512::new();
//             io::copy(&mut file, &mut hasher)?;
//             hex::encode(hasher.finalize())
//         }
//         _=>{
//             bail!("invalid hash {}",kind)
//         }
//     };
//     Ok(s==hash.as_str())
// }
// #[test]
// fn test(){
//     assert!(check_hash(&"md5".to_string(),"/var/cache/rdnf/rpmfusion-nonfree-updates-77c32c35/tmp/repomd.xml",&"d56148af65634f42c1a2c9bd6a0597be".to_string()).unwrap());
// }
