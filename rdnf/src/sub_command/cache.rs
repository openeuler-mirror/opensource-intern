use std::{
    fs::{metadata, read_dir, remove_dir, remove_file},
    path::PathBuf,
    time::SystemTime,
};

use anyhow::{bail, Result};
use console::style;

use crate::{
    default::{CMDLINE_REPO_NAME, REPODATA_DIR_NAME, REPO_METADATA_MARKER, SOLVCACHE_DIR_NAME},
    errors::ERROR_RDNF_CACHE_REFRESH,
    utils::check_root,
    Rdnf,
};
impl Rdnf {
    pub fn make_cache(&mut self) -> Result<()> {
        let mut new_repos = Vec::new();
        for repo in self.repos.clone() {
            if repo.psz_name == CMDLINE_REPO_NAME || !repo.base.enabled {
                new_repos.push(repo);
                continue;
            }
            if repo.base.lmetadata_expire >= 0 && !self.rc.cli.cacheonly {
                match &repo.details.cache_name {
                    Some(s) => {
                        let refresh_flag =
                            self.rc.conf.cachedir.clone() + s.as_str() + "/" + REPO_METADATA_MARKER;
                        if should_sync_metadata(refresh_flag.as_str(), repo.base.lmetadata_expire)?
                        {
                            if check_root().is_err() {
                                if !self.rc.cli.cacheonly {
                                    bail!(ERROR_RDNF_CACHE_REFRESH);
                                }
                            }
                            let repodata_dir = PathBuf::from(
                                self.rc.conf.cachedir.clone()
                                    + s.as_str()
                                    + "/"
                                    + REPODATA_DIR_NAME,
                            );
                            recursively_remove_dir(&repodata_dir)?;
                            let solv_cache_dir = PathBuf::from(
                                self.rc.conf.cachedir.to_string()
                                    + s.as_str()
                                    + "/"
                                    + SOLVCACHE_DIR_NAME,
                            );
                            recursively_remove_dir(&solv_cache_dir)?;
                        }
                        let psz_id = repo.psz_id.clone();
                        let status = if repo.base.skip_if_unavailable {
                            match self.rc.init_repo(repo) {
                                Ok(s) => {
                                    new_repos.push(s);
                                    format!("{}", style("Done").green())
                                }
                                Err(_) => {
                                    format!("{}", style("Skip").red())
                                }
                            }
                        } else {
                            new_repos.push(self.rc.init_repo(repo)?);
                            format!("{}", style("Done").green())
                        };
                        if self.rc.cli.refresh {
                            let (_, width) = self.rc.term.size();
                            let offset = (width - 10) as usize;
                            self.rc.term.write_line(&psz_id)?;
                            self.rc.term.move_cursor_up(1)?;
                            self.rc.term.move_cursor_right(offset)?;
                            self.rc.term.write_line(&status)?;
                        }
                    }
                    None => {
                        bail!("repo {} enabled,but need baseurl or metalink", repo.psz_id);
                    }
                }
            }
        }
        self.repos = new_repos;
        // Ok(self)
        Ok(())
    }
}
pub fn should_sync_metadata(f: &str, expire: i128) -> Result<bool> {
    let should = match metadata(f) {
        Ok(m) => {
            let mtime = m.modified()?;
            let now = SystemTime::now();
            let duration = now.duration_since(mtime)?;
            let s = duration.as_secs();
            if s as i128 >= expire {
                true
            } else {
                false
            }
        }
        Err(_) => true,
    };
    // if should {
    //     let file = OpenOptions::new().append(true).create(true).write(true).open(f)?;
    //     file.write_at(buf, offset)
    // }
    // let file = ;
    Ok(should)
}
pub fn recursively_remove_dir(dir: &PathBuf) -> Result<()> {
    if dir.is_dir() {
        match read_dir(dir) {
            Ok(s) => {
                for x in s {
                    let entry = x?;
                    let path = entry.path();
                    if path.is_dir() {
                        recursively_remove_dir(&path)?;
                    } else if path.is_file() {
                        remove_file(path)?;
                    }
                }
            }
            Err(_) => {}
        }
        remove_dir(dir)?;
    }
    Ok(())
}
