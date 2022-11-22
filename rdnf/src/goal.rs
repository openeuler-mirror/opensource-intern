use std::{ffi::CString, mem::size_of};

use crate::{
    c_lib::{
        create_dataiterator_empty, get_queue_element_value, map_set, map_setall, pool_id2solvable,
        queue_push, queue_push2, solv_add_flags_to_jobs,
    },
    cli::AlterOption,
    errors::{
        ERROR_RDNF_ALREADY_EXISTS, ERROR_RDNF_INVALID_PARAMETER, ERROR_RDNF_INVALID_RESOLVE_ARG,
        ERROR_RDNF_OUT_OF_MEMORY,
    },
    solv::{
        rdnf_pkg::{init_map, solv_add_excludes, SkipProblem},
        rdnf_query::{
            init_queue, 
        },
        SolvPackageList,
    },
    sub_command::{install::{is_glob, AlterType}, info::{PkgInfo, PkgInfoLevel}},
    Rdnf,
};
use anyhow::{bail, Result};
use glob::Pattern;

use solv_sys::ffi::{
    dataiterator_free, dataiterator_init, dataiterator_step, map_grow, map_init, map_subtract,
    pool_evrcmp_str, s_Map, s_Solver, solv_knownid_SOLVABLE_EVR, solv_knownid_SOLVABLE_NAME,
    solvable_lookup_str, solver_create, solver_create_transaction, solver_get_unneeded,
    solver_set_flag, solver_solve, testcase_write, transaction_type, Queue, Repo, Transaction,
    EVRCMP_COMPARE, SEARCH_STRING, SOLVER_ERASE, SOLVER_FLAG_ALLOW_DOWNGRADE,
    SOLVER_FLAG_ALLOW_UNINSTALL, SOLVER_FLAG_ALLOW_VENDORCHANGE, SOLVER_FLAG_BEST_OBEY_POLICY,
    SOLVER_FLAG_INSTALL_ALSO_UPDATES, SOLVER_FLAG_KEEP_ORPHANS, SOLVER_FLAG_YUM_OBSOLETES,
    SOLVER_FORCEBEST, SOLVER_INSTALL, SOLVER_SOLVABLE, SOLVER_SOLVABLE_ALL,
    SOLVER_TRANSACTION_DOWNGRADE, SOLVER_TRANSACTION_ERASE, SOLVER_TRANSACTION_INSTALL,
    SOLVER_TRANSACTION_OBSOLETED, SOLVER_TRANSACTION_REINSTALL, SOLVER_TRANSACTION_SHOW_ACTIVE,
    SOLVER_TRANSACTION_SHOW_ALL, SOLVER_TRANSACTION_SHOW_OBSOLETES, SOLVER_TRANSACTION_UPGRADE,
    SOLVER_UPDATE, TESTCASE_RESULT_PROBLEMS, TESTCASE_RESULT_TRANSACTION,
};
impl Rdnf {
    pub fn get_pkgs_with_specified_type(
        &self,
        trans: *mut Transaction,
        dw_type: i32,
    ) -> Result<Option<Vec<PkgInfo>>> {
        let mut solved_pkgs = init_queue();
        unsafe {
            for i in 0..(*trans).steps.count {
                let pkg = get_queue_element_value(&mut (*trans).steps, i as u32);
                let pkg_type = if dw_type == SOLVER_TRANSACTION_OBSOLETED as i32 {
                    transaction_type(trans, pkg, SOLVER_TRANSACTION_SHOW_OBSOLETES as i32)
                } else {
                    transaction_type(
                        trans,
                        pkg,
                        (SOLVER_TRANSACTION_SHOW_ACTIVE | SOLVER_TRANSACTION_SHOW_ALL) as i32,
                    )
                };
                if dw_type == pkg_type {
                    queue_push(&mut solved_pkgs, pkg);
                }
            }
        }
        let pkg_list = match SolvPackageList::queue_to_pkg_list(&mut solved_pkgs) {
            Ok(s) => Some(s),
            Err(_) => None,
        };
        if pkg_list.is_some() {
            if pkg_list.as_ref().unwrap().get_size() > 0 {
                let pkg_infos = PkgInfo::populate_pkg_info(&self.rc.sack, &pkg_list.unwrap(),PkgInfoLevel::Details)?;
                if pkg_infos.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(pkg_infos))
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    pub fn goal(
        &self,
        pkg_list: &mut Queue,
        alter_type: AlterType,
        aler_args: &AlterOption,
    ) -> Result<SolvedPkgInfoBase> {
        let excludes = self.pkgs_to_exclude()?;
        let mut queue_job = init_queue();
        if alter_type.is_upgrade_all() {
            queue_push2(
                &mut queue_job,
                (SOLVER_UPDATE | SOLVER_SOLVABLE_ALL) as i32,
                0,
            );
        } else if alter_type.is_distro_sync() {
        } else {
            if pkg_list.count == 0 {
                bail!(ERROR_RDNF_ALREADY_EXISTS);
            }
            for i in 0..pkg_list.count {
                let id = get_queue_element_value(pkg_list as *const Queue, i as u32);
                self.add_goal(alter_type.clone(), &mut queue_job, id, &excludes)?;
            }
        }
        let mut flags = 0;
        if aler_args.best {
            flags = flags | SOLVER_FORCEBEST;
        }
        solv_add_flags_to_jobs(&mut queue_job as *mut Queue, flags as i32);

        if !excludes.is_empty() {
            solv_add_excludes(self.rc.sack.pool, &excludes);
        }
        self.solv_add_min_version();
        let solv = unsafe { solver_create(self.rc.sack.pool) };
        if solv.is_null() {
            bail!(ERROR_RDNF_OUT_OF_MEMORY);
        }
        if aler_args.allow_erasing || alter_type.is_erase() || alter_type.is_auto_erase() {
            unsafe {
                solver_set_flag(solv, SOLVER_FLAG_ALLOW_UNINSTALL as i32, 1);
            };
        }
        let n_problems = unsafe {
            solver_set_flag(solv, SOLVER_FLAG_BEST_OBEY_POLICY as i32, 1);
            solver_set_flag(solv, SOLVER_FLAG_ALLOW_VENDORCHANGE as i32, 1);
            solver_set_flag(solv, SOLVER_FLAG_KEEP_ORPHANS as i32, 1);
            solver_set_flag(solv, SOLVER_FLAG_BEST_OBEY_POLICY as i32, 1);
            solver_set_flag(solv, SOLVER_FLAG_YUM_OBSOLETES as i32, 1);
            solver_set_flag(solv, SOLVER_FLAG_ALLOW_DOWNGRADE as i32, 1);
            solver_set_flag(solv, SOLVER_FLAG_INSTALL_ALSO_UPDATES as i32, 1);
            solver_solve(solv, &mut queue_job)
        };
        if n_problems > 0 {
            let mut skip_problem = self.get_skip_problem_opt(aler_args);
            if alter_type.is_upgrade() && !excludes.is_empty() {
                skip_problem.disabled = true;
            }
            self.rc.sack.solv_report_problems(solv, skip_problem)?;
        }
        let p_trans = unsafe { solver_create_transaction(solv) };
        if p_trans.is_null() {
            bail!(ERROR_RDNF_INVALID_PARAMETER);
        }
        if aler_args.debug_solver {
            let result_flags = TESTCASE_RESULT_TRANSACTION | TESTCASE_RESULT_PROBLEMS;
            unsafe {
                let dir = CString::new("debugdata").unwrap();
                if testcase_write(
                    solv,
                    dir.as_ptr(),
                    result_flags as i32,
                    0 as *mut i8,
                    0 as *mut i8,
                ) == 0
                {
                    println!("Could not write debugdata to folder {}", "debugdata")
                }
            };
        }
        Ok(self.goal_get_all_results_ignore_no_data(alter_type, p_trans, solv)?)
    }
    pub fn add_goal(
        &self,
        alter_type: AlterType,
        queue_job: &mut Queue,
        id: i32,
        excludes: &Vec<String>,
    ) -> Result<()> {
        if !excludes.is_empty() {
            let pkg_name = self.rc.sack.solv_get_pkg_name_by_id(id)?;
            for ele in excludes {
                if is_glob(ele.as_str()) {
                    let p = Pattern::new(ele.as_str())?;
                    if p.matches(pkg_name.as_str()) {
                        return Ok(());
                    };
                } else {
                    if ele == pkg_name.as_str() {
                        return Ok(());
                    }
                }
            }
        }
        match alter_type {
            AlterType::DownGrade | AlterType::DownGradeAll => {
                queue_push2(queue_job, (SOLVER_SOLVABLE | SOLVER_INSTALL) as i32, id);
            }
            AlterType::AutoErase | AlterType::Erase => {
                queue_push2(queue_job, (SOLVER_SOLVABLE | SOLVER_ERASE) as i32, id);
            }
            AlterType::ReInstall | AlterType::Install | AlterType::Upgrade => {
                queue_push2(queue_job, (SOLVER_SOLVABLE | SOLVER_INSTALL) as i32, id);
            }
            // AlterType::AutoErase => {
            //     queue_push2(
            //         queue_job,
            //         (SOLVER_SOLVABLE | SOLVER_USERINSTALLED) as i32,
            //         id,
            //     );
            // }
            _ => {
                bail!(ERROR_RDNF_INVALID_RESOLVE_ARG)
            }
        }
        Ok(())
    }
    pub fn solv_add_min_version(&self) {
        if self.rc.conf.minversions.is_some() {
            let pool = self.rc.sack.pool;
            let mut map_versins = unsafe { init_map((*pool).nsolvables) };
            for ele in self.rc.conf.minversions.as_ref().unwrap() {
                let (name, ver) = ele.split_once("=").unwrap();
                let mut di = create_dataiterator_empty();
                unsafe {
                    let m = CString::new(name).unwrap();
                    dataiterator_init(
                        &mut di,
                        pool,
                        0 as *mut Repo,
                        0,
                        solv_knownid_SOLVABLE_NAME as i32,
                        m.as_ptr(),
                        SEARCH_STRING as i32,
                    );
                    while dataiterator_step(&mut di) != 0 {
                        let solv = pool_id2solvable(pool, di.solvid);
                        let evr = solvable_lookup_str(solv, solv_knownid_SOLVABLE_EVR as i32);
                        let evr2 = CString::new(ver).unwrap();
                        if pool_evrcmp_str(pool, evr, evr2.as_ptr(), EVRCMP_COMPARE as i32) < 0 {
                            map_set(&mut map_versins, di.solvid);
                        }
                    }
                    dataiterator_free(&mut di);
                }
            }
            unsafe {
                if (*pool).considered.is_null() {
                    (*pool).considered = libc::malloc(size_of::<s_Map>()) as *mut s_Map;
                    map_init((*pool).considered, (*pool).nsolvables);
                } else {
                    map_grow((*pool).considered, (*pool).nsolvables);
                }
                map_setall((*pool).considered);
                map_subtract((*pool).considered, &mut map_versins);
            }
        }
    }
    pub fn get_skip_problem_opt(&self, aler_args: &AlterOption) -> SkipProblem {
        let mut skip_problem = SkipProblem {
            none: false,
            conflicts: false,
            obsoletes: false,
            disabled: false,
        };
        // match self.rc.cli.command {
        //     crate::cli::Commands::Check => skip_problem.none = true,
        //     _ => {
                if aler_args.skip_confilicts {
                    skip_problem.conflicts = true;
                }
                if aler_args.skip_obsolete {
                    skip_problem.obsoletes = true;
                }
        //     }
        // };
        skip_problem
    }
    pub fn goal_get_all_results_ignore_no_data(
        &self,
        alter_type: AlterType,
        trans: *mut Transaction,
        solv: *mut s_Solver,
    ) -> Result<SolvedPkgInfoBase> {
        let mut solved_pkg_info = SolvedPkgInfoBase::default();
        solved_pkg_info.to_install =
            self.get_pkgs_with_specified_type(trans, SOLVER_TRANSACTION_INSTALL as i32)?;
        solved_pkg_info.to_upgrade =
            self.get_pkgs_with_specified_type(trans, SOLVER_TRANSACTION_UPGRADE as i32)?;
        solved_pkg_info.to_downgrade =
            self.get_pkgs_with_specified_type(trans, SOLVER_TRANSACTION_DOWNGRADE as i32)?;
        solved_pkg_info.removed_by_downgrade = if solved_pkg_info.to_downgrade.is_some() {
            let mut pkg_to_remove = init_queue();
            for pkg_info in solved_pkg_info.to_downgrade.as_ref().unwrap() {
                let pkg_id = self
                    .rc
                    .sack
                    .solv_find_installed_pkg_by_name(pkg_info.base.name.as_str())?
                    .get_pkg_id(0);
                queue_push(&mut pkg_to_remove, pkg_id);
            }
            if pkg_to_remove.count > 0 {
                let remove_pkg_list = SolvPackageList::queue_to_pkg_list(&mut pkg_to_remove)?;
                Some(PkgInfo::populate_pkg_info(&self.rc.sack, &remove_pkg_list,PkgInfoLevel::Details)?)
            } else {
                None
            }
        } else {
            None
        };
        solved_pkg_info.to_remove =
            self.get_pkgs_with_specified_type(trans, SOLVER_TRANSACTION_ERASE as i32)?;
        solved_pkg_info.un_needed = if alter_type.is_auto_erase() {
            let mut queue_result = init_queue();
            unsafe { solver_get_unneeded(solv, &mut queue_result, 0) };
            if queue_result.count > 0 {
                let pkg_list = SolvPackageList::queue_to_pkg_list(&mut queue_result)?;
                Some(PkgInfo::populate_pkg_info(&self.rc.sack, &pkg_list,PkgInfoLevel::Details)?)
            } else {
                None
            }
        } else {
            None
        };
        solved_pkg_info.to_reinstall =
            self.get_pkgs_with_specified_type(trans, SOLVER_TRANSACTION_REINSTALL as i32)?;
        solved_pkg_info.obsoleted =
            self.get_pkgs_with_specified_type(trans, SOLVER_TRANSACTION_OBSOLETED as i32)?;
        Ok(solved_pkg_info)
    }
}

#[derive(Debug)]
pub struct SolvedPkgInfo {
    pub need_action: u32,
    pub need_download: u32,
    pub not_available: Option<Vec<PkgInfo>>,
    pub existing: Option<Vec<PkgInfo>>,
    pub not_resolved: Vec<String>,
    pub not_installed: Option<Vec<String>>,
    pub base: SolvedPkgInfoBase,
}
#[derive(Debug)]
pub struct SolvedPkgInfoBase {
    pub to_install: Option<Vec<PkgInfo>>,
    pub to_downgrade: Option<Vec<PkgInfo>>,
    pub to_upgrade: Option<Vec<PkgInfo>>,
    pub to_remove: Option<Vec<PkgInfo>>,
    pub un_needed: Option<Vec<PkgInfo>>,
    pub to_reinstall: Option<Vec<PkgInfo>>,
    pub obsoleted: Option<Vec<PkgInfo>>,
    pub removed_by_downgrade: Option<Vec<PkgInfo>>,
}
impl SolvedPkgInfoBase {
    pub fn default() -> Self {
        SolvedPkgInfoBase {
            to_install: None,
            to_downgrade: None,
            to_upgrade: None,
            to_remove: None,
            un_needed: None,
            to_reinstall: None,
            obsoleted: None,
            removed_by_downgrade: None,
        }
    }
    pub fn get_need_action(&self) -> u32 {
        let mut action = 0;
        if self.to_install.is_some() {
            action += 1;
        }
        if self.to_upgrade.is_some() {
            action += 1;
        }
        if self.to_downgrade.is_some() {
            action += 1;
        }
        if self.to_remove.is_some() {
            action += 1;
        }
        if self.un_needed.is_some() {
            action += 1;
        }
        if self.to_reinstall.is_some() {
            action += 1;
        }
        if self.obsoleted.is_some() {
            action += 1;
        }
        action
    }
    pub fn get_need_download(&self) -> u32 {
        let mut download = 0;
        if self.to_install.is_some() {
            download += 1;
        }
        if self.to_upgrade.is_some() {
            download += 1;
        }
        if self.to_downgrade.is_some() {
            download += 1;
        }
        if self.to_reinstall.is_some() {
            download += 1;
        }
        download
    }
}
impl SolvedPkgInfo {
    pub fn default() -> Self {
        SolvedPkgInfo {
            need_action: 0,
            need_download: 0,
            not_available: None,
            existing: None,
            not_resolved: Vec::new(),
            not_installed: None,
            base: SolvedPkgInfoBase::default(),
        }
    }
}
