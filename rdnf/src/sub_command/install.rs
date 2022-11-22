use std::{
    ffi::CString,
    fs::{create_dir_all, metadata},
    path::Path,
};

use crate::{
    c_lib::{queue_push, queue_empty, get_queue_element_value},
    cli::AlterOption,
    default::{CMDLINE_REPO_NAME, RPM_CACHE_DIR_NAME, GPGKEY_CACHE_DIR_NAME},
    errors::{
        ERROR_RDNF_INVALID_PARAMETER, ERROR_RDNF_NOTHING_TO_DO, ERROR_RDNF_NO_MATCH,
        ERROR_RDNF_REPO_NOT_FOUND, ERROR_RDNF_URL_INVALID,
    },
    goal::SolvedPkgInfo,
    solv::rdnf_query::init_queue,
    Rdnf,
};
use anyhow::{bail, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm};
use solv_sys::ffi::{
    pool_createwhatprovides, repo_add_rpm, repo_internalize, s_Queue, Queue, REPO_NO_INTERNALIZE,
    REPO_REUSE_REPODATA, RPM_ADD_WITH_HDRID, RPM_ADD_WITH_SHA256SUM,
};

use super::{repo::RepoData, repoutils::download_file};
impl Rdnf {
    pub fn alter_command(
        &mut self,
        pkgs: Vec<String>,
        alter_type: AlterType,
        alter_args: &AlterOption,
    ) -> Result<()> {
        let solved_pkg_info = self.resolve(&pkgs, alter_type.clone(), alter_args)?;
        let silent = alter_args.quiet && alter_args.assume_yes;
        if !silent && !solved_pkg_info.not_resolved.is_empty() {
            for pkg_name in &solved_pkg_info.not_resolved {
                self.rc
                    .term
                    .write_line(&format!("No package {} available", style(pkg_name).red()))?;
            }
        }
        if solved_pkg_info.need_action == 0 {
            if solved_pkg_info.not_resolved.is_empty() {
                bail!(ERROR_RDNF_NO_MATCH);
            } else {
                bail!(ERROR_RDNF_NOTHING_TO_DO);
            }
        }
        if !silent {
            solved_pkg_info.print(&self.rc.term)?;
            if alter_args.download_only {
                self.rc
                    .term
                    .write_line("rdnf will only download packages needed for the transaction")?;
            }
        }
        if solved_pkg_info.need_action > 0 {
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Is this ok")
                .interact()
                .unwrap()
            {
                if !silent && solved_pkg_info.need_download > 0 {
                    self.rc.term.write_line("Downloading:")?;
                }
                self.rpm_exec_transaction(&solved_pkg_info, &alter_type, alter_args)?;
            }
        }
        Ok(())
    }
    pub fn resolve(
        &mut self,
        pkgs: &Vec<String>,
        alter_type: AlterType,
        alter_args: &AlterOption,
    ) -> Result<SolvedPkgInfo> {
        let mut queue_goal = init_queue();
        let mut not_resolved = Vec::<String>::new();
        if alter_type.is_install() || alter_type.is_reinstall() {
            self.add_cmdline_pkgs(&pkgs, &mut queue_goal as *mut Queue)?;
        }
        self.make_cache()?;
        self.prepare_all_pkgs(alter_type.clone(), pkgs, &mut not_resolved, &mut queue_goal)?;
        let solved_pkg_info_base = self.goal(&mut queue_goal, alter_type.clone(), alter_args)?;
        solved_pkg_info_base.check_protected_pkgs()?;
        Ok(SolvedPkgInfo {
            need_action: solved_pkg_info_base.get_need_action(),
            need_download: solved_pkg_info_base.get_need_download(),
            not_available: None,
            existing: None,
            not_resolved,
            not_installed: None,
            base: solved_pkg_info_base,
        })
    }

    pub fn add_cmdline_pkgs(&self, pkgs: &Vec<String>, queue: *mut s_Queue) -> Result<()> {
        for pkg in pkgs {
            let rpm_path = if Path::new(pkg.as_str()).exists() && pkg.ends_with(".rpm") {
                pkg.clone()
            } else {
                if !is_remote_url(pkg.as_str()) {
                    if !pkg.starts_with("file://") {
                        continue;
                    } else {
                        let k = pkg.split_once("file://").unwrap().1;
                        if k == "" || k.matches("#").collect::<Vec<_>>().len() > 0 {
                            bail!(ERROR_RDNF_URL_INVALID);
                        };
                        "/".to_string() + k.split_once("/").unwrap().1
                    }
                } else {
                    let pkg_name = pkg.rsplit_once("/").unwrap().1;
                    match self.repos.iter().find(|x| x.psz_id == CMDLINE_REPO_NAME) {
                        Some(repo) => self.download_pkg_to_cache(
                            pkg.as_str(),
                            pkg_name,
                            repo,
                            RPM_CACHE_DIR_NAME,
                        )?,
                        None => {
                            bail!(ERROR_RDNF_REPO_NOT_FOUND)
                        }
                    }
                }
            };
            unsafe {
                let file_path = CString::new(rpm_path.as_str()).unwrap();
                let id = repo_add_rpm(
                    self.solv_cmdline_repo,
                    file_path.as_ptr(),
                    (REPO_REUSE_REPODATA
                        | REPO_NO_INTERNALIZE
                        | RPM_ADD_WITH_HDRID
                        | RPM_ADD_WITH_SHA256SUM)
                        .try_into()
                        .unwrap(),
                );
                if id == 0 {
                    bail!(ERROR_RDNF_INVALID_PARAMETER)
                }
                queue_push(queue, id);
            }
        }
        unsafe {
            pool_createwhatprovides(self.rc.sack.pool);
            repo_internalize(self.solv_cmdline_repo);
        }
        Ok(())
    }
    pub fn prepare_all_pkgs(
        &self,
        alter_type: AlterType,
        pkgs: &Vec<String>,
        not_resolved: &mut Vec<String>,
        queue_goal: *mut Queue,
    ) -> Result<()> {
        let mut queue_local = init_queue();
        match alter_type {
            AlterType::DownGradeAll => {
                //TODO
            }
            AlterType::AutoEraseAll => {
                //TODO
            }
            _ => {}
        }
        let cli = &self.rc.cli;
        if (alter_type.is_upgrade_all() || alter_type.is_upgrade())
            && (cli.security || cli.sec_severity.is_some() || cli.reboot_required)
        {
            let pkgs=self.get_update_pkgs()?;
            for pkg_name in pkgs {
                self.prepare_single_pkg(pkg_name.as_str(), AlterType::Upgrade, not_resolved, queue_goal)?;
            }
        } else {
            for pkg in pkgs {
                if is_glob(pkg.as_str()) {
                    queue_empty(&mut queue_local);
                    self.rc.sack.get_glob_pkgs(pkg, &mut queue_local)?;
                    if queue_local.count ==0{
                        not_resolved.push(pkg.to_string());
                    }else{
                        for index in 0..queue_local.count{
                            let id=get_queue_element_value(&queue_local, index as u32);
                            let pkg_name=self.rc.sack.solv_get_pkg_name_by_id(id)?;
                            self.prepare_single_pkg(pkg_name.as_str(), alter_type.clone(), not_resolved, queue_goal)?;
                        }
                    }
                } else {
                    if Path::new(pkg.as_str()).exists() && pkg.ends_with(".rpm") {
                        continue;
                    }
                    if is_remote_url(pkg.as_str()) || pkg.starts_with("file://") {
                        continue;
                    }
                    self.prepare_single_pkg(
                        pkg.as_str(),
                        alter_type.clone(),
                        not_resolved,
                        queue_goal,
                    )?;
                }
            }
        };
        Ok(())
    }

    pub fn prepare_single_pkg(
        &self,
        pkg_name: &str,
        alter_type: AlterType,
        not_resolved: &mut Vec<String>,
        queue_goal: *mut Queue,
    ) -> Result<()> {
        match self.rc.sack.solv_count_pkg_by_name(pkg_name) {
            Ok(0) => {
                not_resolved.push(pkg_name.to_string());
                return Ok(());
            }
            Ok(count) => count,
            Err(_) => {
                bail!("{} package not found or not installed", pkg_name)
            }
        };
        match alter_type {
            AlterType::ReInstall => {
                self.rc.sack.add_pkgs_for_reinstall(queue_goal, pkg_name)?;
            }
            AlterType::Erase | AlterType::AutoErase => {
                self.rc.sack.add_pkgs_for_erase(queue_goal, pkg_name)?;
            }
            AlterType::Install => {
                if !self.rc.sack.add_pkgs_for_install(queue_goal, pkg_name)? {
                    println!("Package {} is already installed", pkg_name);
                }
            }
            AlterType::Upgrade => {
                self.rc.sack.add_pkgs_for_upgrade(queue_goal, pkg_name)?;
            }
            AlterType::DownGradeAll | AlterType::DownGrade => {}
            _ => {}
        }
        Ok(())
    }
}
pub fn is_remote_url(url: &str) -> bool {
    let mut is_url = false;
    for m in ["http://", "https://", "ftp://", "ftps://"] {
        if url.starts_with(m) {
            is_url = true;
            break;
        };
    }
    is_url
}
pub fn is_glob(s: &str) -> bool {
    for ele in s.chars() {
        if ele == '*' || ele == '?' || ele == '[' {
            return true;
        }
    }
    false
}

impl Rdnf {
    pub fn download_pkg_to_cache(
        &self,
        url: &str,
        pkg_name: &str,
        repo: &RepoData,
        dir: &str,
    ) -> Result<String> {
        let rpm_cache_dir = self.rc.conf.cachedir.clone() + repo.psz_id.as_str() + "/" + dir + "/";
        let u = match url.split_once("://") {
            Some((_, s)) => match s.split_once("/") {
                Some((_, s)) => s,
                None => {
                    bail!(ERROR_RDNF_URL_INVALID)
                }
            },
            None => {
                bail!(ERROR_RDNF_URL_INVALID)
            }
        };
        let rpm_cache_file = rpm_cache_dir + u;
        let download_cache_dir = rpm_cache_file
            .rsplit_once("/")
            .unwrap()
            .0
            .trim_end_matches("/");
        let rpm_cache_file = download_cache_dir.to_string() + "/" + pkg_name;
        create_dir_all(download_cache_dir)?;
        let (_, width) = self.rc.term.size();
        let repo_width = (width as f32 * 0.6) as usize;
        let flag_width = width as usize - repo_width - 4;

        if Path::new(rpm_cache_file.as_str()).exists() {
            let m = metadata(rpm_cache_file.as_str())?;
            if m.len() > 0 {
                let item = format!(
                    "{:<width$}{:>rest$}{:>4}",
                    pkg_name,
                    style("exists").green(),
                    "",
                    width = repo_width,
                    rest = flag_width
                );
                self.rc.term.write_line(item.as_str())?;
                return Ok(rpm_cache_file);
            }
        }
        download_file(&self.rc, repo, url, rpm_cache_file.as_str(), pkg_name)?;
        let item = format!(
            "{:<width$}{:>rest$}{:>4}",
            pkg_name,
            style("downloaded").green(),
            "",
            width = repo_width,
            rest = flag_width
        );
        self.rc.term.write_line(item.as_str())?;
        Ok(rpm_cache_file)
    }
    pub fn download_key_to_cache(
        &self,
        url: &str,
        repo: &RepoData,
    ) -> Result<String> {
        let (_,rest) = url.split_once("://").unwrap();
        let rest = match rest.rsplit_once("/") {
            Some((_,r)) => {r},
            None => {rest},
        };
        let file_path=self.rc.conf.cachedir.to_string()+repo.psz_id.as_str()+"/"+GPGKEY_CACHE_DIR_NAME+"/"+rest;
        let (_, width) = self.rc.term.size();
        let repo_width = (width as f32 * 0.6) as usize-" gpgkey".len();
        let flag_width = width as usize - repo_width - 4;
        download_file(&self.rc, repo, url, file_path.as_str(), repo.psz_id.as_str())?;
        let item = format!(
            "{:<width$} gpgkey {:>rest$}{:>4}",
            repo.psz_id,
            style("downloaded").green(),
            "",
            width = repo_width,
            rest = flag_width
        );
        self.rc.term.write_line(item.as_str())?;
        Ok(file_path)
    }
}

#[derive(Debug, Clone)]
pub enum AlterType {
    AutoErase,
    AutoEraseAll,
    DownGrade,
    DownGradeAll,
    Erase,
    Install,
    ReInstall,
    Upgrade,
    UpgradeAll,
    DistroSync,
    Obsoleted,
}
impl AlterType {
    pub fn is_auto_erase(&self) -> bool {
        match self {
            Self::AutoErase => true,
            _ => false,
        }
    }
    pub fn is_auto_erase_all(&self) -> bool {
        match self {
            Self::AutoEraseAll => true,
            _ => false,
        }
    }
    pub fn is_down_grade(&self) -> bool {
        match self {
            Self::DownGradeAll => true,
            _ => false,
        }
    }
    pub fn is_erase(&self) -> bool {
        match self {
            Self::Erase => true,
            _ => false,
        }
    }
    pub fn is_install(&self) -> bool {
        match self {
            Self::Install => true,
            _ => false,
        }
    }
    pub fn is_reinstall(&self) -> bool {
        match self {
            Self::ReInstall => true,
            _ => false,
        }
    }
    pub fn is_upgrade(&self) -> bool {
        match self {
            Self::Upgrade => true,
            _ => false,
        }
    }
    pub fn is_upgrade_all(&self) -> bool {
        match self {
            Self::UpgradeAll => true,
            _ => false,
        }
    }
    pub fn is_distro_sync(&self) -> bool {
        match self {
            Self::DistroSync => true,
            _ => false,
        }
    }
    pub fn is_obsoleted(&self) -> bool {
        match self {
            Self::Obsoleted => true,
            _ => false,
        }
    }
}
