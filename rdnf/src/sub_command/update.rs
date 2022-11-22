use std::{
    ffi::{CStr, CString},
};

use anyhow::{bail, Result};
use chrono::NaiveDateTime;
use libc::{atof};
use solv_sys::ffi::{
    dataiterator_free, dataiterator_init, dataiterator_prepend_keyname, dataiterator_setpos,
    dataiterator_setpos_parent, dataiterator_skip_solvable, dataiterator_step, pool_evrcmp,
    pool_id2str, pool_lookup_id, pool_lookup_num, pool_lookup_str, pool_lookup_void,
    solv_knownid_SOLVABLE_BUILDTIME, solv_knownid_SOLVABLE_DESCRIPTION, solv_knownid_SOLVABLE_NAME,
    solv_knownid_SOLVABLE_PATCHCATEGORY, solv_knownid_UPDATE_COLLECTION,
    solv_knownid_UPDATE_COLLECTION_ARCH, solv_knownid_UPDATE_COLLECTION_EVR,
    solv_knownid_UPDATE_COLLECTION_FILENAME, solv_knownid_UPDATE_COLLECTION_NAME,
    solv_knownid_UPDATE_REBOOT, solv_knownid_UPDATE_SEVERITY, Repo, EVRCMP_COMPARE, SEARCH_STRING,
    SOLVID_POS,
};

use crate::{
    c_lib::{create_dataiterator_empty, pool_id2solvable, queue_push},
    errors::ERROR_RDNF_INVALID_PARAMETER,
    solv::{rdnf_query::init_queue, sack::Solvsack, SolvPackageList},
    utils::c_str_ptr_to_rust_string,
    Rdnf,
};
pub enum UpdateInfoKind {
    Unknown,
    Security,
    Bugfix,
    Enhancement,
}
pub struct UpdateInfo {
    kind: UpdateInfoKind,
    pkg_id: Option<String>,
    pkg_date: Option<NaiveDateTime>,
    pkg_desc: Option<String>,
    reboot_required: bool,
    _refers: Vec<UpdateInfoRef>,
    pkgs: Vec<UpdateInfoPkg>,
}
impl UpdateInfo {
    pub fn default() -> Self {
        UpdateInfo {
            kind: UpdateInfoKind::Unknown,
            pkg_id: None,
            pkg_date: None,
            pkg_desc: None,
            reboot_required: false,
            _refers: Vec::<UpdateInfoRef>::new(),
            pkgs: Vec::<UpdateInfoPkg>::new(),
        }
    }
}
pub struct UpdateInfoRef {
    pub pkg_id: Option<String>,
    pub pkg_link: Option<String>,
    pub pkg_title: Option<String>,
    pub pkg_type: Option<String>,
}
pub struct UpdateInfoPkg {
    pkg_name: Option<String>,
    pkg_file_name: Option<String>,
    pkg_evr: Option<String>,
    pkg_arch: Option<String>,
}
impl UpdateInfoPkg {
    pub fn default() -> Self {
        UpdateInfoPkg {
            pkg_name: None,
            pkg_file_name: None,
            pkg_evr: None,
            pkg_arch: None,
        }
    }
}
impl Rdnf {
    pub fn update_info(&self, pkg_name: Option<Vec<String>>) -> Result<Vec<UpdateInfo>> {
        // self.make_cache()?;
        let installed_pkg_list = match pkg_name {
            Some(s) => self.rc.sack.solv_find_installed_pkg_by_multiple_names(s)?,
            None => self.rc.sack.solv_find_all_installed()?,
        };
        let security = self.rc.cli.security;
        let sec_severity = self.rc.cli.sec_severity.clone();
        let reboot_required = self.rc.cli.reboot_required;
        let count = installed_pkg_list.get_size();
        let mut infos=Vec::new();
        for index in 0..count {
            let id = installed_pkg_list.get_pkg_id(index);
            match self.rc.sack.solv_get_update_advisories(id) {
                Err(_) => {
                    continue;
                }
                Ok(update_adv_pkg_list) => {
                    let ucount = update_adv_pkg_list.get_size();
                    for uadv in 0..ucount {
                        let adv_id = update_adv_pkg_list.get_pkg_id(uadv);
                        let info=self.rc.sack.populate_updateinfo_of_one_advisory(adv_id, security, &sec_severity, reboot_required)?;
                        if info.is_some() {
                            infos.push(info.unwrap())
                        }

                    }
                }
            };
        }
        if infos.len() >0{
            self.rc.term.write_line(&format!("{} updates.",infos.len()))?;
        }
        Ok(infos)
    }
    pub fn get_update_pkgs(&self,) -> Result<Vec<String>> {
        let update_infos=self.update_info(None)?;
        let count=update_infos.len();
        let mut pkgs=Vec::new();
        if count>0{
            for update_info in update_infos {
                for update_info_pkg in update_info.pkgs {
                    if update_info_pkg.pkg_name.is_some() {
                        pkgs.push(update_info_pkg.pkg_name.unwrap())
                    }
                }
            }
        }    
        Ok(pkgs) 
    }
}
impl Solvsack {
    pub fn solv_get_update_advisories(&self, id: i32) -> Result<SolvPackageList> {
        let mut queue_adv = init_queue();
        let solvable = pool_id2solvable(self.pool, id);
        if solvable.is_null() {
            bail!(ERROR_RDNF_INVALID_PARAMETER);
        };
        let pkg_name_ptr = unsafe { pool_id2str(self.pool, (*solvable).name) };
        let mut di = create_dataiterator_empty();
        unsafe {
            dataiterator_init(
                &mut di,
                self.pool,
                0 as *mut Repo,
                0,
                solv_knownid_UPDATE_COLLECTION_NAME as i32,
                pkg_name_ptr,
                SEARCH_STRING as i32,
            );
            dataiterator_prepend_keyname(&mut di, solv_knownid_UPDATE_COLLECTION as i32);
            while dataiterator_step(&mut di) != 0 {
                dataiterator_setpos_parent(&mut di);
                let arch = pool_lookup_id(
                    self.pool,
                    SOLVID_POS,
                    solv_knownid_UPDATE_COLLECTION_ARCH as i32,
                );
                if arch != (*solvable).arch {
                    continue;
                }
                let evr = pool_lookup_id(
                    self.pool,
                    SOLVID_POS,
                    solv_knownid_UPDATE_COLLECTION_EVR as i32,
                );
                if evr == 0 {
                    continue;
                }
                let cmp_result =
                    pool_evrcmp(self.pool, evr, (*solvable).evr, EVRCMP_COMPARE as i32);
                if cmp_result > 0 {
                    queue_push(&mut queue_adv, di.solvid);
                    dataiterator_skip_solvable(&mut di);
                }
            }
        };
        unsafe { dataiterator_free(&mut di) };
        SolvPackageList::queue_to_pkg_list(&mut queue_adv)
    }
    pub fn populate_updateinfo_of_one_advisory(
        &self,
        adv_id: i32,
        security: bool,
        sec_severity: &Option<String>,
        reboot_required: bool,
    ) -> Result<Option<UpdateInfo>> {
        let psz_type_ptr = unsafe {
            pool_lookup_str(
                self.pool,
                adv_id,
                solv_knownid_SOLVABLE_PATCHCATEGORY as i32,
            )
        };
        let temp_ptr =
            unsafe { pool_lookup_str(self.pool, adv_id, solv_knownid_UPDATE_SEVERITY as i32) };
        let reboot =
            unsafe { pool_lookup_void(self.pool, adv_id, solv_knownid_UPDATE_REBOOT as i32) } == 1;
        let mut keep_entry = true;
        if security {
            if !psz_type_ptr.is_null() {
                if unsafe { CStr::from_ptr(psz_type_ptr).to_str()? } == "security" {
                    keep_entry = false;
                }
            }
        } else if sec_severity.is_some() {
            let severity = CString::new(sec_severity.as_ref().unwrap().as_str())?;
            if temp_ptr.is_null() || unsafe { atof(severity.as_ptr()) > atof(temp_ptr) } {
                keep_entry = false;
            }
        }
        if reboot_required {
            if reboot {
                keep_entry = false;
            }
        }
        if keep_entry {
            let mut update_info = UpdateInfo::default();
            update_info.kind = if psz_type_ptr.is_null() {
                UpdateInfoKind::Unknown
            } else {
                let psz_type = unsafe { CStr::from_ptr(psz_type_ptr).to_str()? };
                match psz_type {
                    "bugfix" => UpdateInfoKind::Bugfix,
                    "enhancement" => UpdateInfoKind::Enhancement,
                    "security" => UpdateInfoKind::Security,
                    _ => UpdateInfoKind::Unknown,
                }
            };
            update_info.reboot_required = reboot;
            update_info.pkg_id=c_str_ptr_to_rust_string(unsafe {
                pool_lookup_str(self.pool, adv_id, solv_knownid_SOLVABLE_NAME as i32)
            });
            update_info.pkg_desc=c_str_ptr_to_rust_string(unsafe {
                pool_lookup_str(self.pool, adv_id, solv_knownid_SOLVABLE_DESCRIPTION as i32)
            });
            let updated = unsafe {
                pool_lookup_num(self.pool, adv_id, solv_knownid_SOLVABLE_BUILDTIME as i32, 0)
            };
            if updated > 0 {
                update_info.pkg_date =
                    Some(NaiveDateTime::from_timestamp_opt(updated as i64, 0).unwrap());
            }
            update_info.pkgs=self.get_update_info_pkgs(adv_id)?;
            return Ok(Some(update_info));
        }
        Ok(None)
    }
    pub fn get_update_info_pkgs(&self, id: i32) -> Result<Vec<UpdateInfoPkg>> {
        let mut di = create_dataiterator_empty();
        unsafe {
            dataiterator_init(
                &mut di,
                self.pool,
                0 as *mut Repo,
                id,
                solv_knownid_UPDATE_COLLECTION as i32,
                0 as *const i8,
                0,
            )
        };
        let mut update_pkgs = Vec::new();
        while unsafe { dataiterator_step(&mut di) } != 0 {
            unsafe { dataiterator_setpos(&mut di) };
            let mut update_pkg = UpdateInfoPkg::default();
            update_pkg.pkg_name = c_str_ptr_to_rust_string(unsafe {
                pool_lookup_str(
                    self.pool,
                    SOLVID_POS as i32,
                    solv_knownid_UPDATE_COLLECTION_NAME as i32,
                )
            });
            update_pkg.pkg_evr = c_str_ptr_to_rust_string(unsafe {
                pool_lookup_str(
                    self.pool,
                    SOLVID_POS as i32,
                    solv_knownid_UPDATE_COLLECTION_EVR as i32,
                )
            });
            update_pkg.pkg_arch = c_str_ptr_to_rust_string(unsafe {
                pool_lookup_str(
                    self.pool,
                    SOLVID_POS as i32,
                    solv_knownid_UPDATE_COLLECTION_ARCH as i32,
                )
            });
            update_pkg.pkg_file_name = c_str_ptr_to_rust_string(unsafe {
                pool_lookup_str(
                    self.pool,
                    SOLVID_POS as i32,
                    solv_knownid_UPDATE_COLLECTION_FILENAME as i32,
                )
            });
            update_pkgs.push(update_pkg);
        }
        unsafe{dataiterator_free(&mut di)};
        Ok(update_pkgs)
    }
}
