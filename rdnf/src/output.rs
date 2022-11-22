use console::{style, Term};

use crate::{
    errors::ERROR_RDNF_INVALID_PARAMETER,
    goal::{SolvedPkgInfo},
    i18n::action_alter::{
        ACTION_ALTER_DOWNGRADE, ACTION_ALTER_ERASE, ACTION_ALTER_INTALL, ACTION_ALTER_OBSOLETED,
        ACTION_ALTER_REINSTALL, ACTION_ALTER_UPGRADE,
    },
    sub_command::{install::AlterType, info::PkgInfo},
    utils::format_size,
};
use anyhow::{bail, Result};
impl SolvedPkgInfo {
    pub fn print(&self, term: &Term) -> Result<()> {
        if self.existing.is_some() {
            for pkg_info in self.existing.as_ref().unwrap() {
                let base = &pkg_info.base;
                let name = format!(
                    "{}-{}-{}.{}",
                    style(base.name.as_str()).green(),
                    base.version,
                    base.release,
                    base.arch
                );
                term.write_line(&format!("Package {} is already installed, skipping", name))?;
            }
        }
        if self.not_available.is_some() {
            for pkg_info in self.not_available.as_ref().unwrap() {
                term.write_line(&format!(
                    "No package {} available.",
                    style(pkg_info.base.name.as_str()).red()
                ))?;
            }
        }
        if self.base.to_install.is_some() {
            PkgInfo::print_action(
                self.base.to_install.as_ref().unwrap(),
                term,
                AlterType::Install,
            )?;
        }
        if self.base.to_upgrade.is_some() {
            PkgInfo::print_action(
                self.base.to_upgrade.as_ref().unwrap(),
                term,
                AlterType::Upgrade,
            )?;
        }
        if self.base.to_downgrade.is_some() {
            PkgInfo::print_action(
                self.base.to_downgrade.as_ref().unwrap(),
                term,
                AlterType::DownGrade,
            )?;
        }
        if self.base.to_remove.is_some() {
            PkgInfo::print_action(
                self.base.to_remove.as_ref().unwrap(),
                term,
                AlterType::Erase,
            )?;
        }
        if self.base.un_needed.is_some() {
            PkgInfo::print_action(
                self.base.un_needed.as_ref().unwrap(),
                term,
                AlterType::Erase,
            )?;
        }
        if self.base.to_reinstall.is_some() {
            PkgInfo::print_action(
                self.base.to_reinstall.as_ref().unwrap(),
                term,
                AlterType::ReInstall,
            )?;
        }
        if self.base.obsoleted.is_some() {
            PkgInfo::print_action(
                self.base.obsoleted.as_ref().unwrap(),
                term,
                AlterType::Obsoleted,
            )?;
        }
        Ok(())
    }
}
impl PkgInfo {
    pub fn print_action(pkg_infos: &Vec<Self>, term: &Term, alter_type: AlterType) -> Result<()> {
        term.write_line((alter_type.to_str()?.to_string() + ":").as_str())?;
        let (_, width) = term.size();
        let width_float = width as f32;
        let name_col = (width_float * 0.3) as usize;
        let arch_col = (width_float * 0.15) as usize;
        let evr_col = (width_float * 0.25) as usize;
        let repo_col = (width_float * 0.15) as usize;
        let install_size_col = (width_float * 0.15) as usize;
        let mut total_size = 0;
        for pkg_info in pkg_infos {
            let base = &pkg_info.base;
            let evr = if base.epoch == 0 {
                format!("{}-{}", base.version, base.release)
            } else {
                format!("{}:{}-{}", base.epoch, base.version, base.release)
            };
            total_size += pkg_info.details.install_size;
            let item = format!(
                "{:<name$}{:<arch$}{:<evr$}{:<repo$}{:<install$}",
                style(base.name.as_str()).green(),
                base.arch.as_str(),
                evr.as_str(),
                pkg_info.base.repo_name.as_str(),
                pkg_info.details.formatted_size.as_str(),
                name = name_col,
                arch = arch_col,
                evr = evr_col,
                repo = repo_col,
                install = install_size_col
            );
            term.write_line(&item)?;
        }
        let formatted_size = format_size(total_size);
        term.write_line("")?;
        let total = format!(
            "Total installed size: {} ; packages : {}",
            style(formatted_size.as_str()).green(),
            pkg_infos.len()
        );
        term.write_line(&total)?;
        Ok(())
    }
}
impl AlterType {
    pub fn to_str(&self) -> Result<&str> {
        let p = match self {
            Self::Install => ACTION_ALTER_INTALL,
            Self::Upgrade => ACTION_ALTER_UPGRADE,
            Self::Erase => ACTION_ALTER_ERASE,
            Self::DownGrade => ACTION_ALTER_DOWNGRADE,
            Self::ReInstall => ACTION_ALTER_REINSTALL,
            Self::Obsoleted => ACTION_ALTER_OBSOLETED,
            _ => {
                bail!(ERROR_RDNF_INVALID_PARAMETER)
            }
        };
        Ok(p)
    }
}
