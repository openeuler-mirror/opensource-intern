use crate::{
    cli::InfoOption,
    i18n::pkg_info::{
        PKG_INFO_ARCH___, PKG_INFO_DESC___, PKG_INFO_EPOCH__, PKG_INFO_LICENSE, PKG_INFO_NAME___,
        PKG_INFO_RELEASE, PKG_INFO_REPO___, PKG_INFO_SIZE___, PKG_INFO_SUMMARY, PKG_INFO_URL____,
        PKG_INFO_VERSION,
    },
    solv::rdnf_query::SolvQuery,
    Rdnf,
};
use anyhow::Result;
// #[derive(Clone)]
// pub enum RdnfScope {
//     All,
//     Installed,
//     Available,
//     Extras,
//     Obsoletes,
//     Recent,
//     Upgrades,
//     DownGrades,
// }

#[derive(Debug, Clone)]
pub struct PkgInfo {
    pub base: PkgInfoBase,
    pub details: PkgInfoDetails,
    pub other: Option<PkgInfoOther>,
}
#[derive(Debug, Clone)]
pub struct PkgInfoBase {
    pub epoch: u32,
    pub name: String,
    pub version: String,
    pub release: String,
    pub arch: String,
    pub evr: String,
    pub repo_name: String,
}
#[derive(Debug, Clone)]
pub struct PkgInfoDetails {
    pub install_size: u64,
    pub formatted_size: String,
    pub summary: String,
    pub location: Option<String>,
}
#[derive(Debug, Clone)]
pub struct PkgInfoOther {
    pub url: String,
    pub license: String,
    pub description: String,
}
// impl InfoOption {
//     pub fn parse_scope(&self){
//         let mut scopes=Vec::new();

//     }

// }
impl Rdnf {
    pub fn info_command(&mut self, info_opt: InfoOption) -> Result<()> {
        self.make_cache()?;
        let mut query = SolvQuery::default(self.rc.sack.clone());
        if info_opt.all || info_opt.installed {
            query.solv_add_system_repo_filter()?;
        }
        if info_opt.available {
            query.solv_add_available_repo_filter()?;
        }
        if !info_opt.pkgs.is_empty() {
            query.package_names = Some(info_opt.pkgs);
        }
        query.solv_apply_list_query()?;
        match query.solv_get_query_result() {
            Ok(pkg_list) => {
                let pkg_infos =
                    PkgInfo::populate_pkg_info(&self.rc.sack, &pkg_list, PkgInfoLevel::Other)?;
                for pkg_info in pkg_infos {
                    let term = &self.rc.term;
                    term.write_line(
                        format!("{}\t : {}", PKG_INFO_NAME___, pkg_info.base.name).as_str(),
                    )?;
                    term.write_line(
                        format!("{}\t : {}", PKG_INFO_ARCH___, pkg_info.base.arch).as_str(),
                    )?;
                    term.write_line(
                        format!("{}\t : {}", PKG_INFO_EPOCH__, pkg_info.base.epoch).as_str(),
                    )?;
                    term.write_line(
                        format!("{}\t : {}", PKG_INFO_VERSION, pkg_info.base.version).as_str(),
                    )?;
                    term.write_line(
                        format!("{}\t : {}", PKG_INFO_RELEASE, pkg_info.base.release).as_str(),
                    )?;
                    term.write_line(
                        format!(
                            "{}\t : {}",
                            PKG_INFO_SIZE___, pkg_info.details.formatted_size
                        )
                        .as_str(),
                    )?;
                    term.write_line(
                        format!("{}\t : {}", PKG_INFO_REPO___, pkg_info.base.repo_name).as_str(),
                    )?;
                    term.write_line(
                        format!("{}\t : {}", PKG_INFO_SUMMARY, pkg_info.details.summary).as_str(),
                    )?;
                    if pkg_info.other.is_some() {
                        term.write_line(
                            format!(
                                "{}\t : {}",
                                PKG_INFO_URL____,
                                pkg_info.other.as_ref().unwrap().url
                            )
                            .as_str(),
                        )?;
                        term.write_line(
                            format!(
                                "{}\t : {}",
                                PKG_INFO_LICENSE,
                                pkg_info.other.as_ref().unwrap().license
                            )
                            .as_str(),
                        )?;
                        let desc:Vec<&str> = pkg_info.other.as_ref().unwrap().description.split("\n").collect();
                        term.write_line(format!("{}\t : {}",PKG_INFO_DESC___,desc[0]).as_str())?;
                        for item in &desc[1..]{
                            term.write_line(format!("\t\t : {}",item).as_str())?;
                        }

                    }
                    term.write_line("")?;
                }
            }
            Err(_) => {}
        };
        Ok(())
    }
}
pub enum PkgInfoLevel {
    Details,
    Other,
}
