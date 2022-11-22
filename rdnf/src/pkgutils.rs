use std::ffi::CStr;

use anyhow::{bail, Result};
use solv_sys::ffi::{Queue};

use crate::{
    c_lib::{pool_id2solvable, queue_push},
    default::RDNF_NAME,
    errors::{ERROR_RDNF_NO_DATA, ERROR_RDNF_NO_MATCH, ERROR_RDNF_SELF_ERASE},
    goal::{SolvedPkgInfoBase},
    solv::{sack::Solvsack, SolvPackageList},
    Rdnf, sub_command::info::{PkgInfo, PkgInfoLevel},
};

impl Solvsack {
    pub fn get_glob_pkgs(&self,pkg_glob:&str,queue_goal: *mut Queue)->Result<()>{
        let pkg_list=self.solv_find_available_pkg_by_name(pkg_glob)?;
        if pkg_list.get_size() >0{
            for index in 0..pkg_list.get_size(){
                let id=pkg_list.get_pkg_id(index);
                queue_push(queue_goal, id);
            }
        }
        Ok(())
    }
    pub fn add_pkgs_for_install(&self, queue_goal: *mut Queue, pkg_name: &str) -> Result<bool> {
        let highest_id = self.solv_find_highest_available(pkg_name)?;
        if self.verify_install_pkg(highest_id)? {
            queue_push(queue_goal, highest_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn add_pkgs_for_upgrade(&self,queue_goal: *mut Queue,pkg_name: &str)->Result<bool>{
        let highest_id=self.solv_find_highest_available(pkg_name)?;
        if self.verify_upgrade_pkg(highest_id)?{
            queue_push(queue_goal, highest_id);
            Ok(true)
        }else{
            Ok(false)
        }
    }
    pub fn add_pkgs_for_erase(&self,queue_goal: *mut Queue,pkg_name: &str)->Result<bool>{
        match self.solv_find_installed_pkg_by_name(pkg_name) {
            Ok(install_pkg_list) => {
                let count = install_pkg_list.get_size();
                for index in 0..count{
                    let id=install_pkg_list.get_pkg_id(index);
                    queue_push(queue_goal, id);
                }
            },
            Err(_) => {bail!("Package {} don't be installed,can't be removed",pkg_name)},
        };
        Ok(true)
    }
    pub fn add_pkgs_for_reinstall(&self,queue_goal: *mut Queue,pkg_name: &str)->Result<()>{
        match self.solv_find_installed_pkg_by_name(pkg_name) {
            Ok(install_pkg_list) => {
                let installed_id=install_pkg_list.get_pkg_id(0);
                let nevr=self.solv_get_pkg_nevr_by_id(installed_id)?;
                let available_pkg_list=self.solv_find_available_pkg_by_name(nevr.as_str())?;
                let available_pkg_id=available_pkg_list.get_pkg_id(0);
                queue_push(queue_goal, available_pkg_id);
            },
            Err(_) => {bail!("Package {} don't be installed,can't be reinstalled",pkg_name)},
        };
        Ok(())
    }
   
    pub fn verify_install_pkg(&self, id: i32) -> Result<bool> {
        let pkg_name = self.solv_get_pkg_name_by_id(id)?;
        let installed_id = match self.solv_find_highest_installed(pkg_name.as_str()) {
            Ok(s) => s,
            Err(_) => {
                return Ok(true);
            }
        };
        let evr_cmp = self.solv_cmp_evr(id, installed_id)?;
        Ok(evr_cmp != 0)
    }
    pub fn verify_upgrade_pkg(&self,id:i32)->Result<bool>{
        let pkg_name=self.solv_get_pkg_name_by_id(id)?;
        let intalled_id=match self.solv_find_highest_installed(pkg_name.as_str()) {
            Ok(s) => {s},
            Err(_) => {return Ok(true);},
        };
        let result=match self.solv_cmp_evr(id, intalled_id) {
            Ok(evr) => {evr >0},
            Err(_) => {true},
        };
        Ok(result)
    }
    pub fn solv_get_pkg_reponame_by_id(&self, pkg_id: i32) -> Result<&str> {
        let solv = pool_id2solvable(self.pool, pkg_id);
        if solv.is_null() {
            bail!(ERROR_RDNF_NO_DATA);
        };
        let name_ptr = unsafe { (*(*solv).repo).name };
        if name_ptr.is_null() {
            bail!(ERROR_RDNF_NO_DATA);
        };
        Ok(unsafe { CStr::from_ptr(name_ptr).to_str()? })
    }
}
impl PkgInfo {
    pub fn populate_pkg_info(sack: &Solvsack, pkg_list: &SolvPackageList,level:PkgInfoLevel) -> Result<Vec<Self>> {
        let count = pkg_list.get_size();
        if count == 0 {
            bail!(ERROR_RDNF_NO_MATCH);
        };
        let mut pkginfos = Vec::new();
        for index in 0..count {
            let pkg_id = pkg_list.get_pkg_id(index);
            match level {
                _ => {},
            }
            let base = sack.solv_get_pkginfo_base_by_id(pkg_id)?;
            let details = sack.solv_get_pkginfo_details_by_id(pkg_id)?;
            let other=match level {
                PkgInfoLevel::Other => {
                  Some(sack.solv_get_pkginfo_other_by_id(pkg_id)?)  
                },
                _=>{None}
            };
            pkginfos.push(PkgInfo {
                base,
                details,
                other,
            });
        }
        Ok(pkginfos)
    }
}
impl Rdnf {
    pub fn pkgs_to_exclude(&self) -> Result<Vec<String>> {
        let mut count = 0;
        let mut exclude_pkgs = Vec::new();
        if !self.rc.cli.disable_excludes && self.rc.conf.excludepkgs.is_some() {
            println!("Warning: The following packages are excluded from rdnf.conf");
            for ele in self.rc.conf.excludepkgs.as_ref().unwrap() {
                print!("{}/t", ele);
                exclude_pkgs.push(ele.clone());
                count += 1;
                if count % 3 == 0 {
                    print!("\n");
                }
            }
        }
        if !self.rc.cli.disable_excludes && self.rc.cli.exclude.is_some() {
            for ele in self.rc.cli.exclude.as_ref().unwrap() {
                exclude_pkgs.push(ele.clone());
            }
        }
        Ok(exclude_pkgs)
    }
}
impl SolvedPkgInfoBase {
    pub fn check_protected_pkgs(&self) -> Result<()> {
        if self.to_remove.is_some() {
            for pkg_info in self.to_remove.as_ref().unwrap() {
                if pkg_info.base.name == RDNF_NAME {
                    bail!(ERROR_RDNF_SELF_ERASE)
                }
            }
        }
        if self.obsoleted.is_some() {
            for pkg_info in self.obsoleted.as_ref().unwrap() {
                if pkg_info.base.name == RDNF_NAME {
                    bail!(ERROR_RDNF_SELF_ERASE)
                }
            }
        }
        Ok(())
    }
}
