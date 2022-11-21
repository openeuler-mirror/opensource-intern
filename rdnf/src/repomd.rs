use anyhow::{bail, Result};
use quick_xml::{events::Event, Reader};
use std::{
    fs::{metadata, File},
    io::Read,
    path::Path,
};

use crate::{
    sub_command::{
        repo::RepoData,
        repoutils::{download_file, HashKind},
    },
    RdnfContext,
};
#[derive(Debug)]
pub struct RepoMd {
    pub primary: Option<RepoMdItem>,
    pub filelists: Option<RepoMdItem>,
    pub updateinfo: Option<RepoMdItem>,
    pub other: Option<RepoMdItem>,
}
#[derive(Debug)]
pub struct RepoMdItem {
    pub checksum: (HashKind, String),
    pub location: String,
    pub size: u64,
}
impl RepoMdItem {
    #[inline]
    pub fn ensure_exists(
        mut self,
        rc: &RdnfContext,
        repo: &RepoData,
        prefix_path: &str,
    ) -> Result<Self> {
        let file_path = prefix_path.to_string() + self.location.as_str();
        if !Path::new(file_path.as_str()).exists() {
            let url = repo.details.base_url.clone().unwrap()+"/" + self.location.as_str();
            let mut flag = false;
            for _i in 1..10 {
                download_file(rc, repo, url.as_str(), file_path.as_str(), &repo.psz_id)?;
                if metadata(file_path.as_str())?.len() == self.size {
                    if self
                        .checksum
                        .0
                        .clone()
                        .checksum(file_path.as_str(), self.checksum.1.as_str())?
                    {
                        flag = true;
                        break;
                    };
                }
            }
            if !flag {
                bail!(
                    "Failed to download file {},or source file corrupted ",
                    url.as_str()
                );
            }
        }
        self.location = file_path;
        Ok(self)
    }
}
impl RepoMd {
    pub fn parse_from(path: &str) -> Result<Self> {
        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;
        let mut reader = Reader::from_str(buf.as_str());
        reader.trim_text(true);
        let mut repomd = RepoMd {
            primary: None,
            filelists: None,
            updateinfo: None,
            other: None,
        };
        loop {
            match reader.read_event() {
                Ok(Event::Start(data)) => match data.name().as_ref() {
                    b"data" => {
                        let mut checksum = (HashKind::Invalid, String::new());
                        let mut location = String::new();
                        let mut size = 0;
                        loop {
                            match reader.read_event()? {
                                Event::Empty(ele) => match ele.name().as_ref() {
                                    b"location" => {
                                        for attr in ele.attributes() {
                                            let attr = attr?;
                                            if String::from_utf8_lossy(attr.key.as_ref()) == "href"
                                            {
                                                location =
                                                    String::from_utf8_lossy(attr.value.as_ref())
                                                        .to_string();
                                            }
                                        }
                                    }
                                    _ => {}
                                },
                                Event::Start(ele) => match ele.name().as_ref() {
                                    b"checksum" => {
                                        for attr in ele.attributes() {
                                            let attr = attr?;
                                            if String::from_utf8_lossy(attr.key.as_ref()) == "type"
                                            {
                                                let kind =
                                                    String::from_utf8_lossy(attr.value.as_ref())
                                                        .to_string();
                                                checksum.0 = HashKind::from(kind.as_str());
                                                checksum.1 =
                                                    reader.read_text(ele.name())?.to_string();
                                            }
                                        }
                                    }
                                    b"size" => {
                                        size = reader.read_text(ele.name())?.parse::<u64>()?;
                                    }
                                    _ => {}
                                },
                                Event::End(ele) => {
                                    if ele.name().as_ref() == b"data" {
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                        let repo_md_item = RepoMdItem {
                            checksum,
                            location,
                            size,
                        };
                        for attr_data in data.attributes() {
                            let attr_data = attr_data?;
                            if String::from_utf8_lossy(attr_data.key.as_ref()) == "type" {
                                match String::from_utf8_lossy(attr_data.value.as_ref()).as_bytes() {
                                    b"primary" => repomd.primary = Some(repo_md_item),
                                    b"filelists" => repomd.filelists = Some(repo_md_item),
                                    b"updateinfo" => repomd.updateinfo = Some(repo_md_item),
                                    b"other" => repomd.other = Some(repo_md_item),
                                    _ => {}
                                }
                                break;
                            }
                        }
                    }
                    _ => {}
                },
                Ok(Event::Eof) => break,
                Err(e) => {
                    bail!(
                        "Failed to parse {} at position {}: {:?}",
                        path,
                        reader.buffer_position(),
                        e
                    )
                }
                _ => {}
            }
        }
        Ok(repomd)
    }
    pub fn ensure_repo_md_parts(
        self,
        rc: &RdnfContext,
        repo: &RepoData,
        cache_dir: String,
    ) -> Result<Self> {
        let mut repo_md = RepoMd {
            primary: None,
            filelists: None,
            updateinfo: None,
            other: None,
        };

        if let Some(primary) = self.primary {
            repo_md.primary = Some(primary.ensure_exists(rc, repo, cache_dir.as_str())?);
        }

        if !repo.base.skip_md_filelists {
            if let Some(file_lists) = self.filelists {
                repo_md.filelists = Some(file_lists.ensure_exists(rc, repo, cache_dir.as_str())?);
            }
        }
        if !repo.base.skip_md_updateinfo {
            if let Some(update_info) = self.updateinfo {
                repo_md.updateinfo =
                    Some(update_info.ensure_exists(rc, repo, cache_dir.as_str())?);
            }
        }
        if !repo.base.skip_md_other {
            if let Some(other) = self.other {
                repo_md.other = Some(other.ensure_exists(rc, repo, cache_dir.as_str())?);
            }
        }
        Ok(repo_md)
    }
}
