use rpm_sys::ffi::rpmts;
use solv_sys::ffi::{Dataiterator, Map, Pool, Queue, Repo, Solvable};

extern "C" {
    fn char_ptr_offset_bind(ptr: *const i8, offset: i32) -> *const i8;
    fn queue_empty_static(q: *mut Queue);
    fn queue_push_static(q: *mut Queue, id: i32);
    fn queue_push2_static(q: *mut Queue, id1: i32, id2: i32);
    fn create_data_iterator_empty_bind() -> Dataiterator; // Dataiterator di={0}
                                                          // fn create_Repo_empty_bind() -> s_Repo; // s_Pool p={0}
    fn dataiterator_init_simple_bind(
        di: *mut Dataiterator,
        pool: *mut Pool,
        match_: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    );
    fn dataiterator_set_search_simple_bind(di: *mut Dataiterator);
    fn get_queue_element_value_bind(q: *const Queue, index: ::std::os::raw::c_int) -> i32;
    fn pool_id2solvable_static(pool: *const Pool, p: ::std::os::raw::c_int) -> *mut Solvable;
    fn solv_add_flags_to_jobs_bind(q: *mut Queue, flags: ::std::os::raw::c_int);
    fn get_pool_solvables_value_bind(
        pool: *const Pool,
        index: ::std::os::raw::c_uint,
    ) -> *mut Solvable;
    fn is_pseudo_package_static(pool: *mut Pool, s: *mut Solvable) -> i32;
    fn pool_id2repo_static(pool: *const Pool, id: i32) -> *mut Repo;
    fn pool_whatprovides_static(pool: *mut Pool, d: i32) -> i32;
    fn get_pool_whatprovidesdata_value_bind(pool: *const Pool, index: i32) -> i32;
    fn pool_match_nevr_static(pool: *mut Pool, s: *mut Solvable, d: i32) -> i32;
    fn pool_disabled_solvable_static(pool: *const Pool, s: *mut Solvable) -> i32;
    // fn map_empty_static(m: *mut Map);
    fn map_set_static(m: *mut Map, n: i32);
    fn map_setall_static(m: *mut Map);
    // fn map_clr_static(m: *mut Map, n: i32);
    // fn map_tst_static(m: *mut Map, n: i32);
    // fn map_clr_at_static(m: *mut Map, n: i32);
    // int set_callback_fn(rpmts ts,int quiet,int term_width){
    fn set_callback_fn(ts: rpmts, quiet: i32, term_width: u16) -> i32;

}
// fn char_ptr_offset_bind(ptr:*const i8,offset:i32)->*const i8;
pub fn char_ptr_offset(ptr: *const i8, offset: i32) -> *const i8 {
    unsafe { char_ptr_offset_bind(ptr, offset) }
}
#[inline]
pub fn queue_empty(q: *mut Queue) {
    unsafe {
        queue_empty_static(q);
    }
}
#[inline]
pub fn queue_push(q: *mut Queue, id: i32) {
    unsafe {
        queue_push_static(q, id);
    }
}
#[inline]
pub fn queue_push2(q: *mut Queue, id1: i32, id2: i32) {
    unsafe {
        queue_push2_static(q, id1, id2);
    }
}
#[inline]
pub fn create_dataiterator_empty() -> Dataiterator {
    unsafe { create_data_iterator_empty_bind() }
}
// #[inline]
// pub fn create_Repo_empty() -> s_Repo {
//     unsafe { create_Repo_empty_bind() }
// }
#[inline]
pub fn dataiterator_init_simple(
    di: *mut Dataiterator,
    pool: *mut Pool,
    match_: *const ::std::os::raw::c_char,
    flags: ::std::os::raw::c_int,
) {
    unsafe {
        dataiterator_init_simple_bind(di, pool, match_, flags);
    }
}
#[inline]
pub fn dataiterator_set_search_simple(di: *mut Dataiterator) {
    unsafe {
        dataiterator_set_search_simple_bind(di);
    }
}
#[inline]
pub fn get_queue_element_value(q: *const Queue, index: u32) -> i32 {
    unsafe { get_queue_element_value_bind(q, index as i32) }
}
#[inline]
pub fn pool_id2solvable(pool: *const Pool, p: i32) -> *mut Solvable {
    unsafe { pool_id2solvable_static(pool, p) }
}
#[inline]
pub fn solv_add_flags_to_jobs(q: *mut Queue, flags: i32) {
    unsafe { solv_add_flags_to_jobs_bind(q, flags) }
}
#[inline]
pub fn get_pool_solvables_value(pool: *const Pool, index: u32) -> *mut Solvable {
    unsafe { get_pool_solvables_value_bind(pool, index) }
}
#[inline]
pub fn is_pseudo_package(pool: *mut Pool, s: *mut Solvable) -> bool {
    unsafe { is_pseudo_package_static(pool, s) == 1 }
}
#[inline]
pub fn pool_id2repo(pool: *const Pool, id: i32) -> *mut Repo {
    unsafe { pool_id2repo_static(pool, id) }
}
#[inline]
pub fn pool_whatprovides(pool: *mut Pool, id: i32) -> i32 {
    unsafe { pool_whatprovides_static(pool, id) }
}
#[inline]
pub fn get_pool_whatprovidesdata_value(pool: *const Pool, index: i32) -> i32 {
    unsafe { get_pool_whatprovidesdata_value_bind(pool, index) }
}
#[inline]
pub fn pool_match_nevr(pool: *mut Pool, s: *mut Solvable, d: i32) -> i32 {
    unsafe { pool_match_nevr_static(pool, s, d) }
}

pub fn pool_disabled_solvable(pool: *const Pool, s: *mut Solvable) -> bool {
    let result = unsafe { pool_disabled_solvable_static(pool, s) };
    result == 1
}
// pub fn map_empty(m: *mut Map) {
//     unsafe {
//         map_empty_static(m);
//     }
// }

pub fn map_set(m: *mut Map, n: i32) {
    unsafe {
        map_set_static(m, n);
    }
}
pub fn map_setall(m: *mut Map) {
    unsafe {
        map_setall_static(m);
    }
}
// pub fn map_clr(m: *mut Map, n: i32) {
//     unsafe {
//         map_clr_static(m, n);
//     }
// }
// pub fn map_tst(m: *mut Map, n: i32) {
//     unsafe {
//         map_tst_static(m, n);
//     }
// }
// pub fn map_clr_at(m: *mut Map, n: i32) {
//     unsafe {
//         map_clr_at_static(m, n);
//     }
// }
pub fn set_callbackfunction(ts: rpmts, quiet: bool, term_width: u16) -> i32 {
    unsafe { set_callback_fn(ts, quiet as i32, term_width) }
}
