use std::ffi::{CStr, CString};

use solv_sys::ffi::{
    queue_init, queue_insertn, selection_filter, selection_make, Queue, Solver, SELECTION_CANON,
    SELECTION_DOTARCH, SELECTION_FILELIST, SELECTION_GLOB, SELECTION_NAME, SELECTION_NOCASE,
    SELECTION_PROVIDES, SELECTION_REL, SOLVER_DISTUPGRADE, SOLVER_SELECTMASK,
    SOLVER_SETREPO, SOLVER_SETVENDOR, SOLVER_SOLVABLE, SOLVER_SOLVABLE_ALL, SOLVER_SOLVABLE_NAME,
    SOLVER_SOLVABLE_ONE_OF, SOLVER_SOLVABLE_REPO, 
};

use crate::{
    c_lib::{
        get_pool_solvables_value, get_pool_whatprovidesdata_value, get_queue_element_value,
        is_pseudo_package, pool_id2repo, pool_id2solvable, pool_match_nevr, pool_whatprovides,
        queue_empty, queue_push, queue_push2,
    },
    default::SYSTEM_REPO_NAME,
    errors::ERROR_RDNF_INVALID_PARAMETER,
};

use super::{sack::Solvsack, SolvPackageList};
use anyhow::{bail, Result};
pub struct SolvQuery {
    pub sack: Solvsack,
    pub queue_job: Queue,
    pub p_solv: Option<*mut Solver>,
    pub queue_repo_filter: Queue,
    pub package_names: Option<Vec<String>>,
    pub queue_result: Queue,
    pub dw_new_packages: Option<usize>,
    // pub scope: Option<RdnfScope>,
}
impl SolvQuery {
    pub fn default(solv_sack: Solvsack) -> Self {
        SolvQuery {
            sack: solv_sack,
            queue_job: init_queue(),
            p_solv: None,
            queue_repo_filter: init_queue(),
            package_names: None,
            queue_result: init_queue(),
            dw_new_packages: None,
            // scope: None,
        }
    }
    // pub fn solv_apply_pkg_filter(&mut self,pkg_names:&Vec<String>)->Result<()>{
    //     let p=pkg_names.clone();
    //     self.package_names=Some(p);
    //     Ok(())
    // }
    pub fn solv_apply_list_query(&mut self) -> Result<()> {
        let mut queue_temp = init_queue();
        let queue_temp_ptr = &mut queue_temp as *mut Queue;
        let flags = SELECTION_NAME | SELECTION_PROVIDES | SELECTION_GLOB;
        let flags = flags | (SELECTION_CANON | SELECTION_DOTARCH | SELECTION_REL);
        self.solv_generate_common_job(flags)?;
        // let scope = self.scope.clone();
        // if scope.as_ref().is_some() && scope.as_ref().unwrap().is_upgrades() {
            // self.solv_apply_up_down_scope(true);
            // todo!();
        // } else if scope.as_ref().is_some() && scope.as_ref().unwrap().is_down_grades() {
        // } else 
        if self.queue_job.count > 0 {
            let mut index: u32 = 0;
            while index < self.queue_job.count as u32 {
                queue_empty(queue_temp_ptr);
                let what =
                    get_queue_element_value(&mut self.queue_job as *mut Queue, index + 1 as u32);
                let how = SOLVER_SELECTMASK
                    & get_queue_element_value(&mut self.queue_job as *mut Queue, index) as u32;
                let pool = self.sack.pool;
                if how == SOLVER_SOLVABLE_ALL {
                    let mut p = 2;
                    unsafe {
                        while p < (*pool).nsolvables {
                            let solvable_item = get_pool_solvables_value(pool, p as u32);
                            if !(*solvable_item).repo.is_null()
                                && !is_pseudo_package(pool, solvable_item)
                            {
                                queue_push(queue_temp_ptr, p);
                            };
                            p += 1;
                        }
                    }
                } else if how == SOLVER_SOLVABLE_REPO {
                    let repo = pool_id2repo(pool, what);
                    if !repo.is_null() {
                        unsafe {
                            let mut p = (*repo).start;
                            let mut s = pool_id2solvable((*repo).pool, p);
                            while p < (*repo).end {
                                let solvable_item = get_pool_solvables_value(pool, p as u32);
                                if (*s).repo == repo && !is_pseudo_package(pool, solvable_item) {
                                    queue_push(queue_temp_ptr, p);
                                };
                                p += 1;
                                s = pool_id2solvable((*repo).pool, p);
                            }
                        }
                    }
                } else {
                    let mut pp = if how == SOLVER_SOLVABLE {
                        0
                    } else {
                        if how == SOLVER_SOLVABLE_ONE_OF {
                            what
                        } else {
                            pool_whatprovides(pool, what)
                        }
                    };
                    let mut p = if how == SOLVER_SOLVABLE {
                        what
                    } else {
                        get_pool_whatprovidesdata_value(pool, pp)
                    };
                    pp += 1;
                    while p != 0 {
                        let s = pool_id2solvable(pool, p);
                        let solvable_item = get_pool_solvables_value(pool, p as u32);
                        if !(how == SOLVER_SOLVABLE_NAME && pool_match_nevr(pool, s, what) == 0)
                            && !(is_pseudo_package(pool, solvable_item))
                        {
                            queue_push(queue_temp_ptr, p);
                        }
                        p = get_pool_whatprovidesdata_value(pool, pp);
                        pp += 1;
                    }
                }
                unsafe {
                    queue_insertn(
                        &mut self.queue_result as *mut Queue,
                        self.queue_result.count,
                        queue_temp.count,
                        queue_temp.elements,
                    );
                }
                index += 2;
            }
        } else if self.package_names.is_none() {
            let pool = self.sack.pool;
            // let mut p = 2;
            unsafe {
                // while p < (*pool).nsolvables {
                //     let solvable_item = get_pool_solvables_value(pool, p as u32);
                //     if !(*solvable_item).repo.is_null() && !is_pseudo_package(pool, solvable_item) {
                //         queue_push(queue_temp_ptr, p);
                //     };
                //     p += 1;
                // }
                for id in 2..(*pool).nsolvables {
                    let solvable_item = get_pool_solvables_value(pool, id as u32);
                    if !(*solvable_item).repo.is_null() && !is_pseudo_package(pool, solvable_item) {
                        queue_push(&mut self.queue_result, id);
                    };
                }
            }
        }
        Ok(())
    }
    // pub fn solv_apply_up_down_scope(&mut self, up: bool) -> Result<()> {
    //     let p = if self.package_names.is_none() {
    //         self.sack.solv_find_all_installed()?
    //     } else {
    //         let p = self.package_names.clone().unwrap();
    //         self.sack.solv_find_installed_pkg_by_multiple_names(p)?
    //     };
    //     Ok(())
    // }
    pub fn solv_add_system_repo_filter(&mut self) -> Result<()> {
        let pool = self.sack.pool;
        unsafe {
            queue_push2(
                &mut self.queue_repo_filter as *mut Queue,
                (SOLVER_SOLVABLE_REPO | SOLVER_SETREPO) as i32,
                (*(*pool).installed).repoid,
            );
        }
        Ok(())
    }
    pub fn solv_add_available_repo_filter(&mut self) -> Result<()> {
        let pool = self.sack.pool;
        unsafe {
            for repoid in 1..(*pool).nrepos{
                let repo = pool_id2repo(pool, repoid);
                if !repo.is_null() {
                    let repo_name = CStr::from_ptr((*repo).name).to_str()?;
                    if SYSTEM_REPO_NAME.to_lowercase() != repo_name.to_lowercase() {
                        queue_push2(
                            &mut self.queue_repo_filter as *mut Queue,
                            (SOLVER_SOLVABLE_REPO | SOLVER_SETREPO | SOLVER_SETVENDOR) as i32,
                            (*repo).repoid,
                        );
                    }
                }
            }
        }
        Ok(())
    }
    pub fn solv_apply_single_pkg_filter(&mut self, pkg_name: &str) -> Result<()> {
        self.package_names = Some(vec![pkg_name.to_string()]);
        Ok(())
    }
    pub fn solv_generate_common_job(&mut self, select_flags:u32) -> Result<u32> {
        let mut queue_job = init_queue();
        let queue_job_ptr = &mut queue_job as *mut Queue;
        let pool = self.sack.pool;
        match &self.package_names {
            Some(pkgs) => {
                for pkg in pkgs {
                    // let mut ret_flags = 0;
                    let mut flags = select_flags;
                    unsafe {
                        queue_empty(queue_job_ptr);
                        if pool.is_null()
                            || (*pool).solvables.is_null()
                            || (*pool).whatprovides.is_null()
                        {
                            bail!(ERROR_RDNF_INVALID_PARAMETER);
                        }
                        let pkg_ptr = CString::new(pkg.as_str()).unwrap();
                        let mut ret_flags =
                            selection_make(pool, queue_job_ptr, pkg_ptr.as_ptr(), flags as i32);
                        if self.queue_repo_filter.count != 0 {
                            selection_filter(
                                pool,
                                queue_job_ptr,
                                &mut self.queue_repo_filter as *mut Queue,
                            );
                        }
                        if queue_job.count == 0 {
                            flags = flags | SELECTION_NOCASE;
                            ret_flags =
                                selection_make(pool, queue_job_ptr, pkg_ptr.as_ptr(), flags as i32);
                            if self.queue_repo_filter.count != 0 {
                                selection_filter(
                                    pool,
                                    queue_job_ptr,
                                    &mut self.queue_repo_filter as *mut Queue,
                                );
                            }
                            if queue_job.count != 0 {
                                println!("[ignoring case for {}]", pkg.as_str());
                            }()
                        }
                        if queue_job.count != 0 {
                            if (ret_flags & SELECTION_FILELIST as i32) != 0 {
                                println!("[using file list match for {}]", pkg.as_str());
                            }
                            if (ret_flags & SELECTION_PROVIDES as i32) != 0 {
                                println!("[using capability match for {}]", pkg.as_str());
                            }
                            queue_insertn(
                                &mut self.queue_job as *mut Queue,
                                self.queue_job.count,
                                queue_job.count,
                                queue_job.elements,
                            );
                        }
                    }
                }
            }
            None => {
                if self.queue_repo_filter.count != 0 {
                    queue_empty(queue_job_ptr);
                    queue_push2(queue_job_ptr, SOLVER_SOLVABLE_ALL as i32, 0);
                    if self.queue_repo_filter.count != 0 {
                        unsafe {
                            selection_filter(
                                pool,
                                queue_job_ptr,
                                &mut self.queue_repo_filter as *mut Queue,
                            );
                            queue_insertn(
                                &mut self.queue_job as *mut Queue,
                                self.queue_job.count,
                                queue_job.count,
                                queue_job.elements,
                            );
                        }
                    }
                }
            }
        }
        Ok(0)
    }
}
pub fn init_queue() -> Queue {
    let mut queue = Queue {
        elements: &mut 0 as *mut i32,
        count: 0,
        alloc: &mut 0 as *mut i32,
        left: 0,
    };
    unsafe {
        queue_init(&mut queue as *mut Queue);
    };
    queue
}
pub fn _solv_add_dist_upgrade_job(queue_job: &mut Queue) -> Result<()> {
    queue_push2(
        queue_job as *mut Queue,
        (SOLVER_DISTUPGRADE | SOLVER_SOLVABLE_ALL) as i32,
        0,
    );
    Ok(())
}

impl Solvsack {
    pub fn solv_find_all_up_down_candidates(
        &mut self,
        installed_pkgs: &SolvPackageList,
        _up: bool,
        _queue_result: &Queue,
    ) -> Result<()> {
        let _queue_up_down = init_queue();
        let dw_size = installed_pkgs.get_size();
        for index in 0..dw_size {
            let _id = installed_pkgs.get_pkg_id(index as u32);
        }
        Ok(())
    }
}
