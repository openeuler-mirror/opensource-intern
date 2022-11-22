use std::ffi::CStr;
use std::ffi::CString;
use std::path::Path;

use anyhow::bail;
use anyhow::Result;
use console::style;
use console::Term;

use libc::c_void;
use rpm_sys::ffi::fnpyKey;
use rpm_sys::ffi::rpmDbiTag_e_RPMDBI_LABEL;
use rpm_sys::ffi::rpmProblemFree;
use rpm_sys::ffi::rpmProblemGetStr;
use rpm_sys::ffi::rpmProblemGetType;
use rpm_sys::ffi::rpmProblemString;
use rpm_sys::ffi::rpmProblemType_e_RPMPROB_REQUIRES;
use rpm_sys::ffi::rpmRelocation;
use rpm_sys::ffi::rpmTag;
use rpm_sys::ffi::rpmVSFlags_e_RPMVSF_NODSA;
use rpm_sys::ffi::rpmVSFlags_e_RPMVSF_NODSAHEADER;
use rpm_sys::ffi::rpmVSFlags_e_RPMVSF_NOMD5;
use rpm_sys::ffi::rpmVSFlags_e_RPMVSF_NOPAYLOAD;
use rpm_sys::ffi::rpmVSFlags_e_RPMVSF_NORSA;
use rpm_sys::ffi::rpmVSFlags_e_RPMVSF_NORSAHEADER;
use rpm_sys::ffi::rpmVSFlags_e_RPMVSF_NOSHA1HEADER;
use rpm_sys::ffi::rpmVSFlags_e_RPMVSF_NOSHA256HEADER;
use rpm_sys::ffi::rpmdbFreeIterator;
use rpm_sys::ffi::rpmdbGetIteratorOffset;
use rpm_sys::ffi::rpmdbNextIterator;
use rpm_sys::ffi::rpmlogLvl_e_RPMLOG_ALERT;
use rpm_sys::ffi::rpmlogLvl_e_RPMLOG_CRIT;
use rpm_sys::ffi::rpmlogLvl_e_RPMLOG_DEBUG;
use rpm_sys::ffi::rpmlogLvl_e_RPMLOG_EMERG;
use rpm_sys::ffi::rpmlogLvl_e_RPMLOG_ERR;
use rpm_sys::ffi::rpmlogLvl_e_RPMLOG_INFO;
use rpm_sys::ffi::rpmlogLvl_e_RPMLOG_NOTICE;
use rpm_sys::ffi::rpmlogLvl_e_RPMLOG_WARNING;
use rpm_sys::ffi::rpmlogSetMask;
use rpm_sys::ffi::rpmprobFilterFlags_e_RPMPROB_FILTER_OLDPACKAGE;
use rpm_sys::ffi::rpmprobFilterFlags_e_RPMPROB_FILTER_REPLACEPKG;
use rpm_sys::ffi::rpmps;
use rpm_sys::ffi::rpmpsFreeIterator;
use rpm_sys::ffi::rpmpsGetProblem;
use rpm_sys::ffi::rpmpsInitIterator;
use rpm_sys::ffi::rpmpsNextIterator;
use rpm_sys::ffi::rpmpsNumProblems;
use rpm_sys::ffi::rpmteEVR;
use rpm_sys::ffi::rpmteN;
use rpm_sys::ffi::rpmtransFlags_e_RPMTRANS_FLAG_NONE;
use rpm_sys::ffi::rpmtransFlags_e_RPMTRANS_FLAG_NOSCRIPTS;
use rpm_sys::ffi::rpmtransFlags_e_RPMTRANS_FLAG_TEST;
use rpm_sys::ffi::rpmts;
use rpm_sys::ffi::rpmtsAddEraseElement;
use rpm_sys::ffi::rpmtsAddInstallElement;
use rpm_sys::ffi::rpmtsCheck;
use rpm_sys::ffi::rpmtsClean;
use rpm_sys::ffi::rpmtsCreate;
use rpm_sys::ffi::rpmtsInitIterator;
use rpm_sys::ffi::rpmtsOrder;
use rpm_sys::ffi::rpmtsProblems;
use rpm_sys::ffi::rpmtsRun;
use rpm_sys::ffi::rpmtsSetFlags;
use rpm_sys::ffi::rpmtsSetRootDir;
use rpm_sys::ffi::rpmtsSetVSFlags;
use rpm_sys::ffi::rpmtsSetVfyLevel;
use rpm_sys::ffi::rpmtsVSFlags;
use rpm_sys::ffi::rpmtsiInit;
use rpm_sys::ffi::rpmtsiNext;
use rpm_sys::ffi::rpmvercmp;
use rpm_sys::ffi::RPMLOG_PRIMASK;
use rpm_sys::ffi::RPMSIG_DIGEST_TYPE;
use rpm_sys::ffi::RPMSIG_SIGNATURE_TYPE;

use crate::c_lib::set_callbackfunction;
use crate::cli::AlterOption;
use crate::default::RPM_CACHE_DIR_NAME;
use crate::errors::ERROR_RDNF_INVALID_PARAMETER;
use crate::errors::ERROR_RDNF_REPO_NOT_FOUND;
use crate::errors::ERROR_RDNF_RPMTS_CREATE_FAILED;
use crate::errors::ERROR_RDNF_RPM_CHECK;
use crate::errors::ERROR_RDNF_TRANSACTION_FAILED;
use crate::goal::SolvedPkgInfo;
use crate::sub_command::info::PkgInfo;
use crate::sub_command::install::AlterType;
use crate::Rdnf;
pub const RPMVSF_MASK_NODIGESTS: u32 = rpmVSFlags_e_RPMVSF_NOSHA1HEADER
    | rpmVSFlags_e_RPMVSF_NOSHA256HEADER
    | rpmVSFlags_e_RPMVSF_NOPAYLOAD
    | rpmVSFlags_e_RPMVSF_NOMD5;

pub const RPMVSF_MASK_NOSIGNATURES: u32 = rpmVSFlags_e_RPMVSF_NODSAHEADER
    | rpmVSFlags_e_RPMVSF_NORSAHEADER
    | rpmVSFlags_e_RPMVSF_NODSA
    | rpmVSFlags_e_RPMVSF_NORSA;
pub const RPMSIG_VERIFIABLE_TYPE: u32 = RPMSIG_DIGEST_TYPE | RPMSIG_SIGNATURE_TYPE;
pub struct RpmTs {
    pub cached_rpms: Vec<String>,
    pub trans_flags: i32,
    pub prob_filter_flags: u32,
    pub ts: rpmts,
}
impl Rdnf {
    pub fn parse_rpm_verbosity(&self) -> u32 {
        match self.rc.cli.rpm_verbosity.as_str() {
            "emergency" => rpmlogLvl_e_RPMLOG_EMERG,
            "alert" => rpmlogLvl_e_RPMLOG_ALERT,
            "critical" => rpmlogLvl_e_RPMLOG_CRIT,
            "error" => rpmlogLvl_e_RPMLOG_ERR,
            "warning" => rpmlogLvl_e_RPMLOG_WARNING,
            "notice" => rpmlogLvl_e_RPMLOG_NOTICE,
            "info" => rpmlogLvl_e_RPMLOG_INFO,
            "debug" => rpmlogLvl_e_RPMLOG_DEBUG,
            _ => rpmlogLvl_e_RPMLOG_ERR,
        }
    }
    pub fn rpm_exec_transaction(
        &self,
        solved_pkg_info: &SolvedPkgInfo,
        alter_type: &AlterType,
        alter_args: &AlterOption,
    ) -> Result<()> {
        unsafe {
            let p = self.parse_rpm_verbosity();
            let pri = (1 << ((p & RPMLOG_PRIMASK) + 1)) - 1;
            rpmlogSetMask(pri)
        };
        let mut prob_filter_flags = rpmprobFilterFlags_e_RPMPROB_FILTER_OLDPACKAGE;
        if alter_type.is_reinstall() {
            prob_filter_flags = prob_filter_flags | rpmprobFilterFlags_e_RPMPROB_FILTER_REPLACEPKG;
        }
        let ts = unsafe { rpmtsCreate() };
        if ts.is_null() {
            bail!(ERROR_RDNF_RPMTS_CREATE_FAILED);
        };
        let mut trans_flags = rpmtransFlags_e_RPMTRANS_FLAG_NONE;
        if alter_args.tsflags_noscripts {
            trans_flags |= rpmtransFlags_e_RPMTRANS_FLAG_NOSCRIPTS;
        }
        let root_ptr =
            CString::new(self.rc.cli.installroot.clone()).unwrap_or(CString::new("/").unwrap());
        unsafe { rpmtsSetRootDir(ts, root_ptr.as_ptr()) };
        let mut rpm_ts = RpmTs {
            cached_rpms: Vec::new(),
            trans_flags,
            prob_filter_flags,
            ts,
        };
        let (_, width) = self.rc.term.size();
        set_callbackfunction(rpm_ts.ts, alter_args.quiet, width);
        self.populate_transaction(&mut rpm_ts, solved_pkg_info, alter_args)?;
        self.run_transaction(&mut rpm_ts, &self.rc.term, alter_args)?;
        Ok(())
    }
    pub fn populate_transaction(
        &self,
        rpm_ts: &mut RpmTs,
        solved_pkg_info: &SolvedPkgInfo,
        alter_args: &AlterOption,
    ) -> Result<()> {
        if solved_pkg_info.base.to_install.is_some() {
            let pkg_infos = solved_pkg_info.base.to_install.as_ref().unwrap();
            self.trans_add_install_pkgs(rpm_ts, pkg_infos, 0, alter_args)?;
        }
        if solved_pkg_info.base.to_reinstall.is_some() {
            let pkg_infos = solved_pkg_info.base.to_reinstall.as_ref().unwrap();
            self.trans_add_install_pkgs(rpm_ts, pkg_infos, 0, alter_args)?;
        }
        if solved_pkg_info.base.to_upgrade.is_some() {
            let pkg_infos = solved_pkg_info.base.to_upgrade.as_ref().unwrap();
            self.trans_add_install_pkgs(rpm_ts, pkg_infos, 1, alter_args)?;
        }
        if solved_pkg_info.base.to_remove.is_some() {
            let pkg_infos = solved_pkg_info.base.to_remove.as_ref().unwrap();
            self.trans_add_erase_pkg(rpm_ts, pkg_infos);
        }
        if solved_pkg_info.base.obsoleted.is_some() {
            let pkg_infos = solved_pkg_info.base.obsoleted.as_ref().unwrap();
            self.trans_add_erase_pkg(rpm_ts, pkg_infos);
        }
        if solved_pkg_info.base.to_downgrade.is_some() {
            let pkg_infos = solved_pkg_info.base.to_downgrade.as_ref().unwrap();
            self.trans_add_install_pkgs(rpm_ts, pkg_infos, 0, alter_args)?;
            if solved_pkg_info.base.removed_by_downgrade.is_some() {
                let pkg_infos = solved_pkg_info.base.removed_by_downgrade.as_ref().unwrap();
                self.trans_add_erase_pkg(rpm_ts, pkg_infos);
            }
        }
        Ok(())
    }
    pub fn trans_add_install_pkgs(
        &self,
        rpm_ts: &mut RpmTs,
        pkg_infos: &Vec<PkgInfo>,
        upgrade: i32,
        alter_args: &AlterOption,
    ) -> Result<()> {
        for pkg_info in pkg_infos {
            let mut location = pkg_info.details.location.clone().unwrap();
            let pkg_name = pkg_info.base.name.as_str();
            let repo_name = pkg_info.base.repo_name.as_str();
            let repo = match self.repos.iter().find(|x| x.psz_id == repo_name) {
                Some(repo) => repo,
                None => {
                    bail!(ERROR_RDNF_REPO_NOT_FOUND)
                }
            };
            if !Path::new(location.as_str()).exists() {
                location = match repo.details.base_url.as_ref() {
                    Some(base_url) => {
                        let url =
                            base_url.trim_end_matches("/").to_string() + "/" + location.as_str();
                        self.download_pkg_to_cache(
                            url.as_str(),
                            pkg_name,
                            repo,
                            RPM_CACHE_DIR_NAME,
                        )?
                    }
                    None => {
                        bail!(ERROR_RDNF_REPO_NOT_FOUND)
                    }
                };
            };
            let (header, gpg_check) =
                self.gpgcheck_pkg(rpm_ts, location.as_str(), repo, alter_args)?;
            if !gpg_check {
                unsafe {
                    rpmtsSetVSFlags(
                        rpm_ts.ts,
                        rpmtsVSFlags(rpm_ts.ts) | RPMVSF_MASK_NODIGESTS | RPMVSF_MASK_NOSIGNATURES,
                    );
                    rpmtsSetVfyLevel(rpm_ts.ts, !RPMSIG_VERIFIABLE_TYPE as i32);
                }
            }
            unsafe {
                let file_ptr = CString::new(location.as_str()).unwrap().into_raw();
                rpmtsAddInstallElement(
                    rpm_ts.ts,
                    header,
                    file_ptr as fnpyKey,
                    upgrade,
                    0 as *mut rpmRelocation,
                )
            };
            rpm_ts.cached_rpms.push(location);
        }
        Ok(())
    }
    pub fn trans_add_erase_pkg(&self, rpm_ts: &mut RpmTs, pkg_infos: &Vec<PkgInfo>) {
        for pkg_info in pkg_infos {
            let pkg_name = CString::new(pkg_info.base.name.as_str()).unwrap();
            let iter = unsafe {
                rpmtsInitIterator(
                    rpm_ts.ts,
                    rpmDbiTag_e_RPMDBI_LABEL as rpmTag,
                    pkg_name.as_ptr() as *const c_void,
                    0,
                )
            };
            loop {
                let rpm_header = unsafe { rpmdbNextIterator(iter) };
                if rpm_header.is_null() {
                    break;
                }
                let offset = unsafe { rpmdbGetIteratorOffset(iter) };
                if offset > 0 {
                    unsafe { rpmtsAddEraseElement(rpm_ts.ts, rpm_header, offset as i32) };
                }
            }
            if !iter.is_null() {
                unsafe { rpmdbFreeIterator(iter) };
            }
        }
    }
    pub fn run_transaction(
        &self,
        rpm_ts: &mut RpmTs,
        term: &Term,
        alter_args: &AlterOption,
    ) -> Result<()> {
        let mut rpm_vfy_level_mask = 0;
        unsafe { rpmtsOrder(rpm_ts.ts) };
        rpm_ts.do_check(term)?;
        unsafe { rpmtsClean(rpm_ts.ts) };
        if alter_args.no_gpg_check {
            unsafe {
                rpmtsSetVSFlags(
                    rpm_ts.ts,
                    rpmtsVSFlags(rpm_ts.ts) | RPMVSF_MASK_NODIGESTS | RPMVSF_MASK_NOSIGNATURES,
                );
                rpmtsSetVSFlags(rpm_ts.ts, !RPMSIG_VERIFIABLE_TYPE);
            }
        } else if alter_args.skip_signatures || alter_args.skip_digest {
            if alter_args.skip_signatures {
                unsafe {
                    rpmtsSetVSFlags(
                        rpm_ts.ts,
                        rpmtsVSFlags(rpm_ts.ts) | RPMVSF_MASK_NOSIGNATURES,
                    );
                    rpm_vfy_level_mask |= RPMSIG_SIGNATURE_TYPE;
                }
            }
            if alter_args.skip_digest {
                unsafe {
                    rpmtsSetVSFlags(rpm_ts.ts, rpmtsVSFlags(rpm_ts.ts) | RPMVSF_MASK_NODIGESTS);
                    rpm_vfy_level_mask |= RPMSIG_DIGEST_TYPE;
                }
            }
            unsafe {
                rpmtsSetVfyLevel(rpm_ts.ts, !rpm_vfy_level_mask as i32);
            }
        }
        let rc = unsafe {
            rpmtsSetFlags(rpm_ts.ts, rpmtransFlags_e_RPMTRANS_FLAG_TEST as u32);
            rpmtsRun(rpm_ts.ts, 0 as rpmps, rpm_ts.prob_filter_flags)
        };
        if rc != 0 {
            println!("a");
            bail!(ERROR_RDNF_TRANSACTION_FAILED);
        }
        term.write_line("Running transaction")?;
        unsafe { rpmtsSetFlags(rpm_ts.ts, rpm_ts.trans_flags as u32) };
        let rc = unsafe { rpmtsRun(rpm_ts.ts, 0 as rpmps, rpm_ts.prob_filter_flags) };
        if rc != 0 {
            println!("b");
            bail!(ERROR_RDNF_TRANSACTION_FAILED);
        }
        Ok(())
    }
}
// pub fn rdnf_rpm_cb(
// ) -> unsafe extern "C" fn(*const c_void, u32, u64, u64, *const c_void, *mut c_void) -> *mut c_void {
//     unsafe extern "C" fn cb(
//         arg: *const c_void,
//         what: u32,
//         amount: u64,
//         total: u64,
//         key: *const c_void,
//         data: *mut c_void,
//     ) -> *mut c_void {
//         let pkg_header = arg as Header;
//         let mut callback_data = Box::from_raw(data as *mut RpmTsCallback);
//         let file_name_ptr = key as *const c_char;
//         let nevra = CStr::from_ptr(headerGetAsString(pkg_header, rpmTag_e_RPMTAG_NEVRA))
//             .to_str()
//             .unwrap();
//         match what {
//             rpmCallbackType_e_RPMCALLBACK_INST_OPEN_FILE => {
//                 if file_name_ptr.is_null() {
//                     println!("rpmcallback_inst_open_file null ");
//                     return 0 as *mut c_void;
//                 } else {
//                     println!(
//                         "rpmcallback_inst_open_file {}",
//                         CStr::from_ptr(file_name_ptr).to_str().unwrap()
//                     );
//                     let mode = CString::new("r.ufdio").unwrap();
//                     let fd = Fopen(file_name_ptr, mode.as_ptr());
//                     callback_data.fs = Some(fd);
//                 }
//             }
//             rpmCallbackType_e_RPMCALLBACK_INST_CLOSE_FILE => {
//                 if callback_data.fs.is_some() {
//                     let fs_ptr = callback_data.fs.unwrap();
//                     if !fs_ptr.is_null() {
//                         println!(
//                             "rpmcallback_inst_close_file {}",
//                             CStr::from_ptr(file_name_ptr).to_str().unwrap()
//                         );
//                         Fclose(fs_ptr);
//                         callback_data.fs = None;
//                     }
//                 }
//             }
//             rpmCallbackType_e_RPMCALLBACK_INST_START => {
//                 if !callback_data.quiet {
//                     println!("{:<20}{}", ACTION_ALTER_INTALL, nevra);
//                 }
//             }
//             rpmCallbackType_e_RPMCALLBACK_UNINST_START => {
//                 if !callback_data.quiet {
//                     println!("{:<20}{}", ACTION_ALTER_ERASE, nevra);
//                 }
//             }
//             rpmCallbackType_e_RPMCALLBACK_SCRIPT_ERROR => {
//                 let script = match amount as i32 {
//                     rpmTag_e_RPMTAG_PREIN => "%prein",
//                     rpmTag_e_RPMTAG_POSTIN => "%postin",
//                     rpmTag_e_RPMTAG_PREUN => "%preun",
//                     rpmTag_e_RPMTAG_POSTUN => "%postun",
//                     _ => "(unkown)",
//                 };
//                 let flag = if total == rpmRC_e_RPMRC_OK as u64 {
//                     "warning"
//                 } else {
//                     "error"
//                 };
//                 println!("package {}: script {} in {}", nevra, flag, script);
//             }
//             _ => {}
//         }

//         let _ = Box::into_raw(callback_data);
//         0 as *mut c_void
//     }
//     cb
// }
impl RpmTs {
    pub fn do_check(&self, term: &Term) -> Result<()> {
        let _nresult = unsafe { rpmtsCheck(self.ts) };
        let ps = unsafe { rpmtsProblems(self.ts) };
        if !ps.is_null() {
            let n_probs = unsafe { rpmpsNumProblems(ps) };
            if n_probs > 0 {
                term.write_line(format!("Found {} problems", n_probs).as_str())?;
                let psi = unsafe { rpmpsInitIterator(ps) };
                while unsafe { rpmpsNextIterator(psi) } >= 0 {
                    let prob = unsafe { rpmpsGetProblem(psi) };
                    let msg_ptr = unsafe { rpmProblemString(prob) };
                    let msg = unsafe { CStr::from_ptr(msg_ptr).to_str()? };
                    if msg.matches("no digest").collect::<Vec<_>>().len() >= 1 {
                        let info =
                            format!("{}. Use {} to ignore", msg, style("--skipdigest").red());
                        term.write_line(info.as_str())?;
                    } else {
                        term.write_line(msg)?;
                        if unsafe { rpmProblemGetType(prob) } == rpmProblemType_e_RPMPROB_REQUIRES {
                            let error_str =
                                unsafe { CStr::from_ptr(rpmProblemGetStr(prob)).to_str()? };
                            // Error str has the format: <Pkg_name> <Symbol> <version-releas>
                            let token = error_str.split(' ').collect::<Vec<_>>();
                            if token.len() != 3 {
                                term.write_line("RPM problem string format unsupported")?;
                                bail!(ERROR_RDNF_INVALID_PARAMETER);
                            }
                            let pkg_name = token[0];
                            let pkg_symbol = token[1];
                            let pkg_version = token[2];
                            let pkg_version_c = CString::new(pkg_version).unwrap();
                            let pi = unsafe { rpmtsiInit(self.ts) };
                            loop {
                                let pte = unsafe { rpmtsiNext(pi, 0) };
                                if pte.is_null() {
                                    break;
                                }
                                let cached_pkg_name =
                                    unsafe { CStr::from_ptr(rpmteN(pte)).to_str()? };
                                let cached_pkg_evr_ptr = unsafe { rpmteEVR(pte) };
                                if cached_pkg_name == token[0] {
                                    let more = pkg_symbol.find(">").is_some()
                                        && unsafe {
                                            rpmvercmp(cached_pkg_evr_ptr, pkg_version_c.as_ptr())
                                                > 0
                                        };
                                    let less = pkg_symbol.find("<").is_some()
                                        && unsafe {
                                            rpmvercmp(cached_pkg_evr_ptr, pkg_version_c.as_ptr())
                                                < 0
                                        };
                                    let equal = pkg_symbol.find("=").is_some()
                                        && unsafe {
                                            rpmvercmp(cached_pkg_evr_ptr, pkg_version_c.as_ptr())
                                                == 0
                                        };
                                    if more || less || equal {
                                        let item=format!("Detected rpm pre-transaction dependency errors. Install {} {} {} first to resolve this failure",
                                        pkg_name,pkg_symbol,pkg_version);
                                        term.write_line(item.as_str())?;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    unsafe { rpmProblemFree(prob) };
                }
                unsafe { rpmpsFreeIterator(psi) };
                bail!(ERROR_RDNF_RPM_CHECK);
            }
        }
        Ok(())
    }
}
