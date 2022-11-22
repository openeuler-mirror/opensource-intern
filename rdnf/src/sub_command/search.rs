use std::ffi::{CStr, CString};

use anyhow::{bail, Result};
use console::style;
use solv_sys::ffi::{
    dataiterator_free, dataiterator_set_keyname, dataiterator_step, queue_free, queue_insertn,
    selection_solvables, solv_knownid_SOLVABLE_ARCH, solv_knownid_SOLVABLE_DESCRIPTION,
    solv_knownid_SOLVABLE_NAME, solv_knownid_SOLVABLE_SUMMARY, solvable_lookup_str, Dataiterator,
    Queue, SEARCH_NOCASE, SEARCH_SUBSTRING, SOLVER_SOLVABLE,
};

use crate::{
    c_lib::{
        create_dataiterator_empty, dataiterator_init_simple, dataiterator_set_search_simple,
        get_queue_element_value, pool_id2solvable, queue_empty, queue_push2,
    },
    errors::{ERROR_RDNF_NO_DATA, ERROR_RDNF_NO_MATCH},
    solv::rdnf_query::{init_queue, SolvQuery},
    Rdnf,
};

impl Rdnf {
    pub fn search_pkg(&mut self, pkgs: Vec<String>) -> Result<()> {
        self.make_cache()?;
        let pkg_infos = SolvQuery::default(self.rc.sack.clone())
            .apply_search(pkgs)?
            .get_query_result()?;
        let offset = pkg_infos
            .iter()
            .max_by_key(|x| x.pkg_name.len())
            .unwrap()
            .pkg_name
            .len()
            + 5;
        let (_, width) = self.rc.term.size();
        for pkg in pkg_infos {
            self.rc
                .term
                .write_line(&format!("{}", style(pkg.pkg_name).green()))?;
            if offset < width as usize {
                self.rc.term.move_cursor_up(1)?;
            }
            self.rc.term.move_cursor_right(offset)?;
            self.rc
                .term
                .write_line(&format!("{}", style(pkg.pkg_summary).yellow()))?;
        }
        Ok(())
    }
}
impl SolvQuery {
    pub fn apply_search(mut self, pkgs: Vec<String>) -> Result<Self> {
        let mut queue_sel = init_queue();
        let mut queue_result = init_queue();
        let mut di = create_dataiterator_empty();
        let pool = self.sack.pool;
        unsafe {
            let di_ptr = &mut di as *mut Dataiterator;
            let queue_sel_ptr = &mut queue_sel as *mut Queue;
            let queue_result_ptr = &mut queue_result as *mut Queue;

            for pkg in pkgs {
                let pkg_ptr = CString::new(pkg.as_str()).unwrap();
                queue_empty(queue_sel_ptr);
                queue_empty(queue_result_ptr);
                dataiterator_init_simple(
                    di_ptr,
                    pool,
                    pkg_ptr.as_ptr(),
                    (SEARCH_SUBSTRING | SEARCH_NOCASE) as i32,
                );

                dataiterator_set_keyname(di_ptr, solv_knownid_SOLVABLE_NAME as i32);
                dataiterator_set_search_simple(di_ptr);
                while dataiterator_step(di_ptr) != 0 {
                    queue_push2(queue_sel_ptr, SOLVER_SOLVABLE as i32, di.solvid);
                }

                dataiterator_set_keyname(di_ptr, solv_knownid_SOLVABLE_SUMMARY as i32);
                dataiterator_set_search_simple(di_ptr);
                while dataiterator_step(di_ptr) != 0 {
                    queue_push2(queue_sel_ptr, SOLVER_SOLVABLE as i32, di.solvid);
                }

                dataiterator_set_keyname(di_ptr, solv_knownid_SOLVABLE_DESCRIPTION as i32);
                dataiterator_set_search_simple(di_ptr);
                while dataiterator_step(di_ptr) != 0 {
                    queue_push2(queue_sel_ptr, SOLVER_SOLVABLE as i32, di.solvid);
                }
                dataiterator_free(di_ptr);

                selection_solvables(pool, queue_sel_ptr, queue_result_ptr);
                let q = &mut self.queue_result as *mut Queue;
                queue_insertn(
                    q,
                    (*q).count,
                    (*queue_result_ptr).count,
                    (*queue_result_ptr).elements,
                );
            }
            queue_free(queue_sel_ptr);
            queue_free(queue_result_ptr);
        }
        Ok(self)
    }
    pub fn get_query_result(self) -> Result<Vec<SearchPkgInfo>> {
        if self.queue_result.count == 0 {
            bail!(ERROR_RDNF_NO_MATCH);
        }
        let mut pkg_infos = Vec::new();
        for index in 0..self.queue_result.count {
            let pkg_id = get_queue_element_value(&self.queue_result as *const Queue, index as u32);

            let solv = pool_id2solvable(self.sack.pool, pkg_id);
            let pkg_name = self.sack.solv_get_pkg_name_by_id(pkg_id)?;
            let pkg_summary = unsafe {
                let temp_summary_ptr =
                    solvable_lookup_str(solv, solv_knownid_SOLVABLE_SUMMARY as i32);
                if temp_summary_ptr.is_null() {
                    bail!(ERROR_RDNF_NO_DATA);
                }
                CStr::from_ptr(temp_summary_ptr).to_str()?.to_string()
            };
            let pkg_arch = unsafe {
                CStr::from_ptr(solvable_lookup_str(solv, solv_knownid_SOLVABLE_ARCH as i32))
                    .to_str()?
            };
            let pkg_name = pkg_name + "." + pkg_arch;
            let pkg_info = SearchPkgInfo {
                pkg_id,
                pkg_name,
                pkg_summary,
            };
            pkg_infos.push(pkg_info);
        }
        Ok(pkg_infos)
    }
}
#[derive(Debug, Clone)]
pub struct SearchPkgInfo {
    pub pkg_id: i32,
    pub pkg_name: String,
    pub pkg_summary: String,
}
