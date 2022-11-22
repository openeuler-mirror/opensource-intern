use solv_sys::ffi::{Queue, Repo };



pub mod rdnf_pkg;
pub mod rdnf_query;
pub mod rdnf_repo;
pub mod sack;
#[derive(Debug, Clone)]
pub struct SolvRepoInfoInernal {
    pub repo: *mut Repo,
    pub cookie: Option<[u8; 32]>,
    pub n_cookie_set: Option<i32>,
    pub repo_cache_dir: Option<String>,
}
pub struct SolvPackageList {
    pub pkgs: Queue,
}
impl SolvPackageList {
    pub fn get_size(&self) -> u32 {
        self.pkgs.count as u32
    }
}


