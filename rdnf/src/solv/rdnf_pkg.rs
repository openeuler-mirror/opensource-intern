use std::{
    ffi::{CStr, CString},
    mem,
};

use crate::{
    c_lib::{
        create_dataiterator_empty, get_queue_element_value, map_set, map_setall,
        pool_disabled_solvable, pool_id2solvable,
    },
    errors::{
        ERROR_RDNF_INVALID_PARAMETER, ERROR_RDNF_NO_DATA, ERROR_RDNF_NO_MATCH,
        ERROR_RDNF_SOLV_FAILED,
    },
    sub_command::{
        info::{PkgInfoBase, PkgInfoDetails, PkgInfoOther},
        install::is_glob,
    },
    utils::{c_str_ptr_to_rust_string, format_size},
};

use super::{
    rdnf_query::{init_queue, SolvQuery},
    sack::Solvsack,
    SolvPackageList,
};

use anyhow::{bail, Result};
use libc::strtol;

use solv_sys::ffi::{
    dataiterator_init, dataiterator_step, map_grow, map_init, map_subtract, pool_evrcmp_str,
    pool_id2str, pool_solvable2str, queue_insertn, solv_knownid_SOLVABLE_ARCH,
    solv_knownid_SOLVABLE_EVR, solv_knownid_SOLVABLE_INSTALLSIZE, solv_knownid_SOLVABLE_NAME,
    solv_knownid_SOLVABLE_SUMMARY, solv_knownid_SOLVABLE_URL, solvable_get_location,
    solvable_lookup_num, solvable_lookup_str, solver_findproblemrule, solver_problem_count,
    solver_problemruleinfo2str, solver_ruleinfo, Dataiterator, Map, Pool, Queue, Repo, Solver,
    SolverRuleinfo_SOLVER_RULE_PKG_CONFLICTS, SolverRuleinfo_SOLVER_RULE_PKG_IMPLICIT_OBSOLETES,
    SolverRuleinfo_SOLVER_RULE_PKG_INSTALLED_OBSOLETES,
    SolverRuleinfo_SOLVER_RULE_PKG_NOT_INSTALLABLE, SolverRuleinfo_SOLVER_RULE_PKG_OBSOLETES,
    SolverRuleinfo_SOLVER_RULE_PKG_REQUIRES, SolverRuleinfo_SOLVER_RULE_PKG_SELF_CONFLICT,
    EVRCMP_COMPARE, SEARCH_GLOB, SEARCH_STRING, solv_knownid_SOLVABLE_LICENSE, solv_knownid_SOLVABLE_DESCRIPTION,
};
#[derive(Debug, Clone, Copy)]
pub struct SkipProblem {
    pub none: bool,
    pub conflicts: bool,
    pub obsoletes: bool,
    pub disabled: bool,
}
pub fn skip_based_on_type(
    solv: *mut Solver,
    rule_type: u32,
    source: i32,
    skip_problem: SkipProblem,
) -> bool {
    let mut result = false;
    if skip_problem.conflicts {
        result = result
            || rule_type == SolverRuleinfo_SOLVER_RULE_PKG_CONFLICTS
            || rule_type == SolverRuleinfo_SOLVER_RULE_PKG_SELF_CONFLICT;
    }
    if skip_problem.obsoletes {
        result = result
            || rule_type == SolverRuleinfo_SOLVER_RULE_PKG_OBSOLETES
            || rule_type == SolverRuleinfo_SOLVER_RULE_PKG_IMPLICIT_OBSOLETES
            || rule_type == SolverRuleinfo_SOLVER_RULE_PKG_INSTALLED_OBSOLETES;
    }
    if skip_problem.disabled {
        if rule_type == SolverRuleinfo_SOLVER_RULE_PKG_NOT_INSTALLABLE {
            let s = unsafe { pool_id2solvable((*solv).pool, source) };
            if unsafe { pool_disabled_solvable((*solv).pool, s) } {
                result = true;
            };
        }
    }
    result
}

impl Solvsack {
    pub fn solv_count_pkg_by_name(&self, pkg: &str) -> Result<u32> {
        let mut p_query = SolvQuery::default(self.clone());
        p_query.solv_apply_single_pkg_filter(pkg)?;
        p_query.solv_apply_list_query()?;
        let pkg_list = p_query.solv_get_query_result()?;
        Ok(pkg_list.get_size())
    }
    pub fn solv_find_all_installed(&self) -> Result<SolvPackageList> {
        let mut p_query = SolvQuery::default(self.clone());
        p_query.solv_add_system_repo_filter()?;
        p_query.solv_apply_list_query()?;
        let pkgs = p_query.solv_get_query_result()?;
        Ok(pkgs)
    }
    pub fn solv_find_installed_pkg_by_multiple_names(
        &self,
        pkg_names: Vec<String>,
    ) -> Result<SolvPackageList> {
        let mut p_query = SolvQuery::default(self.clone());
        p_query.solv_add_system_repo_filter()?;
        p_query.package_names = Some(pkg_names.clone());
        p_query.solv_apply_list_query()?;
        let pkgs = p_query.solv_get_query_result()?;
        Ok(pkgs)
    }
    pub fn solv_find_installed_pkg_by_name(&self, pkg_name: &str) -> Result<SolvPackageList> {
        let mut p_query = SolvQuery::default(self.clone());
        p_query.solv_add_system_repo_filter()?;
        p_query.solv_apply_single_pkg_filter(pkg_name)?;
        p_query.solv_apply_list_query()?;
        let pkgs = p_query.solv_get_query_result()?;
        Ok(pkgs)
    }
    pub fn solv_get_pkg_name_by_id(&self, id: i32) -> Result<String> {
        let p_solv = pool_id2solvable(self.pool, id);
        if p_solv.is_null() {
            bail!(ERROR_RDNF_NO_DATA);
        }
        unsafe {
            let psz_temp = pool_id2str(self.pool, (*p_solv).name);
            if psz_temp.is_null() {
                bail!(ERROR_RDNF_NO_DATA)
            }
            Ok(CStr::from_ptr(psz_temp).to_str().unwrap().to_string())
        }
    }
    pub fn solv_get_pkg_nevr_by_id(&self, id: i32) -> Result<String> {
        let p_solv = pool_id2solvable(self.pool, id);
        if p_solv.is_null() {
            bail!(ERROR_RDNF_NO_DATA);
        }
        unsafe {
            let psz_temp = pool_solvable2str(self.pool, p_solv);
            if psz_temp.is_null() {
                bail!(ERROR_RDNF_NO_DATA)
            }
            Ok(CStr::from_ptr(psz_temp).to_str().unwrap().to_string())
        }
    }
    pub fn solv_find_available_pkg_by_name(&self, pkg_name: &str) -> Result<SolvPackageList> {
        let mut p_query = SolvQuery::default(self.clone());
        p_query.solv_add_available_repo_filter()?;
        p_query.solv_apply_single_pkg_filter(pkg_name)?;
        p_query.solv_apply_list_query()?;
        Ok(p_query.solv_get_query_result()?)
    }
    pub fn solv_find_highest_available(&self, pkg_name: &str) -> Result<i32> {
        let pkg_list = self.solv_find_available_pkg_by_name(pkg_name)?;
        let mut highest_available = pkg_list.get_pkg_id(0);
        let count = pkg_list.get_size();
        for index in 1..count {
            let id = pkg_list.get_pkg_id(index);
            if self.solv_cmp_evr(id, highest_available)? > 0 {
                highest_available = id;
            };
        }
        Ok(highest_available)
    }
    pub fn solv_find_highest_or_lowest_installed(
        &self,
        pkg_name: &str,
        is_higher: bool,
    ) -> Result<i32> {
        let installed_pkg_list = self.solv_find_installed_pkg_by_name(pkg_name)?;
        let mut high_or_low = installed_pkg_list.get_pkg_id(0);
        if high_or_low != 0 {
            let count = installed_pkg_list.get_size();
            for index in 1..count {
                let id = installed_pkg_list.get_pkg_id(index);
                let cmp = self.solv_cmp_evr(id, high_or_low)?;
                match is_higher {
                    true => {
                        if cmp > 0 {
                            high_or_low = id;
                        }
                    }
                    false => {
                        if cmp < 0 {
                            high_or_low = id;
                        }
                    }
                }
            }
        }
        Ok(high_or_low)
    }
    pub fn solv_find_highest_installed(&self, pkg_name: &str) -> Result<i32> {
        Ok(self.solv_find_highest_or_lowest_installed(pkg_name, true)?)
    }
    pub fn solv_find_lowest_installed(&self, pkg_name: &str) -> Result<i32> {
        Ok(self.solv_find_highest_or_lowest_installed(pkg_name, false)?)
    }
    pub fn solv_cmp_evr(&self, id1: i32, id2: i32) -> Result<i32> {
        let pool = self.pool;
        let solv1 = pool_id2solvable(pool, id1);
        let solv2 = pool_id2solvable(pool, id2);
        if solv1.is_null() || solv2.is_null() {
            bail!(ERROR_RDNF_INVALID_PARAMETER);
        }
        unsafe {
            let evr1 = solvable_lookup_str(solv1, solv_knownid_SOLVABLE_EVR as i32);
            let evr2 = solvable_lookup_str(solv2, solv_knownid_SOLVABLE_EVR as i32);
            let p = pool_evrcmp_str(pool, evr1, evr2, EVRCMP_COMPARE as i32);
            Ok(p)
        }
    }
    pub fn solv_report_problems(&self, solv: *mut Solver, skip_problem: SkipProblem) -> Result<()> {
        let mut count = unsafe { solver_problem_count(solv) };
        let mut source = 0;
        let mut target = 0;
        let mut dep = 0;
        let mut prv_pkg_name = "";
        let mut error = "";
        let mut total_problems = 0;
        while count > 0 {
            let problem_id = unsafe { solver_findproblemrule(solv, count as i32) };
            let rule_type =
                unsafe { solver_ruleinfo(solv, problem_id, &mut source, &mut target, &mut dep) };
            if skip_based_on_type(solv, rule_type, source, skip_problem) {
                count -= 1;
                continue;
            };
            let psz_problem =
                unsafe { solver_problemruleinfo2str(solv, rule_type, source, target, dep) };
            let problem = unsafe { CStr::from_ptr(psz_problem).to_str().unwrap() };
            if !skip_problem.none && rule_type == SolverRuleinfo_SOLVER_RULE_PKG_REQUIRES {
                let (_, beg) = problem.split_once("requires").unwrap();
                let (beg, _) = beg.split_once(",").unwrap();
                let pkg_name = beg.trim_start().trim_end();
                if pkg_name == prv_pkg_name {
                    continue;
                }
                prv_pkg_name = pkg_name;
                match self.solv_find_available_pkg_by_name(pkg_name) {
                    Ok(_) => {
                        continue;
                    }
                    Err(_) => {}
                };
            }
            error = ERROR_RDNF_SOLV_FAILED;
            total_problems += 1;
            println!("{}. {}", total_problems, problem);
        }
        if error != "" {
            bail!("Found {} problem(s) while resolving", total_problems);
        }
        Ok(())
    }
    pub fn solv_get_pkginfo_base_by_id(&self, pkg_id: i32) -> Result<PkgInfoBase> {
        let solv = pool_id2solvable(self.pool, pkg_id);
        if solv.is_null() {
            bail!(ERROR_RDNF_NO_DATA);
        }
        let name = c_str_ptr_to_rust_string(unsafe {
            solvable_lookup_str(solv, solv_knownid_SOLVABLE_NAME as i32)
        })
        .unwrap_or("".to_string());
        let arch = c_str_ptr_to_rust_string(unsafe {
            solvable_lookup_str(solv, solv_knownid_SOLVABLE_ARCH as i32)
        })
        .unwrap_or("".to_string());
        let evr = c_str_ptr_to_rust_string(unsafe {
            solvable_lookup_str(solv, solv_knownid_SOLVABLE_EVR as i32)
        })
        .unwrap_or("".to_string());
        let (epoch, version, release) = solv_split_evr(evr.as_str());
        let mut dw_epoch = 0;
        if epoch != "" {
            unsafe {
                let epoch_c = CString::new(epoch).unwrap();
                dw_epoch = strtol(epoch_c.as_ptr(), 0 as *mut *mut i8, 10);
            }
        };
        let repo_name_ptr = unsafe { (*(*solv).repo).name };
        if repo_name_ptr.is_null() {
            bail!(ERROR_RDNF_NO_DATA);
        };
        let repo_name = unsafe { CStr::from_ptr(repo_name_ptr).to_str()? }.to_string();
        Ok(PkgInfoBase {
            epoch: dw_epoch as u32,
            name: name.to_string(),
            version: version.to_string(),
            release: release.to_string(),
            arch,
            evr,
            repo_name,
        })
    }
    pub fn solv_get_pkginfo_details_by_id(&self, pkg_id: i32) -> Result<PkgInfoDetails> {
        let solv = pool_id2solvable(self.pool, pkg_id);
        if solv.is_null() {
            bail!(ERROR_RDNF_NO_DATA);
        };
        let summary = c_str_ptr_to_rust_string(unsafe {
            solvable_lookup_str(solv, solv_knownid_SOLVABLE_SUMMARY as i32)
        })
        .unwrap_or("".to_string());
        let location =
            c_str_ptr_to_rust_string(unsafe { solvable_get_location(solv, 0 as *mut u32) });
        let install_size =
            unsafe { solvable_lookup_num(solv, solv_knownid_SOLVABLE_INSTALLSIZE as i32, 0) };
        let formatted_size = format_size(install_size);
        Ok(PkgInfoDetails {
            install_size,
            formatted_size,
            summary,
            location,
        })
    }
    pub fn solv_get_pkginfo_other_by_id(&self, pkg_id: i32) -> Result<PkgInfoOther> {
        let solv = pool_id2solvable(self.pool, pkg_id);
        let url = c_str_ptr_to_rust_string(unsafe {
            solvable_lookup_str(solv, solv_knownid_SOLVABLE_URL as i32)
        })
        .unwrap_or("None".to_string());
        let license=c_str_ptr_to_rust_string(unsafe {
            solvable_lookup_str(solv, solv_knownid_SOLVABLE_LICENSE as i32)
        }).unwrap_or("None".to_string());
        let description=c_str_ptr_to_rust_string(unsafe {
            solvable_lookup_str(solv, solv_knownid_SOLVABLE_DESCRIPTION as i32)
        }).unwrap_or("None".to_string());
        Ok(PkgInfoOther {
            url,
            license,
            description,
        })
    }
    pub fn solv_get_pkginfo_by_id(&self, pkg_id: i32, which_info: i32) -> Result<&str> {
        let solv = pool_id2solvable(self.pool, pkg_id);
        Ok(unsafe { CStr::from_ptr(solvable_lookup_str(solv, which_info)).to_str()? })
    }
    pub fn solv_get_pkg_location_by_id(&self, pkg_id: i32) -> Result<&str> {
        let solv = pool_id2solvable(self.pool, pkg_id);
        Ok(unsafe { CStr::from_ptr(solvable_get_location(solv, 0 as *mut u32)).to_str()? })
    }
    pub fn solv_get_pkg_install_size_by_id(&self, pkg_id: i32) -> Result<u64> {
        let solv = pool_id2solvable(self.pool, pkg_id);
        Ok(unsafe { solvable_lookup_num(solv, solv_knownid_SOLVABLE_INSTALLSIZE as i32, 0) })
    }
}
impl SolvQuery {
    pub fn solv_get_query_result(&self) -> Result<SolvPackageList> {
        if self.queue_result.count == 0 {
            bail!(ERROR_RDNF_NO_MATCH);
        }
        let mut solv_pkgs_list = SolvPackageList { pkgs: init_queue() };
        unsafe {
            queue_insertn(
                &mut solv_pkgs_list.pkgs as *mut Queue,
                solv_pkgs_list.pkgs.count,
                self.queue_result.count,
                self.queue_result.elements,
            );
        }
        Ok(solv_pkgs_list)
    }
}
impl SolvPackageList {
    pub fn get_pkg_id(&self, index: u32) -> i32 {
        let p = &self.pkgs as *const Queue;
        get_queue_element_value(p, index)
    }
    pub fn queue_to_pkg_list(queue: &mut Queue) -> Result<Self> {
        if queue.count == 0 {
            bail!(ERROR_RDNF_NO_MATCH)
        };
        let mut solv_pkgs_list = SolvPackageList { pkgs: init_queue() };
        unsafe {
            queue_insertn(
                &mut solv_pkgs_list.pkgs,
                solv_pkgs_list.pkgs.count,
                queue.count,
                queue.elements,
            );
        };
        Ok(solv_pkgs_list)
    }
}
pub fn solv_add_excludes(pool: *mut Pool, excludes: &Vec<String>) {
    let mut excludes_map = unsafe { init_map((*pool).nsolvables) };
    solv_data_iterator(pool, excludes, &mut excludes_map);
    unsafe {
        if (*pool).considered.is_null() {
            (*pool).considered = libc::malloc(mem::size_of::<Map>()) as *mut Map;
            map_init((*pool).considered, (*pool).nsolvables);
        } else {
            map_grow((*pool).considered, (*pool).nsolvables);
        }
        map_setall((*pool).considered);
        map_subtract((*pool).considered, &excludes_map as *const Map);
    }
}
pub fn solv_data_iterator(pool: *mut Pool, excludes: &Vec<String>, map: &mut Map) {
    let mut di = create_dataiterator_empty();
    let di_ptr = &mut di as *mut Dataiterator;
    let keyname = solv_knownid_SOLVABLE_NAME;
    for ele in excludes {
        let mut flags = SEARCH_STRING;
        if is_glob(ele.as_str()) {
            flags = SEARCH_GLOB;
        };

        unsafe {
            let temp = CString::new(ele.as_str()).unwrap();
            dataiterator_init(
                di_ptr,
                pool,
                0 as *mut Repo,
                0,
                keyname as i32,
                temp.as_ptr(),
                flags as i32,
            );
            while dataiterator_step(di_ptr) != 0 {
                map_set(map as *mut Map, di.solvid);
            }
        }
    }
}
pub fn init_map(n: i32) -> Map {
    let mut map = Map {
        map: CString::new("").unwrap().into_raw() as *mut u8,
        size: 0,
    };
    unsafe {
        map_init(&mut map as *mut Map, n);
    };
    map
}
pub fn solv_split_evr(evr: &str) -> (&str, &str, &str) {
    let (evr, rest) = match evr.split_once(":") {
        Some(s) => s,
        None => ("", evr),
    };
    let (version, release) = match rest.split_once("-") {
        Some(s) => s,
        None => ("", rest),
    };
    (evr, version, release)
}
